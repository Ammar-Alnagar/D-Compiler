#[derive(Debug, Clone, PartialEq)]
pub enum Punctuation {
    OpenParen = 0,     // (
    CloseParen = 1,    // )
    OpenBrace = 2,     // {
    CloseBrace = 3,    // }
    OpenBracket = 4,   // [
    CloseBracket = 5,  // ]
    Comma = 6,         // ,
    Semicolon = 7,     // ;
    Dot = 8,           // .
    Colon = 9,         // :
    QuestionMark = 10, // ?
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add,          // +
    Subtract,     // -
    Multiply,     // *
    Divide,       // /
    Assign,       // =
    IfEqual,      // ==
    NotEqual,     // !=
    Greater,      // >
    Less,         // <
    GreaterEqual, // >=
    LessEqual,    // <=
    Not,          // !
}

#[derive(Debug, Clone, PartialEq)]
pub enum Reserved {
    Null,
    Void,
    Let,
    Fn,
    If,
    Else,
    While,
    For,
    Continue,
    Break,
    Return,
    Public,
    Private,
    Static,
    Print,
    True,
    False,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(f64),
    String(String),
    Reserved(Reserved),
    Operation(Operation),
    Punctuation(Punctuation),
    Whitespace,
    Newline,
    Eof,
    Invalid(String),
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    #[allow(dead_code)]
    pub lexeme: String,
    #[allow(dead_code)]
    pub line: usize,
    #[allow(dead_code)]
    pub column: usize,
}

impl TokenInfo {
    pub fn new(token: Token, lexeme: String, line: usize, column: usize) -> Self {
        Self {
            token,
            lexeme,
            line,
            column,
        }
    }
}
