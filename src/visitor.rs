use crate::ast::*;

pub trait Visitor<T> {
    fn visit_grouping(&self, grouping: &Grouping) -> T;
    fn visit_binary(&self, binary: &Binary) -> T;
    fn visit_unary(&self, unary: &Unary) -> T;
    fn visit_literal(&self, literal_value: &LiteralValue) -> T;
    fn visit_expr(&self, expr: &Expr) -> T {
        match expr {
            Expr::Grouping(inner) => self.visit_grouping(inner),
            Expr::Binary(inner) => self.visit_binary(inner),
            Expr::Unary(inner) => self.visit_unary(inner),
            Expr::Literal(inner) => self.visit_literal(inner),
        }
    }
}

pub fn walk_expr(visitor: &impl Visitor<()>, expr: &Expr) {
    match expr {
        Expr::Grouping(ref inner) => visitor.visit_grouping(inner),
        Expr::Binary(Binary {
            ref left,
            operator: _,
            ref right,
        }) => {
            visitor.visit_expr(left);
            // visitor.visit_???(operator);
            visitor.visit_expr(right);
        }
        Expr::Unary(Unary {
            operator: _,
            ref right,
        }) => {
            // visitor.visit_???(operator);
            visitor.visit_expr(right);
        }
        Expr::Literal(ref inner) => visitor.visit_literal(inner),
    }
}
