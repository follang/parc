//! Scan configuration for the PARC frontend.
//!
//! Only preprocessing and entry-header concerns live here.
//! Link/binary/ABI configuration belongs to downstream crates.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::driver::Flavor;

/// Configuration for scanning C headers into a `SourcePackage`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    /// Entry-point headers that define the API surface.
    pub entry_headers: Vec<PathBuf>,
    /// Preprocessor include search paths (local/user headers).
    pub include_dirs: Vec<PathBuf>,
    /// System include search paths (for `<...>` includes like `/usr/include`).
    #[serde(default)]
    pub system_include_dirs: Vec<PathBuf>,
    /// Preprocessor defines.
    pub defines: Vec<(String, Option<String>)>,
    /// Compiler command for external preprocessing.
    pub compiler: Option<String>,
    /// Parser flavor.
    pub flavor: Flavor,
    /// Whether to use the built-in preprocessor instead of an external compiler.
    #[serde(default)]
    pub use_builtin_preprocessor: bool,
    /// Whether to resolve all typedef chains after extraction.
    #[serde(default)]
    pub resolve_typedefs: bool,
}

impl ScanConfig {
    pub fn new() -> Self {
        Self {
            entry_headers: Vec::new(),
            include_dirs: Vec::new(),
            system_include_dirs: Vec::new(),
            defines: Vec::new(),
            compiler: None,
            flavor: Flavor::GnuC11,
            use_builtin_preprocessor: false,
            resolve_typedefs: false,
        }
    }

    pub fn entry_header(mut self, path: impl Into<PathBuf>) -> Self {
        self.entry_headers.push(path.into());
        self
    }

    pub fn include_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.include_dirs.push(path.into());
        self
    }

    /// Add a system include search path (for `<...>` includes).
    pub fn system_include_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.system_include_dirs.push(path.into());
        self
    }

    pub fn define(mut self, name: impl Into<String>, value: Option<String>) -> Self {
        self.defines.push((name.into(), value));
        self
    }

    pub fn define_flag(mut self, name: impl Into<String>) -> Self {
        self.defines.push((name.into(), None));
        self
    }

    pub fn with_compiler(mut self, compiler: impl Into<String>) -> Self {
        self.compiler = Some(compiler.into());
        self
    }

    pub fn with_flavor(mut self, flavor: Flavor) -> Self {
        self.flavor = flavor;
        self
    }

    pub fn with_builtin_preprocessor(mut self) -> Self {
        self.use_builtin_preprocessor = true;
        self
    }

    /// Enable transitive typedef resolution after extraction.
    pub fn with_resolve_typedefs(mut self) -> Self {
        self.resolve_typedefs = true;
        self
    }
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self::new()
    }
}
