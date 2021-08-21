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

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekaboo<I>,
    current: usize,
}

impl<I> Iterator for Parser<I>
where
    I: Iterator<Item = Token>,
{
    type Item = Result<Stmt, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // check if we have finished receiving tokens
        if self.tokens.peek().is_none() {
            return None;
        }
        // else return the next Statement
        Some(self.statement())
    }
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: Peekaboo<I>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn statement(&mut self) -> Result<Stmt, Error> {
        // we just checked that peek() is Some
        match self.tokens.peek().unwrap() {
            Token {
                lexeme: _,
                literal: _,
                span: _,
                kind: TokenKind::PRINT,
            } => self.print_statement(),
            _ => self.expression_statement(),
        }
    }

    pub fn print_statement(&mut self) -> Result<Stmt, Error> {
        // consume the PRINT token
        self.tokens.next();

        let expr = self.expression()?;
        Ok(Stmt::print_statement(expr))
    }

    pub fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        if self
            .tokens
            .next_if(|ref t| t.kind == TokenKind::SEMICOLON)
            .is_none()
        {
            if let Some(failed_token) = self.tokens.next() {
                return Err(Error::with_token(
                    "Expect ';' after expression.",
                    ErrorKind::ExprStmtMissingSemicolon,
                    failed_token,
                ));
            } else {
                return Err(Error::without_token(
                    "Expect ';' after expression, found EOF.",
                    ErrorKind::ExprStmtMissingSemicolon,
                ));
            }
        }

        Ok(Stmt::expression_statement(expr))
    }

    pub fn expression_wrapper(&mut self) -> Result<Expr, Error> {
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
    pub fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    //#[trace]
    fn equality(&mut self) -> Result<Expr, Error> {
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
    fn comparison(&mut self) -> Result<Expr, Error> {
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
    fn terms(&mut self) -> Result<Expr, Error> {
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
    fn factor(&mut self) -> Result<Expr, Error> {
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
    fn unary(&mut self) -> Result<Expr, Error> {
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
    fn primary(&mut self) -> Result<Expr, Error> {
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
    use crate::ast::{ExprStmt, Stmt};
    use crate::lex::Lexer;
    use crate::printer::DebugPrinter;

    #[test]
    fn test_parser() {
        let source = r#"
            2 + (3 - 4) * 9 != "foo"
        "#;

        let scanner = Lexer::new(source);

        let statement = &scanner
            .advance_to_parsing()
            .next()
            .unwrap()
            .expect("Error while parsing!");

        if let Stmt::ExprStmt(ExprStmt(e)) = statement {
            assert_eq!(
                r#"(!= (+ 2 (* (group (- 3 4)) 9)) "foo")"#.to_string(),
                DebugPrinter::print(e)
            );
        } else {
            panic!("Expected source to parse as an expression statement")
        }
    }
}
