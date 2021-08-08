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

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer { source }
    }

    pub fn scan_tokens(self) -> Result<Vec<Token>, LoxError> {
        let token_stream = TokenStream {
            source: self.source.chars().peekaboo(),
            start: 0,
            current: 0,
            line: 1,
        };
        token_stream.collect::<Result<_, _>>()
    }
}
pub struct TokenStream<'s> {
    source: Peekaboo<Chars<'s>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'s> TokenStream<'s> {
    fn scan_token(&mut self) -> Option<Result<Token, LoxError>> {
        if let Some(c) = self.advance() {
            if let Ok(Some((token, literal))) = match c {
                //
                // Non Literals
                //
                '(' => Ok(Some((TokenKind::LEFT_PAREN, None))),
                ')' => Ok(Some((TokenKind::RIGHT_PAREN, None))),
                '{' => Ok(Some((TokenKind::LEFT_BRACE, None))),
                '}' => Ok(Some((TokenKind::RIGHT_BRACE, None))),
                ',' => Ok(Some((TokenKind::COMMA, None))),
                '.' => Ok(Some((TokenKind::DOT, None))),
                '-' => Ok(Some((TokenKind::MINUS, None))),
                '+' => Ok(Some((TokenKind::PLUS, None))),
                ';' => Ok(Some((TokenKind::SEMICOLON, None))),
                '/' => {
                    if self.advance_if_eq('/') {
                        dbg!("start matching a comment");
                        self.line_comment();
                        Ok(None)
                    } else {
                        Ok(Some((TokenKind::SLASH, None)))
                    }
                }
                '*' => Ok(Some((TokenKind::STAR, None))),
                '!' => {
                    if self.advance_if_eq('=') {
                        Ok(Some((TokenKind::BANG_EQUAL, None)))
                    } else {
                        Ok(Some((TokenKind::BANG, None)))
                    }
                }
                '=' => {
                    if self.advance_if_eq('=') {
                        Ok(Some((TokenKind::EQUAL_EQUAL, None)))
                    } else {
                        Ok(Some((TokenKind::EQUAL, None)))
                    }
                }
                '>' => {
                    if self.advance_if_eq('=') {
                        Ok(Some((TokenKind::GREATER_EQUAL, None)))
                    } else {
                        Ok(Some((TokenKind::GREATER, None)))
                    }
                }
                '<' => {
                    if self.advance_if_eq('=') {
                        Ok(Some((TokenKind::LESS_EQUAL, None)))
                    } else {
                        Ok(Some((TokenKind::LESS, None)))
                    }
                }
                // ignore whitespace
                ' ' | '\t' | '\r' => Ok(None),
                // increment lines on every newline
                '\n' => {
                    self.line += 1;
                    Ok(None)
                }
                //
                // Literals
                //
                '"' => {
                    dbg!("start matching a string literal");
                    self.string()
                }
                n if n.is_ascii_digit() => {
                    dbg!("start matching a number literal");
                    self.number(n)
                }
                k if k.is_ascii_alphabetic() => {
                    dbg!("start matching a keyword or identifier literal");
                    self.keyword(k)
                }
                _ => panic!("some other character I don't know what to do with yet!"),
            } {
                Some(Ok(self.make_token(token, literal)))
            } else {
                // we consumed some lexemes but skipped making a token
                // e.g. we just lexed a comment!
                // we keep going to return a Token otherwise there will be no more tokens forever!
                self.next()
            }
        } else {
            None
        }
    }

    /// advance one character in the source and return it
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.next()
    }

    /// looks at the next character and returns whether it matches `expected`, only advancing if it does
    fn advance_if_eq(&mut self, expected: char) -> bool {
        match self.source.peek() {
            Some(c) if *c == expected => {
                // advance internal iterator
                self.source.next();
                // advance our manual track of progress
                self.current += 1;
                // we matched!
                true
            }
            // no match, so no internal mutations.
            _ => false,
        }
    }

    /// Create a token from a matched TokenKind and optional literal value.
    fn make_token(&mut self, kind: TokenKind, literal: Option<LiteralValue>) -> Token {
        Token {
            kind,
            literal,
            lexeme: kind.to_string(),
            span: self.span(),
        }
    }

    /// Make a span (location) from current state.
    fn span(&mut self) -> Span {
        Span {
            line: self.line,
            start: self.start,
            end: self.current,
        }
    }

    /// Just a wrapper around eprintln! for now :)
    fn report_non_fatal_error(&mut self, error: LoxError) {
        eprintln!("{}", error);
    }

    /// Ignores the rest of the line
    fn line_comment(&mut self) {
        while self.source.next_if(|&c| c != '\n').is_some() {
            // *Do we* gotta keep current up to date?
            // self.current += 1;
        }
    }

    /// Processes a string token
    fn string(&mut self) -> Result<Option<(TokenKind, Option<LiteralValue>)>, LoxError> {
        // track this to error on unclosed strings
        let mut string_terminated = false;
        let mut string = String::new();

        while let Some(&c) = self.source.peek() {
            if c == '"' {
                string_terminated = true;
                break;
            } else {
                let next_string_char = match c {
                    '\n' => {
                        self.line += 1;
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

            // advance internal iterator
            self.source.next();
            // advance our manual track of progress
            self.current += 1;
        }
        if !string_terminated {
            return Err(LoxError::new(
                r#"Syntax Error: Unterminated string. Expected closing `"`"#,
                self.span(),
            ));
        }
        Ok(Some((
            TokenKind::STRING,
            Some(LiteralValue::String(string)),
        )))
    }

    fn escape_character(&mut self) -> Result<char, LoxError> {
        if self.advance_if_eq('n') {
            self.current += 1;
            Ok('\n')
        } else if self.advance_if_eq('t') {
            self.current += 1;
            Ok('\t')
        } else if self.advance_if_eq('r') {
            self.current += 1;
            Ok('\r')
        } else if self.advance_if_eq('\\') {
            self.current += 1;
            Ok('\\')
        } else {
            Err(LoxError::new(
                r#"Syntax Error: Invalid Escape. Expected one of `\n,\t,\r,\\`"#,
                self.span(),
            ))
        }
    }

    /// Processes a number token
    fn number(
        &mut self,
        first_digit: char,
    ) -> Result<Option<(TokenKind, Option<LiteralValue>)>, LoxError> {
        let mut value = first_digit.to_string();

        while let Some(n) = self.source.next_if(|c| c.is_ascii_digit()) {
            self.current += 1;
            value.push(n);
        }

        if let Some(('.', first_decimal_digit)) =
            self.source.next_and_after_if(|c| c.is_ascii_digit())
        {
            // parsing a decimal
            value.push(first_decimal_digit);
            self.current += 2;

            while let Some(n) = self.source.next_if(|c| c.is_ascii_digit()) {
                self.current += 1;
                value.push(n);
            }
        }

        let number = value.parse().map_err(|_| {
            LoxError::new("Unexpected Error while parsing Literal Number", self.span())
        })?;

        Ok(Some((
            TokenKind::NUMBER,
            Some(LiteralValue::Number(number)),
        )))
    }

    /// Processes a keyword token
    fn keyword(
        &mut self,
        first_char: char,
    ) -> Result<Option<(TokenKind, Option<LiteralValue>)>, LoxError> {
        let mut value = first_char.to_string();

        while let Some(n) = self.source.next_if(|c| c.is_ascii_alphanumeric()) {
            self.current += 1;
            value.push(n);
        }

        Ok(Some((
            TokenKind::is_keyword(value).unwrap_or(TokenKind::IDENTIFIER),
            None,
        )))
    }
}

impl<'s> Iterator for TokenStream<'s> {
    type Item = Result<Token, LoxError>;

    fn next(&mut self) -> Option<Result<Token, LoxError>> {
        // We are at the beginning of the next lexeme.
        self.start = self.current;
        self.scan_token()
    }
}
