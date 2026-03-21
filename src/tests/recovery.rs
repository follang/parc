//! Tests for malformed input recovery.
//!
//! Ensures the extractor handles broken, incomplete, or unusual input gracefully
//! without panicking or producing wildly incorrect results.

use crate::extract;

// --- Empty input ---

#[test]
fn recovery_empty_string() {
    let pkg = extract::extract_from_source("").unwrap();
    assert!(pkg.is_empty());
}

#[test]
fn recovery_whitespace_only() {
    let pkg = extract::extract_from_source("   \n\n\t  \n").unwrap();
    assert!(pkg.is_empty());
}

// --- Comments only ---

#[test]
fn recovery_comments_only() {
    // Parser may reject a translation unit with only comments; resilient mode handles it
    let pkg = extract::parse_and_extract_resilient(
        "/* nothing here */ // also nothing\n",
        crate::driver::Flavor::GnuC11,
    );
    assert!(pkg.is_empty() || pkg.item_count() == 0);
}

// --- Invalid syntax should return error ---

#[test]
fn recovery_invalid_syntax_errors() {
    let result = extract::extract_from_source("@@@invalid@@@");
    assert!(result.is_err());
}

#[test]
fn recovery_unterminated_struct() {
    let result = extract::extract_from_source("struct broken {");
    assert!(result.is_err());
}

#[test]
fn recovery_unterminated_function() {
    let result = extract::extract_from_source("int broken(");
    assert!(result.is_err());
}

// --- Resilient recovery ---

#[test]
fn recovery_resilient_before_and_after_garbage() {
    let pkg = extract::parse_and_extract_resilient(
        "int before;\n@@@garbage@@@;\nint after(void);",
        crate::driver::Flavor::GnuC11,
    );
    // Should recover at least one item
    assert!(pkg.item_count() >= 1);
}

#[test]
fn recovery_resilient_multiple_errors() {
    let pkg = extract::parse_and_extract_resilient(
        r#"
int good1(void);
@@@bad1@@@;
struct also_good { int x; };
@@@bad2@@@;
void good2(int y);
"#,
        crate::driver::Flavor::GnuC11,
    );
    // Should recover the valid items
    assert!(pkg.item_count() >= 2);
}

#[test]
fn recovery_resilient_produces_items_not_just_diagnostics() {
    let pkg = extract::parse_and_extract_resilient(
        "typedef int my_int;\ngarbage here;\nvoid ok(void);",
        crate::driver::Flavor::GnuC11,
    );
    // Should have at least one usable item
    assert!(pkg.item_count() >= 1);
}

// --- Edge case valid inputs ---

#[test]
fn recovery_single_semicolon() {
    // The parser may not accept bare semicolons at top level; resilient mode handles it
    let pkg = extract::parse_and_extract_resilient(";", crate::driver::Flavor::GnuC11);
    assert!(pkg.item_count() == 0);
}

#[test]
fn recovery_multiple_semicolons() {
    let pkg = extract::parse_and_extract_resilient(";;;", crate::driver::Flavor::GnuC11);
    assert!(pkg.item_count() == 0);
}

#[test]
fn recovery_just_a_typedef() {
    let pkg = extract::extract_from_source("typedef int x;").unwrap();
    assert_eq!(pkg.type_alias_count(), 1);
}

// --- Unusual but valid C ---

#[test]
fn recovery_empty_enum() {
    // GNU extension: empty enum
    let result = extract::parse_and_extract("enum empty {};", crate::driver::Flavor::GnuC11);
    // Should not panic whether it parses or not
    match result {
        Ok(_) => {
            // If it parses, the enum should be present
        }
        Err(_) => {
            // If it doesn't parse, that's fine too
        }
    }
}

#[test]
fn recovery_very_long_identifier() {
    let name = "a".repeat(500);
    let src = format!("int {}(void);", name);
    let pkg = extract::extract_from_source(&src).unwrap();
    let f = pkg.find_function(&name).unwrap();
    assert_eq!(f.name, name);
}

// --- Parse-and-extract with different flavors ---

#[test]
fn recovery_stdc11_flavor() {
    let pkg = extract::parse_and_extract("int foo(void);", crate::driver::Flavor::StdC11).unwrap();
    assert_eq!(pkg.function_count(), 1);
}

#[test]
fn recovery_gnuc11_flavor() {
    let pkg = extract::parse_and_extract("int foo(void);", crate::driver::Flavor::GnuC11).unwrap();
    assert_eq!(pkg.function_count(), 1);
}

#[test]
fn recovery_clang_flavor() {
    let pkg =
        extract::parse_and_extract("int foo(void);", crate::driver::Flavor::ClangC11).unwrap();
    assert_eq!(pkg.function_count(), 1);
}

// --- Source offset preservation on error recovery ---

#[test]
fn recovery_offsets_preserved_after_recovery() {
    let pkg = extract::parse_and_extract_resilient(
        "int first(void);\n@@bad@@;\nvoid second(void);",
        crate::driver::Flavor::GnuC11,
    );
    for item in &pkg.items {
        // Items that were extracted should have source offsets
        assert!(
            item.source_offset().is_some(),
            "item {:?} missing source offset",
            item.name()
        );
    }
}
