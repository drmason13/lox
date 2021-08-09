use peekaboo::Peekaboo;
#[allow(unused_imports)]
use trace::trace;

use crate::{
    ast::*,
    lex::{Token, TokenKind},
    LoxError,
};

trace::init_depth_var!();

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekaboo<I>,
    current: usize,
    errors: Vec<LoxError>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: Peekaboo<I>) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    /// Simply a wrapper around expression for now
    //#[trace]
    pub fn parse(&mut self) -> Result<Expr, LoxError> {
        self.expression()
    }

    //#[trace]
    pub fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    //#[trace]
    fn equality(&mut self) -> Result<Expr, LoxError> {
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
    fn comparison(&mut self) -> Result<Expr, LoxError> {
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
    fn terms(&mut self) -> Result<Expr, LoxError> {
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
    fn factor(&mut self) -> Result<Expr, LoxError> {
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
    fn unary(&mut self) -> Result<Expr, LoxError> {
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
    fn primary(&mut self) -> Result<Expr, LoxError> {
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
                        let failed_token = self.tokens.next();
                        self.error("Expected a closing parenthesis, found", failed_token)
                        // TODO: Panic or unwind or something?
                    }
                    Some(_) => Ok(Expr::grouping(expr)),
                }
            }
            Some(unexpected_token) => {
                self.error(
                    "Expected a literal expression, found",
                    Some(unexpected_token),
                )
                // TODO: Panic or unwind or something?
            }
            None => {
                self.error("", None)
                // TODO: Panic or unwind or something?
            }
        }
    }

    // Helper methods

    //#[trace]
    /// This function always returns Err
    fn error(
        &mut self,
        msg: impl Into<String> + std::fmt::Debug,
        token: Option<Token>,
    ) -> Result<Expr, LoxError> {
        println!("ERROR");
        let error = match token.ok_or(LoxError::bare("Unexpected end of Input")) {
            Ok(token) => LoxError::syntax_error(msg, token),
            Err(e) => e,
        };
        self.errors.push(error.clone());
        Err(error)
    }
}

#[cfg(test)]
mod test {
    use crate::lex::Lexer;
    use crate::DebugPrinter;

    #[test]
    fn test_pretty_print() {
        let source = r#"
            2 + (3 - 4) * 9 != "foo"
        "#;

        let scanner = Lexer::new(source.into());
        // For now, just pretty print the parsed AST.
        assert_eq!(
            "(!= (+ 2 (* (group (- 3 4)) 9)) `foo`)".to_string(),
            DebugPrinter::print(
                &scanner
                    .advance_to_parsing()
                    .parse()
                    .expect("Error while parsing!")
            )
        );
    }
}
