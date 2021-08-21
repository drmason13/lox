use thiserror::Error;

use crate::lex::Token;

/// The Error type for `Parser`
#[derive(Clone, Error, Debug)]
pub enum Error {
    #[error("{kind} Occurred at {token} {message}")]
    TokenedError {
        kind: ErrorKind,
        message: String,
        token: Token,
    },
    #[error("{kind} {message}")]
    UntokenedError { kind: ErrorKind, message: String },
    #[error("{0}")]
    KindOnly(ErrorKind),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    EOFWhileSynchronizing,
    InvalidExpression,
    UnclosedParentheses,
    UnexpectedEOF,
    InternalInterpreterError,
    Fatal,
    Other,
}

impl Error {
    pub fn without_token(msg: impl Into<String>, kind: ErrorKind) -> Self {
        Error::UntokenedError {
            kind,
            message: msg.into(),
        }
    }

    pub fn with_token(msg: impl Into<String>, kind: ErrorKind, token: Token) -> Self {
        Error::TokenedError {
            kind,
            token,
            message: msg.into(),
        }
    }

    pub fn internal_interpreter_error(msg: impl Into<String>, token: Token) -> Self {
        Error::TokenedError {
            kind: ErrorKind::InternalInterpreterError,
            token,
            message: msg.into(),
        }
    }

    pub fn fatal(msg: impl Into<String>) -> Self {
        Error::UntokenedError {
            kind: ErrorKind::Fatal,
            message: msg.into(),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::TokenedError {
                kind,
                message: _message,
                token: _token,
            } => kind.clone(),
            Error::UntokenedError {
                kind,
                message: _message,
            } => kind.clone(),
            Error::KindOnly(kind) => kind.clone(),
        }
    }

    pub fn is_fatal(&self) -> bool {
        self.kind() == ErrorKind::Fatal
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::EOFWhileSynchronizing => write!(f, "Encountered errors while parsing."),
            ErrorKind::InvalidExpression => write!(f, "Invalid expression."),
            ErrorKind::UnclosedParentheses => write!(f, "Unclosed Parentheses."),
            ErrorKind::UnexpectedEOF => write!(f, "Unexpected End of Source Code."),
            ErrorKind::InternalInterpreterError => write!(f, "Internal Interpreter Error."),
            ErrorKind::Fatal => write!(f, "Fatal Error!"),
            ErrorKind::Other => write!(f, "Unknown Error."),
        }
    }
}
