# Getting Started

Add PAC as a dependency:

```toml
[dependencies]
pac = { path = "../pac" }
```

## Quick example

```rust
use pac::driver::{Config, parse};

let config = Config::default();
let result = parse(&config, "example.c").unwrap();
println!("{:#?}", result.unit);
```

## Choosing a compiler flavor

```rust
use pac::driver::Config;

// GNU extensions (default on Linux)
let config = Config::with_gcc();

// Clang extensions (default on macOS)
let config = Config::with_clang();
```
