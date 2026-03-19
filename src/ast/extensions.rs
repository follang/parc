use crate::ast::declarations::{TypeName, TypeQualifier};
use crate::ast::expressions::Expression;
use crate::ast::lexical::{Identifier, StringLiteral};
use crate::span::Node;

// Syntax extensions

/// Extended vendor-specific syntax that does not fit elsewhere
#[derive(Debug, PartialEq, Clone)]
pub enum Extension {
    Attribute(Attribute),
    AsmLabel(Node<StringLiteral>),
    AvailabilityAttribute(Node<AvailabilityAttribute>),
}

/// Attributes
#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    pub name: Node<String>,
    pub arguments: Vec<Node<Expression>>,
}

/// Platform availability attribute
#[derive(Debug, PartialEq, Clone)]
pub struct AvailabilityAttribute {
    pub platform: Node<Identifier>,
    pub clauses: Vec<Node<AvailabilityClause>>,
}

/// Platfrom availability attribute clause
#[derive(Debug, PartialEq, Clone)]
pub enum AvailabilityClause {
    Introduced(Node<AvailabilityVersion>),
    Deprecated(Node<AvailabilityVersion>),
    Obsoleted(Node<AvailabilityVersion>),
    Unavailable,
    Message(Node<StringLiteral>),
    Replacement(Node<StringLiteral>),
}

/// Platfrom version inside availability attribute
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AvailabilityVersion {
    pub major: String,
    pub minor: Option<String>,
    pub subminor: Option<String>,
}

/// Inline assembler
#[derive(Debug, PartialEq, Clone)]
pub enum AsmStatement {
    GnuBasic(Node<StringLiteral>),
    GnuExtended(GnuExtendedAsmStatement),
}

/// Extended statement that has access to C variables
#[derive(Debug, PartialEq, Clone)]
pub struct GnuExtendedAsmStatement {
    pub qualifier: Option<Node<TypeQualifier>>,
    pub template: Node<StringLiteral>,
    pub outputs: Vec<Node<GnuAsmOperand>>,
    pub inputs: Vec<Node<GnuAsmOperand>>,
    pub clobbers: Vec<Node<StringLiteral>>,
}

/// Single input or output operand specifier for GNU extended asm statement
#[derive(Debug, PartialEq, Clone)]
pub struct GnuAsmOperand {
    pub symbolic_name: Option<Node<Identifier>>,
    pub constraints: Node<StringLiteral>,
    pub variable_name: Node<Expression>,
}

/// Type of an expression or type
#[derive(Debug, PartialEq, Clone)]
pub enum TypeOf {
    Expression(Node<Expression>),
    Type(Node<TypeName>),
}
