use crate::ast::*;
use crate::evaluate::{Error, ErrorKind};
use crate::lex::TokenKind;
use crate::OwnedVisitor;

pub fn is_truthy(value: LiteralValue) -> bool {
    match value {
        LiteralValue::Bool(false) => false,
        LiteralValue::Nil => false,
        _ => true,
    }
}

pub struct Evaluator;

impl<'a> Evaluator {
    pub fn evaluate(expr: Expr) -> Result<LiteralValue, Error> {
        let ev = Evaluator;
        ev.visit_expr(expr)
    }

    fn eval(&self, expr: Expr) -> Result<LiteralValue, Error> {
        self.visit_expr(expr)
    }
}

impl OwnedVisitor<Result<LiteralValue, Error>> for Evaluator {
    fn visit_grouping(&self, grouping: Grouping) -> Result<LiteralValue, Error> {
        self.eval(*grouping.0)
    }
    fn visit_binary(&self, binary: Binary) -> Result<LiteralValue, Error> {
        let left_value = self.eval(*binary.left)?;
        let right_value = self.eval(*binary.right)?;
        match binary.operator.kind {
            //
            // Addition
            //
            TokenKind::PLUS => match (left_value, right_value) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                    Ok(LiteralValue::Number(l + r))
                }
                // concatenate strings
                (LiteralValue::String(mut l), LiteralValue::String(r)) => {
                    l.push_str(&r);
                    Ok(LiteralValue::String(l))
                }
                // cast booleans as 0 or 1
                (LiteralValue::Number(l), LiteralValue::Bool(x)) => {
                    let r = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Number(l + r))
                }
                (LiteralValue::Bool(x), LiteralValue::Number(r)) => {
                    let l = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Number(l + r))
                }
                (LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
                    let l = if l { 1.0 } else { 0.0 };
                    let r = if r { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Number(l + r))
                }
                _ => Err(Error::tokened("", binary.operator, ErrorKind::BadAddition)),
            },
            //
            // Subtraction
            //
            TokenKind::MINUS => match (left_value, right_value) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                    Ok(LiteralValue::Number(l - r))
                }
                // cast booleans as 0 or 1
                (LiteralValue::Number(l), LiteralValue::Bool(x)) => {
                    let r = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Number(l - r))
                }
                (LiteralValue::Bool(x), LiteralValue::Number(r)) => {
                    let l = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Number(l - r))
                }
                (LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
                    let l = if l { 1.0 } else { 0.0 };
                    let r = if r { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Number(l - r))
                }
                _ => Err(Error::tokened(
                    "",
                    binary.operator,
                    ErrorKind::BadSubtraction,
                )),
            },
            //
            // Multiplication
            //
            TokenKind::STAR => match (left_value, right_value) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                    Ok(LiteralValue::Number(l * r))
                }
                // string repitition
                (LiteralValue::String(l), LiteralValue::Number(r)) => {
                    let mut word = String::new();
                    // ignore fractional parts of a number OR error?
                    let count = r.trunc() as usize;
                    // I think error! :)
                    if r.fract() != 0.0 {
                        return Err(Error::tokened(
                            "",
                            binary.operator,
                            ErrorKind::BadStringRepCount,
                        ));
                    }
                    for _ in 0..count {
                        word.push_str(&l);
                    }
                    Ok(LiteralValue::String(word))
                }
                // cast booleans as 0 or 1
                (LiteralValue::Number(l), LiteralValue::Bool(x)) => {
                    let r = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Number(l * r))
                }
                (LiteralValue::Bool(x), LiteralValue::Number(r)) => {
                    let l = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Number(l * r))
                }
                (LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
                    let l = if l { 1.0 } else { 0.0 };
                    let r = if r { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Number(l * r))
                }
                _ => Err(Error::tokened(
                    "",
                    binary.operator,
                    ErrorKind::BadSubtraction,
                )),
            },
            //
            // Division
            //
            TokenKind::SLASH => match (left_value, right_value) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                    Ok(LiteralValue::Number(l / r))
                }
                // cast booleans as 0 or 1
                (LiteralValue::Number(l), LiteralValue::Bool(x)) => {
                    let r = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Number(l / r))
                }
                (LiteralValue::Bool(x), LiteralValue::Number(r)) => {
                    let l = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Number(l / r))
                }
                (LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
                    let l = if l { 1.0 } else { 0.0 };
                    let r = if r { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Number(l / r))
                }
                _ => Err(Error::tokened("", binary.operator, ErrorKind::BadDivision)),
            },
            //
            // Comparisons
            //
            TokenKind::GREATER => match (left_value, right_value) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Bool(l > r)),
                // cast booleans as 0 or 1
                (LiteralValue::Number(l), LiteralValue::Bool(x)) => {
                    let r = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Bool(l > r))
                }
                (LiteralValue::Bool(x), LiteralValue::Number(r)) => {
                    let l = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Bool(l > r))
                }
                (LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
                    let l = if l { 1.0 } else { 0.0 };
                    let r = if r { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Bool(l > r))
                }
                _ => Err(Error::tokened(
                    "",
                    binary.operator,
                    ErrorKind::BadComparison,
                )),
            },
            TokenKind::GREATER_EQUAL => match (left_value, right_value) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Bool(l > r)),
                // cast booleans as 0 or 1
                (LiteralValue::Number(l), LiteralValue::Bool(x)) => {
                    let r = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Bool(l >= r))
                }
                (LiteralValue::Bool(x), LiteralValue::Number(r)) => {
                    let l = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Bool(l >= r))
                }
                (LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
                    let l = if l { 1.0 } else { 0.0 };
                    let r = if r { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Bool(l >= r))
                }
                _ => Err(Error::tokened(
                    "",
                    binary.operator,
                    ErrorKind::BadComparison,
                )),
            },
            TokenKind::LESS => match (left_value, right_value) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Bool(l > r)),
                // cast booleans as 0 or 1
                (LiteralValue::Number(l), LiteralValue::Bool(x)) => {
                    let r = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Bool(l < r))
                }
                (LiteralValue::Bool(x), LiteralValue::Number(r)) => {
                    let l = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Bool(l < r))
                }
                (LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
                    let l = if l { 1.0 } else { 0.0 };
                    let r = if r { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Bool(l < r))
                }
                _ => Err(Error::tokened(
                    "",
                    binary.operator,
                    ErrorKind::BadComparison,
                )),
            },
            TokenKind::LESS_EQUAL => match (left_value, right_value) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => Ok(LiteralValue::Bool(l > r)),
                // cast booleans as 0 or 1
                (LiteralValue::Number(l), LiteralValue::Bool(x)) => {
                    let r = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Bool(l <= r))
                }
                (LiteralValue::Bool(x), LiteralValue::Number(r)) => {
                    let l = if x { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Bool(l <= r))
                }
                (LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
                    let l = if l { 1.0 } else { 0.0 };
                    let r = if r { 1.0 } else { 0.0 };
                    Ok(LiteralValue::Bool(l <= r))
                }
                _ => Err(Error::tokened(
                    "",
                    binary.operator,
                    ErrorKind::BadComparison,
                )),
            },
            _ => unreachable!(
                "cannot evaluate this token here in a binary expression, bad input to evaluator"
            ),
        }
    }
    fn visit_unary(&self, unary: Unary) -> Result<LiteralValue, Error> {
        let value = self.eval(*unary.right)?;
        match (unary.operator.kind, value) {
            (TokenKind::MINUS, LiteralValue::Number(n)) => Ok(LiteralValue::Number(-n)),
            (TokenKind::MINUS, _) => Err(Error::tokened(
                "",
                unary.operator,
                ErrorKind::BadNumericalNegation,
            )),
            (TokenKind::BANG, value) => Ok(LiteralValue::Bool(!is_truthy(value))),
            _ => unreachable!(
                "cannot evaluate this token here in a unary expression, bad input to evaluator"
            ),
        }
    }
    fn visit_literal(&self, literal: LiteralValue) -> Result<LiteralValue, Error> {
        Ok(literal)
    }
}

#[cfg(test)]
mod test {
    use super::{Error, Evaluator};
    use crate::ast::*;
    use crate::lex::{Span, Token, TokenKind};

    #[test]
    fn evaluation_works() -> Result<(), Error> {
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

        assert_eq!(LiteralValue::Number(3.), Evaluator::evaluate(expression)?);
        Ok(())
    }

    #[test]
    fn evaluation_string_concatenation_works() -> Result<(), Error> {
        let expression = Expr::Binary(Binary {
            left: Box::new(Expr::Binary(Binary {
                left: Box::new(Expr::Literal(LiteralValue::String("Hello".into()))),
                operator: Token {
                    kind: TokenKind::PLUS,
                    literal: None,
                    lexeme: "+".to_string(),
                    span: Span::new(),
                },
                right: Box::new(Expr::Literal(LiteralValue::String(" World".into()))),
            })),
            operator: Token {
                kind: TokenKind::PLUS,
                literal: None,
                lexeme: "+".to_string(),
                span: Span::new(),
            },
            right: Box::new(Expr::Binary(Binary {
                left: Box::new(Expr::Literal(LiteralValue::String("!".into()))),
                operator: Token {
                    kind: TokenKind::STAR,
                    literal: None,
                    lexeme: "*".to_string(),
                    span: Span::new(),
                },
                right: Box::new(Expr::Literal(LiteralValue::Number(3.0))),
            })),
        });

        assert_eq!(
            LiteralValue::String("Hello World!!!".into()),
            Evaluator::evaluate(expression)?
        );
        Ok(())
    }
}
