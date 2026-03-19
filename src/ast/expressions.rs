use crate::ast::declarations::{InitializerListItem, TypeName};
use crate::ast::lexical::{Constant, Identifier, StringLiteral};
use crate::ast::statements::Statement;
use crate::span::Node;

// From 6.5 Expressions

/// Expressions
///
/// (C11 6.5)
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// Identifier
    ///
    /// May be a variable, function name or enumerator. The latter is
    /// different from the standard, where enumerators are classified
    /// as constants.
    ///
    /// (C11 6.5.1)
    Identifier(Box<Node<Identifier>>),
    /// Numeric and character constants
    ///
    /// Enumerator constants, being valid identifiers, are reprented
    /// as `Identifier` in this enum.
    ///
    /// (C11 6.5.1)
    Constant(Box<Node<Constant>>),
    /// String literal
    ///
    /// (C11 6.5.1)
    StringLiteral(Box<Node<StringLiteral>>),
    /// Generic selection
    ///
    /// (C11 6.5.1.1)
    GenericSelection(Box<Node<GenericSelection>>),
    /// Structure and union members
    ///
    /// Both direct (`.`) and indirect (`->`) access.
    ///
    /// (C11 6.5.2)
    Member(Box<Node<MemberExpression>>),
    /// Function call expression
    ///
    /// (C11 6.5.2)
    Call(Box<Node<CallExpression>>),
    /// Compound literal
    ///
    /// (C11 6.5.2)
    CompoundLiteral(Box<Node<CompoundLiteral>>),
    /// Size of a type
    ///
    /// (C11 6.5.3)
    SizeOfTy(Box<Node<SizeOfTy>>),
    /// Size of a unary expression
    ///
    /// (C11 6.5.3)
    SizeOfVal(Box<Node<SizeOfVal>>),
    /// Alignment of a type
    ///
    /// (C11 6.5.3)
    AlignOf(Box<Node<AlignOf>>),
    /// Unary operators
    ///
    /// This represents both postfix and prefix unary oprators. Postfix expressions that take
    /// additional operands are represented by a separate entry in this enum.
    ///
    /// (C11 6.5.2, c11 6.5.3)
    UnaryOperator(Box<Node<UnaryOperatorExpression>>),
    /// Cast expression
    ///
    /// `(type) expr`
    ///
    /// (C11 6.5.4)
    Cast(Box<Node<CastExpression>>),
    /// Binary operators
    ///
    /// All of C binary operators that can be applied to two expressions.
    ///
    /// (C11 6.5.5 -- 6.5.16)
    BinaryOperator(Box<Node<BinaryOperatorExpression>>),
    /// Conditional operator
    ///
    /// (C11 6.5.15)
    Conditional(Box<Node<ConditionalExpression>>),
    /// Comma operator
    ///
    /// (C11 6.5.17)
    Comma(Box<Vec<Node<Expression>>>),
    /// Member offset expression
    ///
    /// Result of expansion of `offsetof` macro.
    ///
    /// (C11 7.19 §3).
    OffsetOf(Box<Node<OffsetOfExpression>>),
    /// Variable argument list access
    ///
    /// Result of expansion of `va_arg` macro.
    ///
    /// (C11 7.16.1.1).
    VaArg(Box<Node<VaArgExpression>>),
    /// Statement expression
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Statement-Exprs.html)
    Statement(Box<Node<Statement>>),
}

/// Struct or union member access
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MemberOperator {
    /// `expression.identifier`
    Direct,
    /// `expression->identifier`
    Indirect,
}

/// Generic selection expression
///
/// (C11 6.5.1.1)
#[derive(Debug, PartialEq, Clone)]
pub struct GenericSelection {
    pub expression: Box<Node<Expression>>,
    pub associations: Vec<Node<GenericAssociation>>,
}

/// Single element of a generic selection expression
///
/// (C11 6.5.1.1)
#[derive(Debug, PartialEq, Clone)]
pub enum GenericAssociation {
    Type(Node<GenericAssociationType>),
    Default(Box<Node<Expression>>),
}

/// Type match case in a generic selection expression
///
/// (C11 6.5.1.1)
#[derive(Debug, PartialEq, Clone)]
pub struct GenericAssociationType {
    pub type_name: Node<TypeName>,
    pub expression: Box<Node<Expression>>,
}

/// Structure and union members
///
/// Both direct (`.`) and indirect (`->`) access.
///
/// (C11 6.5.2)
#[derive(Debug, PartialEq, Clone)]
pub struct MemberExpression {
    pub operator: Node<MemberOperator>,
    pub expression: Box<Node<Expression>>,
    pub identifier: Node<Identifier>,
}

/// Function call expression
///
/// (C11 6.5.2)
#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub callee: Box<Node<Expression>>,
    pub arguments: Vec<Node<Expression>>,
}

/// Compound literal
///
/// (C11 6.5.2)
#[derive(Debug, PartialEq, Clone)]
pub struct CompoundLiteral {
    pub type_name: Node<TypeName>,
    pub initializer_list: Vec<Node<InitializerListItem>>,
}

/// SizeOf a type
///
/// (C11 6.5.3)
#[derive(Debug, PartialEq, Clone)]
pub struct SizeOfTy(pub Node<TypeName>);

/// Size of an unary expression
///
/// (C11 6.5.3)
#[derive(Debug, PartialEq, Clone)]
pub struct SizeOfVal(pub Box<Node<Expression>>);

/// Alignment of a type
///
/// (C11 6.5.3)
#[derive(Debug, PartialEq, Clone)]
pub struct AlignOf(pub Box<Node<TypeName>>);

/// All operators with one operand
///
/// (C11 6.5)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UnaryOperator {
    PostIncrement,
    PostDecrement,
    PreIncrement,
    PreDecrement,
    Address,
    Indirection,
    Plus,
    Minus,
    Complement,
    Negate,
}

/// Unary operator expression
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperatorExpression {
    pub operator: Node<UnaryOperator>,
    pub operand: Box<Node<Expression>>,
}

/// Cast expression
#[derive(Debug, PartialEq, Clone)]
pub struct CastExpression {
    pub type_name: Node<TypeName>,
    pub expression: Box<Node<Expression>>,
}

/// All operators with two operands
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BinaryOperator {
    Index,
    Multiply,
    Divide,
    Modulo,
    Plus,
    Minus,
    ShiftLeft,
    ShiftRight,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    Equals,
    NotEquals,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    Assign,
    AssignMultiply,
    AssignDivide,
    AssignModulo,
    AssignPlus,
    AssignMinus,
    AssignShiftLeft,
    AssignShiftRight,
    AssignBitwiseAnd,
    AssignBitwiseXor,
    AssignBitwiseOr,
}

/// Binary operators
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOperatorExpression {
    pub operator: Node<BinaryOperator>,
    pub lhs: Box<Node<Expression>>,
    pub rhs: Box<Node<Expression>>,
}

/// Conditional operator
#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalExpression {
    pub condition: Box<Node<Expression>>,
    pub then_expression: Box<Node<Expression>>,
    pub else_expression: Box<Node<Expression>>,
}

/// Variable argument list access
#[derive(Debug, PartialEq, Clone)]
pub struct VaArgExpression {
    pub va_list: Box<Node<Expression>>,
    pub type_name: Node<TypeName>,
}

/// Member offset expression
#[derive(Debug, PartialEq, Clone)]
pub struct OffsetOfExpression {
    pub type_name: Node<TypeName>,
    pub designator: Node<OffsetDesignator>,
}

/// Offset designator in a `offsetof` macro expansion
#[derive(Debug, PartialEq, Clone)]
pub struct OffsetDesignator {
    pub base: Node<Identifier>,
    pub members: Vec<Node<OffsetMember>>,
}

/// Single element of an offset designator
#[derive(Debug, PartialEq, Clone)]
pub enum OffsetMember {
    Member(Node<Identifier>),
    IndirectMember(Node<Identifier>),
    Index(Node<Expression>),
}
