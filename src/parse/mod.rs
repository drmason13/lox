// Module declarations

mod error;
pub use error::{Error, ErrorKind};

// lexer.rs

use peekaboo::Peekaboo;
// for debugging, but too verbose so we comment out the #[trace] attributes most of the time
#[allow(unused_imports)]
use trace::trace;

use crate::ast::*;

use crate::lex::{Token, TokenKind};

trace::init_depth_var!();

pub type ParseResult = Result<Expr, Error>;

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekaboo<I>,
    current: usize,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: Peekaboo<I>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Simply a wrapper around expression for now
    //#[trace]
    pub fn parse(&mut self) -> ParseResult {
        match self.expression() {
            Err(err) => {
                if !err.is_fatal() {
                    // report error, synchronize and continue
                    self.report(err);
                    self.synchronize();
                    if self.tokens.next().is_none() {
                        // synchronize skipped all remaining tokens, no more errors to report.
                        return Err(Error::KindOnly(ErrorKind::EOFWhileSynchronizing));
                    }
                } else {
                    return Err(err);
                }
                // we can guarantee no infinite loop of errors as long as the token stream is finite
                self.expression()
            }
            ok => ok,
        }
    }

    //#[trace]
    pub fn expression(&mut self) -> ParseResult {
        self.equality()
    }

    //#[trace]
    fn equality(&mut self) -> ParseResult {
        let left = self.comparison()?;

        while let Some(operator) = self
            .tokens
            .next_if(|ref t| t.kind == TokenKind::EQUAL_EQUAL || t.kind == TokenKind::BANG_EQUAL)
        {
            self.current += 1;
            let right = self.equality()?;
            return Ok(Expr::binary(left, operator, right));
        }

        Ok(left)
    }

    //#[trace]
    fn comparison(&mut self) -> ParseResult {
        let left = self.terms()?;

        while let Some(operator) = self.tokens.next_if(|ref t| {
            t.kind == TokenKind::GREATER_EQUAL
                || t.kind == TokenKind::LESS_EQUAL
                || t.kind == TokenKind::GREATER_EQUAL
                || t.kind == TokenKind::LESS
        }) {
            self.current += 1;
            let right = self.comparison()?;
            return Ok(Expr::binary(left, operator, right));
        }

        Ok(left)
    }

    //#[trace]
    fn terms(&mut self) -> ParseResult {
        let left = self.factor()?;

        while let Some(operator) = self
            .tokens
            .next_if(|ref t| t.kind == TokenKind::PLUS || t.kind == TokenKind::MINUS)
        {
            self.current += 1;
            let right = self.terms()?;
            return Ok(Expr::binary(left, operator, right));
        }

        Ok(left)
    }

    //#[trace]
    fn factor(&mut self) -> ParseResult {
        let left = self.unary()?;

        while let Some(operator) = self
            .tokens
            .next_if(|ref t| t.kind == TokenKind::STAR || t.kind == TokenKind::SLASH)
        {
            self.current += 1;
            let right = self.factor()?;
            return Ok(Expr::binary(left, operator, right));
        }

        Ok(left)
    }

    //#[trace]
    fn unary(&mut self) -> ParseResult {
        if let Some(operator) = self
            .tokens
            .next_if(|ref t| t.kind == TokenKind::BANG || t.kind == TokenKind::MINUS)
        {
            self.current += 1;
            let right = self.unary()?;
            return Ok(Expr::unary(operator, right));
        }

        self.primary()
    }

    //#[trace]
    fn primary(&mut self) -> ParseResult {
        match self.tokens.next() {
            Some(Token {
                lexeme: _,
                span: _,
                literal: _,
                kind: TokenKind::TRUE,
            }) => Ok(Expr::literal_bool(true)),
            Some(Token {
                lexeme: _,
                span: _,
                literal: _,
                kind: TokenKind::FALSE,
            }) => Ok(Expr::literal_bool(false)),
            Some(Token {
                lexeme: _,
                span: _,
                literal: _,
                kind: TokenKind::NIL,
            }) => Ok(Expr::literal_nil()),
            Some(Token {
                lexeme: _,
                span: _,
                literal: Some(value),
                kind: TokenKind::NUMBER | TokenKind::STRING,
            }) => Ok(Expr::Literal(value)),
            Some(Token {
                lexeme: _,
                span: _,
                literal: _,
                kind: TokenKind::LEFT_PAREN,
            }) => {
                // match a grouping
                let expr = self.expression()?;
                match self
                    .tokens
                    .next_if(|ref t| t.kind == TokenKind::RIGHT_PAREN)
                {
                    None => {
                        if let Some(failed_token) = self.tokens.next() {
                            Err(Error::with_token(
                                "Expected a closing parenthesis",
                                ErrorKind::UnclosedParentheses,
                                failed_token,
                            ))
                        } else {
                            Err(Error::without_token(
                                "While a parenthesis was open",
                                ErrorKind::UnexpectedEOF,
                            ))
                        }
                    }
                    Some(_) => Ok(Expr::grouping(expr)),
                }
            }
            Some(unexpected_token) => Err(Error::with_token(
                "Expected a literal value, or an opening parenthesis",
                ErrorKind::InvalidExpression,
                unexpected_token,
            )),
            None => Err(Error::without_token(
                "while parsing an expression",
                ErrorKind::UnexpectedEOF,
            )),
        }
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.tokens.next() {
            match token.kind {
                TokenKind::SEMICOLON => return,
                _ => {
                    if let Some(ref next_token) = self.tokens.peek() {
                        match next_token.kind {
                            TokenKind::CLASS
                            | TokenKind::FUN
                            | TokenKind::VAR
                            | TokenKind::FOR
                            | TokenKind::IF
                            | TokenKind::WHILE
                            | TokenKind::PRINT
                            | TokenKind::RETURN => return,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn report(&mut self, err: Error) -> () {
        eprintln!("{}", err);
    }
}

#[cfg(test)]
mod test {
    use crate::lex::Lexer;
    use crate::printer::DebugPrinter;

    #[test]
    fn test_parser() {
        let source = r#"
            2 + (3 - 4) * 9 != "foo"
        "#;

        let scanner = Lexer::new(source);
        // For now, just pretty print the parsed AST.
        assert_eq!(
            r#"(!= (+ 2 (* (group (- 3 4)) 9)) "foo")"#.to_string(),
            DebugPrinter::print(
                &scanner
                    .advance_to_parsing()
                    .parse()
                    .expect("Error while parsing!")
            )
        );
    }
}
