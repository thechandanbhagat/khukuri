#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Keyword,           // maanau, yedi, bhane, natra, etc.
    
    // Identifiers and Literals
    Identifier,        // variable names
    Number,           // 123, 45.67
    String,           // "text"
    
    // Operators
    Operator,         // =, +, -, *, /, %, ==, !=, >, <, >=, <=
    
    // Delimiters
    LBrace,           // {
    RBrace,           // }
    LParen,           // (
    RParen,           // )
    LBracket,         // [
    RBracket,         // ]
    Comma,            // ,
    Colon,            // : (for optional type hints)
    
    // Special
    Newline,          // \n
    EOF,              // End of file
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            value,
            line,
            column,
        }
    }
}