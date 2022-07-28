#[allow(dead_code)]
pub enum Token {
    Illegal,
    EndOfFile,

    // Identifiers + Literals
    Identifier,
    Integer,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    LeftParantheses,
    RightParantheses,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
}

