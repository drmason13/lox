#[derive(Clone, Debug)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<LiteralValue>,
    pub(crate) span: Span,
}

#[derive(Clone, Debug)]
pub enum LiteralValue {
    Number(f32),
    String(String),
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "{}", s),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.literal {
            Some(l) => write!(f, "{} {}: {}", self.span, self.kind, l),
            None => write!(f, "{} {}", self.span, self.kind),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum TokenKind {
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
    pub fn is_keyword(s: String) -> Option<Self> {
        match s.as_str() {
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
                TokenKind::EOF => "EOF",
            }
        )
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Span {
    pub(crate) line: usize,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ line: {}, start: {}, end: {} }}",
            self.line, self.start, self.end
        )
    }
}
