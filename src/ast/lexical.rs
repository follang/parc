use crate::ast::declarations::TS18661FloatType;

// From 6.4 Lexical elements

/// Variable, function and other names that are not type names
///
/// (C11 6.4.2)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Identifier {
    pub name: String,
}

/// Constant literals
///
/// C11 places string literals under primary expressions, thus they
/// are not included here.
///
/// (C11 6.4.4)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Constant {
    Integer(Integer),
    Float(Float),
    Character(String),
}

/// Integer number literal
///
/// (C11 6.4.4.1)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Integer {
    pub base: IntegerBase,
    pub number: Box<str>,
    pub suffix: IntegerSuffix,
}

/// Base of the integer literal
///
/// (C11 6.4.4.1)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum IntegerBase {
    Decimal,
    Octal,
    Hexadecimal,
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Binary-constants.html)
    Binary,
}

/// Suffix of an integer literal
///
/// (C11 6.4.4.1)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct IntegerSuffix {
    /// Minimum size of the integer literal
    pub size: IntegerSize,
    /// Integer literal has unsigned type
    pub unsigned: bool,
    /// Integer literal is an imaginary part of a complex number
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Complex.html) suffixes `i` and `j`.
    pub imaginary: bool,
}

/// Size part of a integer literal suffix
///
/// (C11 6.4.4.1)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum IntegerSize {
    /// no `l` or `ll`
    Int = 0,
    /// `l`
    Long,
    /// `ll`
    LongLong,
}

/// Floating point number literal
///
/// (C11 6.4.4.2)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Float {
    pub base: FloatBase,
    pub number: Box<str>,
    pub suffix: FloatSuffix,
}

/// Floating point number base
///
/// (C11 6.4.4.2)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FloatBase {
    Decimal,
    Hexadecimal,
}

/// Floating point number suffix
///
/// (C11 6.4.4.2)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FloatSuffix {
    pub format: FloatFormat,
    /// Integer literal is an imaginary part of a complex number
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Complex.html) suffixes `i` and `j`.
    pub imaginary: bool,
}

/// Floating point literal format specified by the suffix
///
/// (C11 6.4.4.2)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FloatFormat {
    /// `f` suffix
    Float,
    /// no suffix
    Double,
    /// `l` suffix
    LongDouble,
    /// [ISO/IEC TS 18661-2:2015](http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1912.pdf)
    /// `df`, `dd`, `dl` suffixes
    ///
    /// [ISO/IEC TS 18661-3:2015](http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1945.pdf)
    /// `fN`, `fNx`, `dN`, `dNx` suffixes
    TS18661Format(TS18661FloatType),
}

/// String literal
///
/// (C11 6.4.5)
pub type StringLiteral = Vec<String>;
