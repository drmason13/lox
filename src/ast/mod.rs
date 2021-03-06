// Mod declarations

pub mod printer;

pub mod visitor;

// ast.rs

use crate::lex::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue {
    Number(f32),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Debug)]
pub enum Ast {
    Expr(Expr),
    Stmt(Stmt),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Grouping(Grouping),
    Binary(Binary),
    Unary(Unary),
    Literal(LiteralValue),
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

#[derive(Debug, Clone)]
pub struct Grouping(pub Box<Expr>);

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Literal(pub LiteralValue);

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(ExprStmt),
    PrintStmt(PrintStmt),
}

#[derive(Debug, Clone)]
pub struct ExprStmt(pub Expr);

#[derive(Debug, Clone)]
pub struct PrintStmt(pub Expr);

impl Stmt {
    pub fn expression_statement(expr: Expr) -> Self {
        Stmt::ExprStmt(ExprStmt(expr))
    }

    pub fn print_statement(expr: Expr) -> Self {
        Stmt::PrintStmt(PrintStmt(expr))
    }
}
