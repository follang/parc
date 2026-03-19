# Lang-C

[![Documentation](https://docs.rs/lang-c/badge.svg)](https://docs.rs/lang-c)

Ligtweight parser of C language for Rust users. Almost full support for C11 revision of the language.
Several GCC and Clang extensions are also supported as an option.

```rust
extern crate lang_c;
use lang_c::driver::{Config, parse}; 

fn main() {
    let config = Config::default();
    println!("{:?}", parse(&config, "example.c"));
}
```

# Bugs

Just open an issue, bug reports and patches are most welcome. 

## License

Dual-licenced under Apache 2.0 or MIT licenses (see `LICENSE-APACHE` and `LICENSE-MIT` for legal terms).

## Development

The parser implementation lives in `src/parser.rs` and is maintained directly in Rust.
There is no external PEG generation step in the build anymore.

The reference fixtures live under `test/reftests/`.

The makefile mirrors the normal Cargo flow:

- `make` runs the full test suite
- `make lib` builds the library
- `make test` runs all tests
- `make check` aliases to `make test`
