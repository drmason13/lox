use thiserror::Error;

use crate::ast::*;
use crate::lexer::TokenKind;
use crate::Visitor;

pub struct Evaluator;

impl<'a> Evaluator {
    pub fn evaluate(expr: &Expr) -> Result<LiteralValue, EvaluationError> {
        let ev = Evaluator;
        ev.visit_expr(expr)
    }

    fn eval(&self, expr: &Expr) -> Result<LiteralValue, EvaluationError> {
        self.visit_expr(expr)
    }
}

impl Visitor<Result<LiteralValue, EvaluationError>> for Evaluator {
    fn visit_grouping(&self, grouping: &Grouping) -> Result<LiteralValue, EvaluationError> {
        self.eval(&Expr::Grouping(grouping.clone()))
    }
    fn visit_binary(&self, binary: &Binary) -> Result<LiteralValue, EvaluationError> {
        let left_value = self.eval(binary.left.as_ref())?;
        let right_value = self.eval(binary.right.as_ref())?;
        match binary.operator.kind {
            TokenKind::PLUS => match (left_value, right_value) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                    Ok(LiteralValue::Number(l + r))
                }
                (LiteralValue::String(mut l), LiteralValue::String(r)) => {
                    l.push_str(&r);
                    Ok(LiteralValue::String(l))
                }
                _ => Err(EvaluationError::BadAddition),
            },
            TokenKind::MINUS => match (left_value, right_value) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                    Ok(LiteralValue::Number(l - r))
                }
                _ => Err(EvaluationError::BadSubtraction),
            },
            // repeat string syntax!
            // "-" * 5 == "-----"
            // true
            TokenKind::STAR => match (left_value, right_value) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                    Ok(LiteralValue::Number(l * r))
                }
                (LiteralValue::String(l), LiteralValue::Number(r)) => {
                    let mut word = String::new();
                    // ignore fractional parts of a number OR error?
                    let count = r.trunc() as usize;
                    // I think error! :)
                    if r.fract() != 0.0 {
                        return Err(EvaluationError::BadStringRepCount);
                    }
                    for _ in 0..count {
                        word.push_str(&l);
                    }
                    Ok(LiteralValue::String(word))
                }
                _ => Err(EvaluationError::BadMultiplication),
            },
            _ => unreachable!(
                "cannot evaluate this token here in a binary expression, bad input to evaluator"
            ),
        }
    }
    fn visit_unary(&self, unary: &Unary) -> Result<LiteralValue, EvaluationError> {
        let value = self.eval(&unary.right)?;
        match (unary.operator.kind, value) {
            (TokenKind::MINUS, LiteralValue::Number(n)) => Ok(LiteralValue::Number(-n)),
            (TokenKind::MINUS, _) => Err(EvaluationError::BadNumericalNegation),
            (TokenKind::BANG, LiteralValue::Bool(x)) => Ok(LiteralValue::Bool(!x)),
            (TokenKind::BANG, _) => Err(EvaluationError::BadBooleanNegation),
            _ => unreachable!(
                "cannot evaluate this token here in a unary expression, bad input to evaluator"
            ),
        }
    }
    fn visit_literal(&self, literal: &LiteralValue) -> Result<LiteralValue, EvaluationError> {
        Ok(literal.clone())
    }
}

#[derive(Clone, Error, Debug)]
pub enum EvaluationError {
    #[error("Bad Numerical Negation")]
    BadNumericalNegation,
    #[error("Bad Boolean Negation")]
    BadBooleanNegation,
    #[error("Bad Addition")]
    BadAddition,
    #[error("Bad Subtraction")]
    BadSubtraction,
    #[error("Bad Multiplication")]
    BadMultiplication,
    #[error("Bad count for string repitition, expected an integer")]
    BadStringRepCount,
}

#[cfg(test)]
mod test {
    use super::{EvaluationError, Evaluator};
    use crate::ast::*;
    use crate::lexer::{Span, Token, TokenKind};

    #[test]
    fn evaluation_works() -> Result<(), EvaluationError> {
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

        assert_eq!(LiteralValue::Number(3.), Evaluator::evaluate(&expression)?);
        Ok(())
    }

    #[test]
    fn evaluation_string_concatenation_works() -> Result<(), EvaluationError> {
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
            Evaluator::evaluate(&expression)?
        );
        Ok(())
    }
}
