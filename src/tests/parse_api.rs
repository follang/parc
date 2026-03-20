use crate::driver::Flavor;
use crate::parse;

#[test]
fn parse_api_constant_works() {
    let constant = parse::constant("42", Flavor::StdC11).expect("parsing constant");
    match constant {
        crate::ast::Constant::Integer(_) => {}
        other => panic!("expected integer constant, got {:?}", other),
    }
}

#[test]
fn parse_api_expression_works() {
    parse::expression("value + 1", Flavor::StdC11).expect("parsing expression");
}

#[test]
fn parse_api_declaration_works() {
    parse::declaration("int value;", Flavor::StdC11).expect("parsing declaration");
}

#[test]
fn parse_api_statement_works() {
    parse::statement("return 0;", Flavor::StdC11).expect("parsing statement");
}

#[test]
fn parse_api_translation_unit_works() {
    parse::translation_unit("typedef int counter_t;\nint main(void) { return 0; }\n", Flavor::StdC11)
        .expect("parsing translation unit");
}

#[test]
fn parse_api_resilient_recovers_from_invalid_declaration() {
    let input = "int before;\n@@@invalid@@@;\nint after;\n";
    // Strict parse should fail
    assert!(parse::translation_unit(input, Flavor::StdC11).is_err());
    // Resilient parse should recover
    let tu = parse::translation_unit_resilient(input, Flavor::StdC11);
    assert_eq!(tu.0.len(), 2, "expected 2 declarations, got {}", tu.0.len());
}

#[test]
fn parse_api_resilient_returns_all_when_valid() {
    let input = "int x;\nint y;\nint z;\n";
    let tu = parse::translation_unit_resilient(input, Flavor::StdC11);
    assert_eq!(tu.0.len(), 3, "expected 3 declarations, got {}", tu.0.len());
}
