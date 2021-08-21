use std::iter::once;

use crate::ast::*;
use crate::Visitor;

pub struct DebugPrinter;

impl<'a> DebugPrinter {
    pub fn print(expr: &Expr) -> String {
        let pp = DebugPrinter;
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

impl Visitor<String> for DebugPrinter {
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
pub struct RpnPrinter;

impl<'a> RpnPrinter {
    pub fn print(expr: &Expr) -> String {
        let pp = RpnPrinter;
        pp.visit_expr(expr)
    }

    fn parenthesize(
        &'a self,
        name: impl AsRef<str>,
        expressions: impl Iterator<Item = &'a Expr>,
    ) -> String {
        let mut print = String::new();

        for expr in expressions {
            print.push_str(&self.visit_expr(expr));
            print.push(' ');
        }

        print.push_str(name.as_ref());
        print
    }
}

impl Visitor<String> for RpnPrinter {
    fn visit_grouping(&self, grouping: &Grouping) -> String {
        self.visit_expr(&grouping.0)
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

#[cfg(test)]
mod test {
    use super::DebugPrinter;
    use super::RpnPrinter;
    use crate::ast::*;
    use crate::lex::{Span, Token, TokenKind};

    #[test]
    fn test_pretty_print() {
        let expression = Expr::Binary(Binary {
            left: Box::new(Expr::Unary(Unary {
                operator: Token {
                    kind: TokenKind::MINUS,
                    literal: None,
                    lexeme: "-".to_string(),
                    span: Span::new(),
                },
                right: Box::new(Expr::Literal(LiteralValue::Number(123.0))),
            })),
            operator: Token {
                kind: TokenKind::STAR,
                literal: None,
                lexeme: "*".to_string(),
                span: Span::new(),
            },
            right: Box::new(Expr::Grouping(Grouping(Box::new(Expr::Literal(
                LiteralValue::Number(45.67),
            ))))),
        });

        assert_eq!(
            "(* (- 123) (group 45.67))",
            DebugPrinter::print(&expression)
        );
    }

    #[test]
    fn test_polish_print() {
        let expression = Expr::Binary(Binary {
            left: Box::new(Expr::Binary(Binary {
                left: Box::new(Expr::Literal(LiteralValue::Number(1.0))),
                operator: Token {
                    kind: TokenKind::PLUS,
                    literal: None,
                    lexeme: "+".to_string(),
                    span: Span::new(),
                },
                right: Box::new(Expr::Literal(LiteralValue::Number(2.0))),
            })),
            operator: Token {
                kind: TokenKind::STAR,
                literal: None,
                lexeme: "*".to_string(),
                span: Span::new(),
            },
            right: Box::new(Expr::Binary(Binary {
                left: Box::new(Expr::Literal(LiteralValue::Number(4.0))),
                operator: Token {
                    kind: TokenKind::MINUS,
                    literal: None,
                    lexeme: "-".to_string(),
                    span: Span::new(),
                },
                right: Box::new(Expr::Literal(LiteralValue::Number(3.0))),
            })),
        });

        assert_eq!("(* (+ 1 2) (- 4 3))", DebugPrinter::print(&expression));
        assert_eq!("1 2 + 4 3 - *", RpnPrinter::print(&expression));
    }
}
