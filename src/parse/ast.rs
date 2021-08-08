use std::iter::once;

pub use crate::lex::LiteralValue;
use crate::{lex::Token, Visitor};

pub struct Grouping(pub Box<Expr>);

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Literal(pub LiteralValue);

pub enum Expr {
    Grouping(Grouping),
    Binary(Binary),
    Unary(Unary),
    Literal(LiteralValue),
}

pub struct PrettyPrinter;

impl<'a> PrettyPrinter {
    pub fn print(expr: &Expr) -> String {
        let pp = PrettyPrinter;
        pp.visit_expr(expr)
    }

    fn parenthesize(
        &'a self,
        name: impl AsRef<str>,
        expressions: impl Iterator<Item = &'a Expr>,
    ) -> String {
        let mut print = String::new();
        print.push('(');
        print.push_str(name.as_ref());

        for expr in expressions {
            print.push(' ');
            print.push_str(&self.visit_expr(expr))
        }

        print.push(')');
        print
    }
}

impl Visitor<String> for PrettyPrinter {
    fn visit_grouping(&self, grouping: &Grouping) -> String {
        self.parenthesize("group", once(grouping.0.as_ref()))
    }
    fn visit_binary(&self, binary: &Binary) -> String {
        self.parenthesize(
            &binary.operator.lexeme,
            once(binary.left.as_ref()).chain(once(binary.right.as_ref())),
        )
    }
    fn visit_unary(&self, unary: &Unary) -> String {
        self.parenthesize(&unary.operator.lexeme, once(unary.right.as_ref()))
    }
    fn visit_literal(&self, literal: &LiteralValue) -> String {
        literal.to_string()
    }
}
