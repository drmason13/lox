use thiserror::Error;

use crate::lex::Token;

#[derive(Clone, Error, Debug)]
#[error("{kind} Occurred at {token} {message}")]
pub struct Error {
    token: Token,
    kind: ErrorKind,
    message: String,
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    BadNumericalNegation,
    BadBooleanNegation,
    BadAddition,
    BadSubtraction,
    BadMultiplication,
    BadDivision,
    BadStringRepCount,
    BadComparison,
}

impl Error {
    pub fn tokened(msg: impl Into<String>, token: Token, kind: ErrorKind) -> Self {
        Error {
            kind,
            message: msg.into(),
            token,
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::BadNumericalNegation => write!(f, "Bad Numerical Negation"),
            ErrorKind::BadBooleanNegation => write!(f, "Bad Boolean Negation"),
            ErrorKind::BadAddition => write!(f, "Bad Addition"),
            ErrorKind::BadSubtraction => write!(f, "Bad Subtraction"),
            ErrorKind::BadMultiplication => write!(f, "Bad Multiplication"),
            ErrorKind::BadDivision => write!(f, "Bad Division"),
            ErrorKind::BadStringRepCount => {
                write!(f, "Bad count for string repitition, expected an integer")
            }
            ErrorKind::BadComparison => write!(f, "Bad Comparison"),
        }
    }
}
