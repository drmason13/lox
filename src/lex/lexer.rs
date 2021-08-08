use std::str::Chars;

// Peekaboo is for double peeking
use peekaboo::{IteratorPeekabooExt, Peekaboo};

use crate::LoxError;

use super::{
    token::{LiteralValue, Token, TokenKind},
    Span,
};

pub struct Lexer {
    source: String,
}

impl<'s> Lexer {
    pub fn new(source: String) -> Self {
        Lexer { source }
    }

    pub fn scan_tokens(&'s self) -> TokenStream<'s> {
        TokenStream {
            source: self.source.chars().peekaboo(),
            span: Span::new(),
        }
    }
}

/// TokenStream is a stream of tokens interpreted from individual characters of a source String
///
/// TokenSteam implements Iterator (but not Stream...)
///
/// The Iterator Item is `Result<Token, LoxError>` which allows the caller can decide how to
/// handle individual invalid tokens. They may choose to error immediately or keep going
/// and report errors once the TokenStream has ended.
pub struct TokenStream<'s> {
    source: Peekaboo<Chars<'s>>,
    span: Span,
}

type MaybeTokenArgs = Option<Result<(TokenKind, Option<LiteralValue>), LoxError>>;

impl<'s> TokenStream<'s> {
    /// This is the function
    fn scan_token(&mut self) -> Option<Result<Token, LoxError>> {
        if let Some(c) = self.source.peek().copied() {
            match self.match_char(&c) {
                Some(Err(e)) => {
                    self.advance();
                    Some(Err(e))
                }
                Some(Ok((token_kind, literal_value))) => {
                    Some(Ok(self.make_token(token_kind, literal_value)))
                }
                None => {
                    // we consumed some lexemes but skipped making a token
                    // e.g. we just lexed a comment!
                    // we keep going to return a Token otherwise there will be no more tokens forever!

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
    fn match_char(&mut self, peeked_char: &char) -> MaybeTokenArgs {
        match peeked_char {
            // ignore whitespace
            ' ' | '\t' | '\r' => None,
            // increment lines on every newline
            '\n' => {
                self.span.newline();
                None
            }
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if (c.is_ascii_alphabetic() || *c == '_') => self.keyword(),
            c => self.operator(c),
        }
    }

    fn operator(&mut self, c: &char) -> MaybeTokenArgs {
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
                if self.source.peek_ahead_eq(&'/') {
                    self.line_comment();
                    None
                } else {
                    Some(Ok((TokenKind::SLASH, None)))
                }
            }
            '!' => {
                if self.source.peek_ahead_eq(&'=') {
                    self.advance();
                    Some(Ok((TokenKind::BANG_EQUAL, None)))
                } else {
                    Some(Ok((TokenKind::BANG, None)))
                }
            }
            '=' => {
                if self.source.peek_ahead_eq(&'=') {
                    self.advance();
                    Some(Ok((TokenKind::EQUAL_EQUAL, None)))
                } else {
                    Some(Ok((TokenKind::EQUAL, None)))
                }
            }
            '>' => {
                if self.source.peek_ahead_eq(&'=') {
                    self.advance(); // advance an extra time since we just matched another character
                    Some(Ok((TokenKind::GREATER_EQUAL, None)))
                } else {
                    Some(Ok((TokenKind::GREATER, None)))
                }
            }
            '<' => {
                if self.source.peek_ahead_eq(&'=') {
                    self.advance();
                    Some(Ok((TokenKind::LESS_EQUAL, None)))
                } else {
                    Some(Ok((TokenKind::LESS, None)))
                }
            }
            c => Some(Err(LoxError::new(
                format!("Unmatched character!: {}", c),
                self.span.clone(),
            ))),
        };
        // here we advance to catch up with the first peek we did all the way back in scan_token (peeked_char)
        self.advance();
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

    /// Advances one character in the source if the next character matches `expected`, returns true if we advanced
    fn advance_if_eq(&mut self, expected: char) -> bool {
        if self.source.next_if_eq(&expected).is_some() {
            self.span.advance();
            true
        } else {
            false
        }
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
        // skip past the opening quote
        self.advance();
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
        if !self.advance_if_eq('"') {
            return Some(Err(LoxError::unterminated_string(
                r#"Syntax Error: Unterminated string. Expected closing `"`"#,
                self.span.clone(),
                string,
            )));
        }
        Some(Ok((TokenKind::STRING, Some(LiteralValue::String(string)))))
    }

    fn escape_character(&mut self) -> Result<char, LoxError> {
        if self.advance_if_eq('n') {
            self.span.advance();
            Ok('\n')
        } else if self.advance_if_eq('t') {
            self.span.advance();
            Ok('\t')
        } else if self.advance_if_eq('r') {
            self.span.advance();
            Ok('\r')
        } else if self.advance_if_eq('\\') {
            self.span.advance();
            Ok('\\')
        } else if self.advance_if_eq('"') {
            self.span.advance();
            Ok('"')
        } else {
            Err(LoxError::new(
                r#"Syntax Error: Invalid Escape. Expected one of `\n,\t,\r,\\`"#,
                self.span.clone(),
            ))
        }
    }

    /// Processes a number token
    fn number(&mut self) -> MaybeTokenArgs {
        let mut value = String::new();

        while let Some(n) = self.advance_if(|c| c.is_ascii_digit()) {
            value.push(n);
        }

        // check if parsing a decimal
        if self.source.peek_eq(&'.') && self.source.peek_ahead_check(|c| c.is_ascii_digit()) {
            self.advance();

            while let Some(n) = self.advance_if(|c| c.is_ascii_digit()) {
                value.push(n);
            }
        }

        match value.parse() {
            Ok(number) => Some(Ok((TokenKind::NUMBER, Some(LiteralValue::Number(number))))),
            Err(_) => Some(Err(LoxError::new(
                "Unexpected Error while parsing Literal Number",
                self.span.clone(),
            ))),
        }
    }

    /// Processes a keyword token
    fn keyword(&mut self) -> MaybeTokenArgs {
        let mut value = String::new();

        while let Some(c) = self.advance_if(|c| c.is_ascii_alphanumeric() || *c == '_') {
            value.push(c);
        }

        Some(Ok((
            TokenKind::is_keyword(&value).unwrap_or(TokenKind::IDENTIFIER),
            Some(LiteralValue::String(value)),
        )))
    }

    /// Thin wrapper around `Peekaboo::next_if` which updates the span
    fn advance_if(&mut self, pred: impl FnOnce(&char) -> bool) -> Option<char> {
        self.source.next_if(pred).and_then(|c| {
            self.span.advance();
            Some(c)
        })
    }
}

impl<'s> Iterator for TokenStream<'s> {
    type Item = Result<Token, LoxError>;

    fn next(&mut self) -> Option<Result<Token, LoxError>> {
        // We are at the beginning of the next lexeme.
        self.span.reset();
        self.scan_token()
    }
}
