pub use crate::lex::LiteralValue;
use crate::lex::Token;

#[derive(Debug)]
pub enum Ast {
    Expr(Expr),
}

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary(Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Expr::Unary(Unary {
            operator,
            right: Box::new(right),
        })
    }

    pub fn literal_bool(value: bool) -> Self {
        Expr::Literal(LiteralValue::Bool(value))
    }

    pub fn literal_nil() -> Self {
        Expr::Literal(LiteralValue::Nil)
    }

    pub fn grouping(expr: Expr) -> Self {
        Expr::Grouping(Grouping(Box::new(expr)))
    }
}

#[derive(Debug)]
pub struct Grouping(pub Box<Expr>);

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Literal(pub LiteralValue);

#[derive(Debug)]
pub enum Expr {
    Grouping(Grouping),
    Binary(Binary),
    Unary(Unary),
    Literal(LiteralValue),
}
