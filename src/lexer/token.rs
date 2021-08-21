use crate::ast::LiteralValue;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub span: Span,
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "{}", escape_string(s)),
            LiteralValue::Bool(true) => write!(f, "true"),
            LiteralValue::Bool(false) => write!(f, "false"),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}

/// escape a string so we can print it nicely
pub fn escape_string(s: &str) -> String {
    let mut source = s.chars();
    let quote_char = '"';

    let mut string = String::from(quote_char);

    while let Some(c) = source.next() {
        match c {
            '\n' => {
                string.push('\\');
                string.push('n');
            }
            '\\' => {
                string.push('\\');
                string.push('\\');
            }
            c if c == quote_char => {
                string.push('\\');
                string.push(quote_char);
            }
            c => {
                string.push(c);
            }
        };
    }
    string.push(quote_char);
    string
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.literal {
            Some(l) => write!(f, "[{}] {}: {}", self.span, self.kind, l),
            None => write!(f, "[{}] {}", self.span, self.kind),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TokenKind {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    #[allow(dead_code)]
    EOF,
}

impl TokenKind {
    /// Some characters map 1:1 with tokens
    pub fn is_keyword(s: impl AsRef<str>) -> Option<Self> {
        match s.as_ref() {
            "and" => Some(TokenKind::AND),
            "class" => Some(TokenKind::CLASS),
            "else" => Some(TokenKind::ELSE),
            "false" => Some(TokenKind::FALSE),
            "fun" => Some(TokenKind::FUN),
            "for" => Some(TokenKind::FOR),
            "if" => Some(TokenKind::IF),
            "nil" => Some(TokenKind::NIL),
            "or" => Some(TokenKind::OR),
            "print" => Some(TokenKind::PRINT),
            "return" => Some(TokenKind::RETURN),
            "super" => Some(TokenKind::SUPER),
            "this" => Some(TokenKind::THIS),
            "true" => Some(TokenKind::TRUE),
            "var" => Some(TokenKind::VAR),
            "while" => Some(TokenKind::WHILE),
            _ => None,
        }
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenKind::LEFT_PAREN => "(",
                TokenKind::RIGHT_PAREN => ")",
                TokenKind::LEFT_BRACE => "{",
                TokenKind::RIGHT_BRACE => "}",
                TokenKind::COMMA => ",",
                TokenKind::DOT => ".",
                TokenKind::MINUS => "-",
                TokenKind::PLUS => "+",
                TokenKind::SEMICOLON => ";",
                TokenKind::SLASH => "/",
                TokenKind::STAR => "*",
                TokenKind::BANG => "!",
                TokenKind::BANG_EQUAL => "!=",
                TokenKind::EQUAL => "=",
                TokenKind::EQUAL_EQUAL => "==",
                TokenKind::GREATER => ">",
                TokenKind::GREATER_EQUAL => ">=",
                TokenKind::LESS => "<",
                TokenKind::LESS_EQUAL => "<=",
                TokenKind::IDENTIFIER => "Ident",
                TokenKind::STRING => "string",
                TokenKind::NUMBER => "number",
                TokenKind::AND => "and",
                TokenKind::CLASS => "class",
                TokenKind::ELSE => "else",
                TokenKind::FALSE => "false",
                TokenKind::FUN => "fun",
                TokenKind::FOR => "for",
                TokenKind::IF => "if",
                TokenKind::NIL => "()",
                TokenKind::OR => "or",
                TokenKind::PRINT => "print",
                TokenKind::RETURN => "return",
                TokenKind::SUPER => "super",
                TokenKind::THIS => "this",
                TokenKind::TRUE => "true",
                TokenKind::VAR => "var",
                TokenKind::WHILE => "while",
                TokenKind::EOF => "<EOF>",
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Span {
    pub(crate) start_line: usize,
    pub(crate) end_line: usize,
    pub(crate) start_character: usize,
    pub(crate) end_character: usize,
}

impl Span {
    pub fn new() -> Self {
        Span {
            start_line: 1,
            start_character: 0,
            end_line: 1,
            end_character: 0,
        }
    }

    pub fn advance(&mut self) {
        self.end_character += 1;
    }

    pub fn newline(&mut self) {
        self.end_line += 1;
        self.start_character = 0;
        self.end_character = 0;
    }

    pub fn reset(&mut self) {
        self.start_character = self.end_character;
        self.start_line = self.end_line;
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (
            self.end_line == self.start_line,
            self.end_character == self.start_character + 1,
        ) {
            // single line, single char
            (true, true) => write!(f, "{}:{}", self.start_line, self.start_character),
            // single line, multiple chars
            (true, false) => write!(
                f,
                "{}:{}-{}",
                self.start_line,
                self.start_character,
                self.end_character.checked_sub(1).unwrap_or(0),
            ),
            // multiple lines, single char
            (false, true) => write!(
                f,
                "{}-{}:{}",
                self.start_line, self.end_line, self.start_character
            ),
            // multiple lines, multiple chars
            (false, false) => write!(
                f,
                "{}-{}:{}-{}",
                self.start_line,
                self.end_line,
                self.start_character,
                self.end_character.checked_sub(1).unwrap_or(0)
            ),
        }
    }
}
