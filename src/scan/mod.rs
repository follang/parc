//! Header scanning frontend for the PARC pipeline.
//!
//! This module provides `ScanConfig` and `scan_headers()` for turning
//! C headers into a `SourcePackage` — either via external preprocessing
//! (gcc/clang -E) or the built-in preprocessor.

pub mod config;

pub use config::ScanConfig;

use std::io;
use std::process::Command;

use crate::extract::Extractor;
use crate::ir::{DiagnosticKind, SourceDiagnostic, SourceDefine, SourceInputs, SourcePackage, SourceTarget};

/// Result of a header scan operation.
#[derive(Debug)]
pub struct ScanResult {
    pub package: SourcePackage,
    pub preprocessed_source: String,
}

/// Scan headers according to the given config, producing a `SourcePackage`.
pub fn scan_headers(config: &ScanConfig) -> Result<ScanResult, ScanError> {
    if config.entry_headers.is_empty() {
        return Err(ScanError::NoEntryHeaders);
    }

    let (preprocessed, compiler_cmd) = if config.use_builtin_preprocessor {
        preprocess_builtin(config)?
    } else {
        preprocess_external(config)?
    };

    let unit = match crate::parse::translation_unit(&preprocessed, config.flavor) {
        Ok(unit) => unit,
        Err(e) => {
            let mut pkg = make_base_package(config, &compiler_cmd);
            pkg.diagnostics.push(SourceDiagnostic::error(
                DiagnosticKind::ParseFailed,
                format!(
                    "parse error at line {}:{}: {:?}",
                    e.line, e.column, e.expected
                ),
            ));
            return Ok(ScanResult {
                package: pkg,
                preprocessed_source: preprocessed,
            });
        }
    };

    let extractor = Extractor::new();
    let (items, diagnostics) = extractor.extract(&unit);

    let mut pkg = make_base_package(config, &compiler_cmd);
    pkg.items = items;
    pkg.diagnostics = diagnostics;

    Ok(ScanResult {
        package: pkg,
        preprocessed_source: preprocessed,
    })
}

fn make_base_package(config: &ScanConfig, compiler_cmd: &str) -> SourcePackage {
    SourcePackage {
        target: SourceTarget {
            compiler_command: Some(compiler_cmd.to_string()),
            flavor: Some(format!("{:?}", config.flavor)),
            ..Default::default()
        },
        inputs: SourceInputs {
            entry_headers: config
                .entry_headers
                .iter()
                .map(|p| p.display().to_string())
                .collect(),
            include_dirs: config
                .include_dirs
                .iter()
                .map(|p| p.display().to_string())
                .collect(),
            defines: config
                .defines
                .iter()
                .map(|(k, v)| SourceDefine {
                    name: k.clone(),
                    value: v.clone(),
                })
                .collect(),
        },
        ..SourcePackage::new()
    }
}

fn preprocess_external(config: &ScanConfig) -> Result<(String, String), ScanError> {
    let compiler = config
        .compiler
        .as_deref()
        .unwrap_or(match config.flavor {
            crate::driver::Flavor::ClangC11 => "clang",
            _ => "gcc",
        });

    let mut cmd = Command::new(compiler);
    cmd.arg("-E");

    for dir in &config.include_dirs {
        cmd.arg("-I").arg(dir);
    }
    for (name, value) in &config.defines {
        match value {
            Some(v) => cmd.arg(format!("-D{}={}", name, v)),
            None => cmd.arg(format!("-D{}", name)),
        };
    }
    for header in &config.entry_headers {
        cmd.arg(header);
    }

    let output = cmd.output().map_err(|e| ScanError::PreprocessorFailed(e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(ScanError::PreprocessorError(stderr));
    }

    let source = String::from_utf8_lossy(&output.stdout).to_string();
    Ok((source, compiler.to_string()))
}

fn preprocess_builtin(config: &ScanConfig) -> Result<(String, String), ScanError> {
    use crate::preprocess::{IncludeResolver, Processor, MacroDef, Token, TokenKind};

    let mut resolver = IncludeResolver::new();
    let mut processor = Processor::new();

    // Add include dirs
    for dir in &config.include_dirs {
        resolver.add_local_path(dir);
    }

    // Add defines
    for (name, value) in &config.defines {
        let body_text = value.as_deref().unwrap_or("1");
        let body_tokens = vec![Token {
            kind: TokenKind::Ident,
            text: body_text.to_string(),
            offset: 0,
        }];
        processor.macros_mut().define(MacroDef {
            name: name.clone(),
            params: None,
            is_variadic: false,
            body: body_tokens,
        });
    }

    // Process each entry header
    let mut all_text = String::new();
    let mut all_errors = Vec::new();

    for header in &config.entry_headers {
        let result = resolver.preprocess_file(header.as_ref(), &mut processor);
        all_text.push_str(&result.text);
        all_text.push('\n');
        all_errors.extend(result.errors);
    }

    if !all_errors.is_empty() {
        return Err(ScanError::PreprocessorError(all_errors.join("\n")));
    }

    Ok((all_text, "builtin".to_string()))
}

/// Errors from the scan operation.
#[derive(Debug)]
pub enum ScanError {
    NoEntryHeaders,
    PreprocessorFailed(io::Error),
    PreprocessorError(String),
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::NoEntryHeaders => write!(f, "no entry headers specified"),
            ScanError::PreprocessorFailed(e) => write!(f, "preprocessor failed: {}", e),
            ScanError::PreprocessorError(s) => write!(f, "preprocessor error: {}", s),
        }
    }
}

impl std::error::Error for ScanError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_no_headers_error() {
        let config = ScanConfig::new();
        let result = scan_headers(&config);
        assert!(result.is_err());
    }

    #[test]
    fn scan_config_builder() {
        let config = ScanConfig::new()
            .entry_header("test.h")
            .include_dir("/usr/include")
            .define("VERSION", Some("2".into()))
            .define_flag("DEBUG")
            .with_flavor(crate::driver::Flavor::StdC11);

        assert_eq!(config.entry_headers.len(), 1);
        assert_eq!(config.include_dirs.len(), 1);
        assert_eq!(config.defines.len(), 2);
        assert_eq!(config.flavor, crate::driver::Flavor::StdC11);
    }

    #[test]
    fn scan_builtin_with_temp_file() {
        let dir = std::env::temp_dir().join("pac_test_scan");
        let _ = std::fs::create_dir_all(&dir);
        std::fs::write(
            dir.join("api.h"),
            "typedef int int32_t;\nint32_t get_value(void);\n",
        )
        .unwrap();

        let config = ScanConfig::new()
            .entry_header(dir.join("api.h"))
            .with_builtin_preprocessor();

        let result = scan_headers(&config).expect("scan should succeed");
        assert!(result.package.functions().count() >= 1);
        assert!(result.package.type_aliases().count() >= 1);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn scan_builtin_with_include() {
        let dir = std::env::temp_dir().join("pac_test_scan_inc");
        let _ = std::fs::create_dir_all(&dir);
        std::fs::write(dir.join("types.h"), "typedef unsigned int uint32_t;\n").unwrap();
        std::fs::write(
            dir.join("api.h"),
            "#include \"types.h\"\nuint32_t get_id(void);\n",
        )
        .unwrap();

        let config = ScanConfig::new()
            .entry_header(dir.join("api.h"))
            .with_builtin_preprocessor();

        let result = scan_headers(&config).expect("scan should succeed");
        assert!(result.package.type_aliases().count() >= 1);
        assert!(result.package.functions().count() >= 1);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn scan_config_default() {
        let config = ScanConfig::default();
        assert!(config.entry_headers.is_empty());
        assert!(!config.use_builtin_preprocessor);
    }

    #[test]
    fn scan_populates_inputs_metadata() {
        let dir = std::env::temp_dir().join("pac_test_scan_meta");
        let _ = std::fs::create_dir_all(&dir);
        std::fs::write(dir.join("a.h"), "int x;\n").unwrap();

        let config = ScanConfig::new()
            .entry_header(dir.join("a.h"))
            .include_dir("/opt/include")
            .define("VER", Some("3".into()))
            .with_builtin_preprocessor();

        let result = scan_headers(&config).unwrap();
        assert_eq!(result.package.inputs.include_dirs.len(), 1);
        assert_eq!(result.package.inputs.defines.len(), 1);
        assert_eq!(result.package.inputs.defines[0].name, "VER");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn scan_config_json_roundtrip() {
        let config = ScanConfig::new()
            .entry_header("api.h")
            .define_flag("NDEBUG");
        let json = serde_json::to_string(&config).unwrap();
        let back: ScanConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.entry_headers.len(), 1);
        assert_eq!(back.defines.len(), 1);
    }
}
