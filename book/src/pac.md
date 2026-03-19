# PAC Reference

PAC (parser c) is a lightweight C parser for Rust. It supports the C11 standard with GNU and Clang extensions.

## Pipeline

```
C source → preprocessor (gcc/clang) → PAC parser → AST
```

## Modules

| Module | Description |
|--------|-------------|
| `driver` | High-level API: preprocess and parse in one step |
| `parse` | Direct parser: parse already-preprocessed source |
| `ast` | Abstract syntax tree types |
| `visit` | Visitor trait for AST traversal |
| `loc` | Source location tracking |
| `span` | Span types for AST nodes |
| `print` | AST pretty-printing |
