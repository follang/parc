# PAC Reference

PAC is a Rust library for C language frontend processing: preprocessing,
parsing, and source-level semantic extraction. It targets C11 and can
optionally accept GNU and Clang extensions.

PAC is the frontend stage of the PARC pipeline. It produces a durable
`SourcePackage` contract suitable for downstream consumption by linker
and codegen stages.

## What PAC gives you

- **Preprocessing**: built-in C preprocessor with macro expansion, conditionals, and includes
- **Parsing**: C11 parser producing a typed AST under `pac::ast`
- **Extraction**: source-level declaration normalization into `pac::ir`
- **Source IR**: a serializable `SourcePackage` with functions, records, enums, typedefs, variables, macros, diagnostics, and provenance
- **Scanning**: end-to-end header scanning via `pac::scan`
- Source spans under `pac::span`
- File/line reconstruction under `pac::loc`
- Recursive visitor API under `pac::visit`
- Tree-style debug printer under `pac::print`

## Pipeline

```text
C source / headers
  -> preprocessor (built-in or gcc/clang)
  -> PAC parser
  -> extraction
  -> SourcePackage (frontend contract)
  -> downstream (LINC, GERC)
```

## Public modules

| Module | Purpose |
| --- | --- |
| `ir` | Source-level IR: `SourcePackage`, `SourceType`, `SourceItem`, etc. |
| `extract` | Declaration extraction from AST to IR |
| `scan` | Header scanning (preprocess + parse + extract) |
| `intake` | Preprocessed source intake |
| `driver` | High-level API for parsing files via a C preprocessor |
| `preprocess` | Built-in C preprocessor |
| `parse` | Direct parsing of expressions, declarations, statements, translation units |
| `ast` | AST definitions for declarations, expressions, statements, extensions |
| `visit` | Recursive traversal API for AST consumers |
| `span` | Byte offsets for parsed nodes |
| `loc` | Mapping byte offsets back to source files and lines |
| `print` | Debug-oriented tree printer for ASTs |

## Typical use cases

- Extract function/struct/enum declarations from C headers
- Build a linter or analysis tool for C code
- Scan headers and produce a serializable source package
- Parse small fragments in tests
- Prototype refactoring or code-search tools

## Where to start

- Read [Getting Started](./010_getting_started.md) for basic setup
- Read [Source IR](./026_source_ir.md) for the data contract
- Read [Extraction](./027_extraction.md) for turning source into IR
- Read [Header Scanning](./028_scanning.md) for end-to-end workflows
- Read [Driver API](./020_driver.md) or [Parser API](./030_parser.md) for parse-level access
- Read [AST Model](./040_ast.md) before writing analysis code
