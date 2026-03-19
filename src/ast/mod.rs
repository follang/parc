//! Abstract syntax tree
//!
//! Types in this module represent various pieces a C program can
//! contain after preprocessing phase. They mostly follow C11 grammar
//! naming conventions.
//!
//! References to C11 standard given in parenthesis refer to the
//! [ISO/IEC 9899:201x
//! draft](http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf)
//! published on April 12, 2011.
//!
//! A number of GNU extensions to the standard C are included here.
//! Types, struct fields or enum variants specific to GNU are marked as
//! "GNU extension" with a link to the relevant section of gcc
//! documentation. Supported extensions are:
//!
//! - attributes in various positions
//! - inline assembly statements and asm labels
//! - extensions to the initializer list syntax
//! - statement expressions
//! - `typeof` type specifiers

mod declarations;
mod expressions;
mod extensions;
mod lexical;
mod statements;

pub use self::declarations::*;
pub use self::expressions::*;
pub use self::extensions::*;
pub use self::lexical::*;
pub use self::statements::*;
