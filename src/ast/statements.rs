use crate::ast::declarations::{Declaration, StaticAssert};
use crate::ast::expressions::Expression;
use crate::ast::extensions::AsmStatement;
use crate::ast::lexical::Identifier;
use crate::span::Node;

// From 6.8 Statement

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Labeled(Node<LabeledStatement>),
    Compound(Vec<Node<BlockItem>>),
    Expression(Option<Box<Node<Expression>>>),
    If(Node<IfStatement>),
    Switch(Node<SwitchStatement>),
    While(Node<WhileStatement>),
    DoWhile(Node<DoWhileStatement>),
    For(Node<ForStatement>),
    Goto(Node<Identifier>),
    Continue,
    Break,
    Return(Option<Box<Node<Expression>>>),
    Asm(Node<AsmStatement>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LabeledStatement {
    pub label: Node<Label>,
    pub statement: Box<Node<Statement>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub condition: Box<Node<Expression>>,
    pub then_statement: Box<Node<Statement>>,
    pub else_statement: Option<Box<Node<Statement>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchStatement {
    pub expression: Box<Node<Expression>>,
    pub statement: Box<Node<Statement>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement {
    pub expression: Box<Node<Expression>>,
    pub statement: Box<Node<Statement>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DoWhileStatement {
    pub statement: Box<Node<Statement>>,
    pub expression: Box<Node<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement {
    pub initializer: Node<ForInitializer>,
    pub condition: Option<Box<Node<Expression>>>,
    pub step: Option<Box<Node<Expression>>>,
    pub statement: Box<Node<Statement>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Label {
    Identifier(Node<Identifier>),
    Case(Box<Node<Expression>>),
    CaseRange(Node<CaseRange>),
    Default,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaseRange {
    pub low: Box<Node<Expression>>,
    pub high: Box<Node<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ForInitializer {
    Empty,
    Expression(Box<Node<Expression>>),
    Declaration(Node<Declaration>),
    StaticAssert(Node<StaticAssert>),
}

// From 6.8.2

#[derive(Debug, PartialEq, Clone)]
pub enum BlockItem {
    Declaration(Node<Declaration>),
    StaticAssert(Node<StaticAssert>),
    Statement(Node<Statement>),
}
