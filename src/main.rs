use anyhow::Result;

use lox::Interpreter;

fn show_usage() {
    eprintln!("Usage: lox [script]");
    std::process::exit(64);
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mut lox = Interpreter::new();

    if let Some(path) = args.next() {
        if args.count() > 0 {
            show_usage();
        };
        lox.run_file(path)?;
    } else {
        lox.run_prompt()?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use lox::ast::*;
    use lox::lex::{Span, Token, TokenKind};

    #[test]
    fn test_prettyprint() {
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
            PrettyPrinter::print(&expression)
        );
    }
}
