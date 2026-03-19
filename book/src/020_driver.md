# Driver API

The `driver` module provides the high-level interface: preprocess a C file and parse it in one step.

## Config

```rust
pub struct Config {
    pub cpp_command: String,      // e.g. "gcc"
    pub cpp_options: Vec<String>, // e.g. ["-E", "-I/usr/include"]
    pub flavor: Flavor,           // GnuC11, ClangC11, StdC11
}
```

## Parsing

```rust
use pac::driver::{Config, parse};

let config = Config {
    cpp_command: "gcc".into(),
    cpp_options: vec!["-E".into(), "-I/usr/local/include".into()],
    flavor: pac::driver::Flavor::GnuC11,
};

let result = parse(&config, "header.h").unwrap();
```

The result contains:
- `unit`: the parsed `TranslationUnit` (the AST root)
- `source`: the preprocessed source text

## Errors

`parse` returns `Error` with variants:
- `PreprocessorError` — the C preprocessor failed
- `SyntaxError` — the parser could not process the preprocessed source
