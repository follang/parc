# Getting Started

This chapter is the shortest path from raw C source or headers to a PARC-owned
source artifact.

Read `parc` as the source frontend of the toolchain:

- `parc` owns preprocessing, parsing, extraction, and source diagnostics
- `linc` owns link and binary evidence
- `gerc` owns Rust lowering and emitted build output

The boundary rule is strict: `parc/src/**` must not depend on `linc` or `gerc`,
and any cross-package translation belongs only in tests, examples, or external
harnesses.

## Add the crate

```toml
[dependencies]
parc = { path = "../pac" }
```

## Pick the right API first

Use `parc::driver` when you have a file on disk and want PARC to run a system preprocessor first.

```rust
use parc::driver::{parse, Config};

fn main() -> Result<(), parc::driver::Error> {
    let config = Config::default();
    let parsed = parse(&config, "examples/sample.c")?;

    println!("preprocessed bytes: {}", parsed.source.len());
    println!("top-level items: {}", parsed.unit.0.len());
    Ok(())
}
```

Use `parc::parse` when you already have source text in memory and want to parse a fragment directly.

```rust
use parc::driver::Flavor;
use parc::parse;

fn main() {
    let expr = parse::expression("a + b * 2", Flavor::StdC11).unwrap();
    println!("{:#?}", expr);
}
```

## Choose a language flavor

PARC supports three parser modes:

| Flavor | Meaning |
| --- | --- |
| `StdC11` | Strict C11 |
| `GnuC11` | C11 plus GNU syntax such as `typeof`, attributes, statement expressions, and GNU asm |
| `ClangC11` | C11 plus Clang-oriented extensions such as availability attributes |

For file-based parsing, `Config::default()` selects:

- `clang -E` on macOS
- `gcc -E` on other targets

You can also select explicitly:

```rust
use parc::driver::Config;

let gnu = Config::with_gcc();
let clang = Config::with_clang();
```

## First useful example

This example parses a translation unit and counts top-level entries:

```rust
use parc::driver::{parse, Config};

fn main() -> Result<(), parc::driver::Error> {
    let parsed = parse(&Config::default(), "examples/header.h")?;

    for (i, item) in parsed.unit.0.iter().enumerate() {
        println!("item #{i}: {:?}", item.node);
    }

    Ok(())
}
```

## First fragment example

If you only need one declaration or statement, the direct parser API is faster to wire in:

```rust
use parc::driver::Flavor;
use parc::parse;

fn main() {
    let decl = parse::declaration("static const int answer = 42;", Flavor::StdC11).unwrap();
    let stmt = parse::statement("return answer;", Flavor::StdC11).unwrap();

    println!("{:#?}", decl);
    println!("{:#?}", stmt);
}
```

## What to read next

- [Common Workflows](./015_workflows.md) for choosing between `driver`, `parse_preprocessed`, and `parse`
- [Driver API](./020_driver.md) for preprocessing and file-based parsing
- [Parser API](./030_parser.md) for fragment parsing

## Architectural boundary

`parc` is the source frontend.

It owns:

- preprocessing
- parsing
- source extraction
- source diagnostics
- the `parc::ir::SourcePackage` artifact

It does not own:

- symbol inventory
- binary validation
- link planning
- Rust code generation

In this repository, cross-package composition should not live in `parc` library
code. `linc` and `gerc` should consume `parc` output only from tests, examples,
or external harnesses.
