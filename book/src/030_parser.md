# Parser API

The `parse` module provides direct access to the parser without running the preprocessor.

## Usage

```rust
use pac::parse::translation_unit;
use pac::driver::Flavor;

let source = "int add(int a, int b);";
let result = translation_unit(source, Flavor::GnuC11);
```

This is useful when you have already-preprocessed source or want to parse C fragments directly.

## Flavors

| Flavor | Description |
|--------|-------------|
| `StdC11` | Strict C11 standard |
| `GnuC11` | C11 with GNU extensions (`__attribute__`, `typeof`, `asm`, etc.) |
| `ClangC11` | C11 with Clang extensions (`__block`, availability attributes, etc.) |
