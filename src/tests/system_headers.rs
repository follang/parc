use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::driver::{self, Config};

fn known_system_headers() -> Vec<&'static str> {
    vec!["stdint.h", "stdio.h", "linux/stddef.h"]
}

fn unique_temp_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    env::temp_dir().join(format!("lang-c-system-headers-{}", stamp))
}

fn write_wrapper(dir: &Path, header: &str) -> PathBuf {
    let wrapper = dir.join("wrapper.c");
    let source = format!("#include <{}>\nint lang_c_header_probe(void) {{ return 0; }}\n", header);
    fs::write(&wrapper, source).expect("writing temporary wrapper");
    wrapper
}

fn parse_header_wrapper(path: &Path) -> Result<(), String> {
    let mut config = Config::with_gcc();
    config.cpp_options.push("-D_GNU_SOURCE".to_owned());
    driver::parse(&config, path)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

#[test]
fn system_header_wrappers_parse_when_headers_exist() {
    let mut attempted = 0usize;
    let mut failures = Vec::new();

    for header in known_system_headers() {
        let full_path = Path::new("/usr/include").join(header);
        if !full_path.exists() {
            continue;
        }

        attempted += 1;
        let dir = unique_temp_dir();
        fs::create_dir_all(&dir).expect("creating temporary wrapper directory");
        let wrapper = write_wrapper(&dir, header);

        if let Err(err) = parse_header_wrapper(&wrapper) {
            failures.push(format!("{}: {}", header, err));
        }

        fs::remove_file(&wrapper).expect("removing temporary wrapper");
        fs::remove_dir(&dir).expect("removing temporary wrapper directory");
    }

    assert!(attempted > 0, "expected at least one system header to exist");
    if !failures.is_empty() {
        panic!(
            "{} system header wrappers failed:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }
}
