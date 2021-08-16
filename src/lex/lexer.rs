use std::str::Chars;

// Peekaboo is for double peeking
use peekaboo::{IteratorPeekabooExt, Peekaboo};

use crate::{LoxError, Parser};

use super::{
    token::{LiteralValue, Token, TokenKind},
    Span,
};

pub struct Lexer {
    source: String,
}

impl<'source> Lexer {
    pub fn new(source: impl Into<String>) -> Self {
        Lexer {
            source: source.into(),
        }
    }

    pub fn scan_tokens(&'source self) -> TokenStream<'source> {
        TokenStream {
            source: self.source.chars().peekaboo(),
            span: Span::new(),
        }
    }

    pub fn advance_to_parsing(&'source self) -> Parser<impl Iterator<Item = Token> + 'source> {
        let tokens = self
            .scan_tokens()
            .filter_map(|token_res| match token_res {
                Ok(token) => Some(token),
                Err(e) => {
                    println!("{}", e);
                    None
                }
            })
            .peekaboo();
        Parser::new(tokens)
    }
}

/// TokenStream is a stream of tokens interpreted from individual characters of a source String
///
/// TokenSteam implements Iterator (but not Stream...)
///
/// The Iterator Item is `Result<Token, LoxError>` which allows the caller can decide how to
/// handle individual invalid tokens. They may choose to error immediately or keep going
/// and report errors once the TokenStream has ended.
pub struct TokenStream<'source> {
    source: Peekaboo<Chars<'source>>,
    span: Span,
}

type MaybeTokenArgs = Option<Result<(TokenKind, Option<LiteralValue>), LoxError>>;

impl<'source> TokenStream<'source> {
    /// This is the function
    fn scan_token(&mut self) -> Option<Result<Token, LoxError>> {
        if let Some(c) = self.advance() {
            match self.match_char(c) {
                Some(Ok((token_kind, literal_value))) => {
                    Some(Ok(self.make_token(token_kind, literal_value)))
                }
                Some(Err(e)) => Some(Err(e)),
                None => {
                    // we consumed some lexemes but skipped making a token
                    // e.g. we just lexed a comment!
                    // we keep going to return a Token otherwise someone will think we ran out of tokens!
                    self.next()
                }
            }
        } else {
            // no more characters to match, TokenStream has finished
            None
        }
    }

    /// Handles the logic of how to process the next token based on a single character
    /// It calls out to other more specific functions that handle the logic based on the first character match
    fn match_char(&mut self, next_char: char) -> MaybeTokenArgs {
        match next_char {
            // ignore whitespace
            ' ' | '\t' | '\r' => None,
            // increment lines on every newline
            '\n' => None,
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(c),
            c if (c.is_ascii_alphabetic() || c == '_') => self.keyword(c),
            c => self.operator(c),
        }
    }

    fn operator(&mut self, c: char) -> MaybeTokenArgs {
        let output = match c {
            '(' => Some(Ok((TokenKind::LEFT_PAREN, None))),
            ')' => Some(Ok((TokenKind::RIGHT_PAREN, None))),
            '{' => Some(Ok((TokenKind::LEFT_BRACE, None))),
            '}' => Some(Ok((TokenKind::RIGHT_BRACE, None))),
            ',' => Some(Ok((TokenKind::COMMA, None))),
            '.' => Some(Ok((TokenKind::DOT, None))),
            '-' => Some(Ok((TokenKind::MINUS, None))),
            '+' => Some(Ok((TokenKind::PLUS, None))),
            ';' => Some(Ok((TokenKind::SEMICOLON, None))),
            '*' => Some(Ok((TokenKind::STAR, None))),
            '/' => {
                if self.advance_if_eq('/').is_some() {
                    self.line_comment();
                    None
                } else {
                    Some(Ok((TokenKind::SLASH, None)))
                }
            }
            '!' => {
                if self.advance_if_eq('=').is_some() {
                    Some(Ok((TokenKind::BANG_EQUAL, None)))
                } else {
                    Some(Ok((TokenKind::BANG, None)))
                }
            }
            '=' => {
                if self.advance_if_eq('=').is_some() {
                    Some(Ok((TokenKind::EQUAL_EQUAL, None)))
                } else {
                    Some(Ok((TokenKind::EQUAL, None)))
                }
            }
            '>' => {
                if self.advance_if_eq('=').is_some() {
                    Some(Ok((TokenKind::GREATER_EQUAL, None)))
                } else {
                    Some(Ok((TokenKind::GREATER, None)))
                }
            }
            '<' => {
                if self.advance_if_eq('=').is_some() {
                    Some(Ok((TokenKind::LESS_EQUAL, None)))
                } else {
                    Some(Ok((TokenKind::LESS, None)))
                }
            }
            c => Some(Err(LoxError::spanned(
                format!("Unmatched character!: {}", c),
                self.span.clone(),
            ))),
        };
        output
    }

    /// Advance one character in the source and return it
    fn advance(&mut self) -> Option<char> {
        match self.source.next() {
            Some(v) => {
                self.span.advance();
                Some(v)
            }
            None => None,
        }
    }

    /// Thin wrapper around `Peekaboo::next_if_eq` which updates the span
    fn advance_if_eq(&mut self, expected: char) -> Option<char> {
        self.source.next_if_eq(&expected).and_then(|c| {
            self.span.advance();
            Some(c)
        })
    }

    /// Thin wrapper around `Peekaboo::next_if` which updates the span
    fn advance_if(&mut self, pred: impl FnOnce(&char) -> bool) -> Option<char> {
        self.source.next_if(pred).and_then(|c| {
            self.span.advance();
            Some(c)
        })
    }

    /// Create a token from a matched TokenKind and optional literal value.
    fn make_token(&mut self, kind: TokenKind, literal: Option<LiteralValue>) -> Token {
        Token {
            kind,
            literal,
            lexeme: kind.to_string(),
            span: self.span.clone(),
        }
    }

    /// Just a wrapper around eprintln! for now :)
    fn report_non_fatal_error(&mut self, error: LoxError) {
        eprintln!("{}", error);
    }

    /// Ignores the rest of the line
    fn line_comment(&mut self) {
        while self.source.next_if(|&c| c != '\n').is_some() {}
    }

    /// Processes a string token
    fn string(&mut self) -> MaybeTokenArgs {
        let mut string = String::new();

        while let Some(c) = self.advance_if(|c| *c != '"') {
            let next_string_char = match c {
                '\n' => {
                    self.span.newline();
                    c
                }
                '\\' => {
                    // escape sequence
                    // check for all special characters
                    match self.escape_character() {
                        Ok(escape_character) => escape_character,
                        Err(e) => {
                            self.report_non_fatal_error(e);
                            c
                        }
                    }
                }
                c => c,
            };
            string.push(next_string_char);
        }
        if self.advance_if_eq('"').is_none() {
            return Some(Err(LoxError::unterminated_string(
                r#"Syntax Error: Unterminated string. Expected closing `"`"#,
                self.span.clone(),
                string,
            )));
        }
        Some(Ok((TokenKind::STRING, Some(LiteralValue::String(string)))))
    }

    fn escape_character(&mut self) -> Result<char, LoxError> {
        if self.advance_if_eq('n').is_some() {
            Ok('\n')
        } else if self.advance_if_eq('t').is_some() {
            Ok('\t')
        } else if self.advance_if_eq('r').is_some() {
            Ok('\r')
        } else if self.advance_if_eq('\\').is_some() {
            Ok('\\')
        } else if self.advance_if_eq('"').is_some() {
            Ok('"')
        } else {
            Err(LoxError::spanned(
                r#"Syntax Error: Invalid Escape. Expected one of `\n,\t,\r,\\`"#,
                self.span.clone(),
            ))
        }
    }

    /// Processes a number token
    fn number(&mut self, previous: char) -> MaybeTokenArgs {
        let mut value = String::from(previous);

        while let Some(n) = self.advance_if(|c| c.is_ascii_digit()) {
            value.push(n);
        }

        // check if parsing a decimal
        if self.source.peek_eq(&'.') && self.source.peek_ahead_check(|c| c.is_ascii_digit()) {
            // add the decimal point
            // unwrap: We just peeked at the decimal point
            value.push(self.advance().unwrap());
            // add the rest of the digits
            while let Some(n) = self.advance_if(|c| c.is_ascii_digit()) {
                value.push(n);
            }
        }

        match value.parse() {
            Ok(number) => Some(Ok((TokenKind::NUMBER, Some(LiteralValue::Number(number))))),
            Err(_) => Some(Err(LoxError::spanned(
                "Unexpected Error while parsing Literal Number",
                self.span.clone(),
            ))),
        }
    }

    /// Processes a keyword token
    fn keyword(&mut self, previous: char) -> MaybeTokenArgs {
        let mut value = String::from(previous);

        while let Some(c) = self.advance_if(|c| c.is_ascii_alphanumeric() || *c == '_') {
            value.push(c);
        }

        Some(Ok((
            TokenKind::is_keyword(&value).unwrap_or(TokenKind::IDENTIFIER),
            Some(LiteralValue::String(value)),
        )))
    }
}

impl<'source> Iterator for TokenStream<'source> {
    type Item = Result<Token, LoxError>;

    fn next(&mut self) -> Option<Result<Token, LoxError>> {
        // We are at the beginning of the next lexeme.
        self.span.reset();
        self.scan_token()
    }
}

#[cfg(test)]
mod test {
    use crate::LoxError;

    use super::Lexer;
    use super::{Token, TokenKind};

    #[test]
    fn test() -> Result<(), LoxError> {
        let source = r#"
            (2 + 3) != "foo" + bar
        "#;

        let lexer = Lexer::new(source);

        Ok(())
    }
}
