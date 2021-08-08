mod lexer;
mod token;

pub use lexer::Lexer;
pub use token::{escape_string, LiteralValue, Span, Token, TokenKind};
