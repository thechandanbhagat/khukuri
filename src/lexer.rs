use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Lexer {
    code: Vec<char>,
    pos: usize,
    current_char: Option<char>,
    line: usize,
    column: usize,
    keywords: HashMap<String, String>,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        let chars: Vec<char> = code.chars().collect();
        let current_char = if chars.is_empty() { None } else { Some(chars[0]) };
        
        let mut keywords = HashMap::new();
        // Nepali keywords
        keywords.insert("maanau".to_string(), "maanau".to_string());      // Variable declaration
        keywords.insert("yedi".to_string(), "yedi".to_string());          // If
        keywords.insert("bhane".to_string(), "bhane".to_string());        // Then
        keywords.insert("natra".to_string(), "natra".to_string());        // Else
        keywords.insert("jaba".to_string(), "jaba".to_string());          // While (part 1)
        keywords.insert("samma".to_string(), "samma".to_string());        // While (part 2)
        keywords.insert("pratyek".to_string(), "pratyek".to_string());    // For each
        keywords.insert("ma".to_string(), "ma".to_string());              // In (for foreach)
        keywords.insert("kaam".to_string(), "kaam".to_string());          // Function
        keywords.insert("pathau".to_string(), "pathau".to_string());      // Return
        keywords.insert("bhan".to_string(), "bhan".to_string());          // Print
        keywords.insert("sodha".to_string(), "sodha".to_string());        // Input
        keywords.insert("rok".to_string(), "rok".to_string());            // Break
        keywords.insert("jane".to_string(), "jane".to_string());          // Continue
        keywords.insert("ra".to_string(), "ra".to_string());              // And
        keywords.insert("wa".to_string(), "wa".to_string());              // Or
        keywords.insert("hoina".to_string(), "hoina".to_string());        // Not
        keywords.insert("sahi".to_string(), "sahi".to_string());          // True
        keywords.insert("galat".to_string(), "galat".to_string());        // False
        keywords.insert("aayaat".to_string(), "aayaat".to_string());      // Import
        
        Lexer {
            code: chars,
            pos: 0,
            current_char,
            line: 1,
            column: 1,
            keywords,
        }
    }
    
    fn advance(&mut self) {
        if let Some('\n') = self.current_char {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        
        self.pos += 1;
        if self.pos >= self.code.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.code[self.pos]);
        }
    }
    
    fn peek(&self) -> Option<char> {
        let peek_pos = self.pos + 1;
        if peek_pos >= self.code.len() {
            None
        } else {
            Some(self.code[peek_pos])
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) {
        // Skip single-line comments starting with //
        if self.current_char == Some('/') && self.peek() == Some('/') {
            while let Some(ch) = self.current_char {
                if ch == '\n' {
                    break;
                }
                self.advance();
            }
        }
    }
    
    fn read_number(&mut self) -> String {
        let mut number = String::new();
        let mut has_dot = false;
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        number
    }
    
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        identifier
    }
    
    fn read_string(&mut self) -> Result<String, String> {
        let mut string = String::new();
        self.advance(); // Skip opening quote
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // Skip closing quote
                return Ok(string);
            } else if ch == '\\' {
                // Handle escape sequences
                self.advance();
                match self.current_char {
                    Some('n') => string.push('\n'),
                    Some('t') => string.push('\t'),
                    Some('r') => string.push('\r'),
                    Some('\\') => string.push('\\'),
                    Some('"') => string.push('"'),
                    Some(c) => string.push(c),
                    None => return Err("Unterminated string literal".to_string()),
                }
                self.advance();
            } else if ch == '\n' {
                return Err("Unterminated string literal".to_string());
            } else {
                string.push(ch);
                self.advance();
            }
        }
        
        Err("Unterminated string literal".to_string())
    }
    
    fn read_operator(&mut self) -> String {
        let mut operator = String::new();
        
        match self.current_char {
            Some('=') => {
                operator.push('=');
                self.advance();
                if self.current_char == Some('=') {
                    operator.push('=');
                    self.advance();
                }
            }
            Some('!') => {
                operator.push('!');
                self.advance();
                if self.current_char == Some('=') {
                    operator.push('=');
                    self.advance();
                }
            }
            Some('>') => {
                operator.push('>');
                self.advance();
                if self.current_char == Some('=') {
                    operator.push('=');
                    self.advance();
                }
            }
            Some('<') => {
                operator.push('<');
                self.advance();
                if self.current_char == Some('=') {
                    operator.push('=');
                    self.advance();
                }
            }
            Some(ch @ ('+' | '-' | '*' | '/' | '%')) => {
                operator.push(ch);
                self.advance();
            }
            _ => {}
        }
        
        operator
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while let Some(ch) = self.current_char {
            let token_line = self.line;
            let token_column = self.column;
            
            match ch {
                // Skip whitespace (except newlines)
                ' ' | '\t' | '\r' => {
                    self.skip_whitespace();
                }
                
                // Handle newlines
                '\n' => {
                    tokens.push(Token::new(
                        TokenType::Newline,
                        "\n".to_string(),
                        token_line,
                        token_column,
                    ));
                    self.advance();
                }
                
                // Handle comments
                '/' if self.peek() == Some('/') => {
                    self.skip_comment();
                }
                
                // Handle strings
                '"' => {
                    let string_value = self.read_string()?;
                    tokens.push(Token::new(
                        TokenType::String,
                        string_value,
                        token_line,
                        token_column,
                    ));
                }
                
                // Handle numbers
                ch if ch.is_ascii_digit() => {
                    let number = self.read_number();
                    tokens.push(Token::new(
                        TokenType::Number,
                        number,
                        token_line,
                        token_column,
                    ));
                }
                
                // Handle identifiers and keywords
                ch if ch.is_alphabetic() || ch == '_' => {
                    let identifier = self.read_identifier();
                    let token_type = if self.keywords.contains_key(&identifier) {
                        TokenType::Keyword
                    } else {
                        TokenType::Identifier
                    };
                    
                    tokens.push(Token::new(
                        token_type,
                        identifier,
                        token_line,
                        token_column,
                    ));
                }
                
                // Handle operators
                '=' | '!' | '>' | '<' | '+' | '-' | '*' | '/' | '%' => {
                    let operator = self.read_operator();
                    tokens.push(Token::new(
                        TokenType::Operator,
                        operator,
                        token_line,
                        token_column,
                    ));
                }
                
                // Handle delimiters
                '{' => {
                    tokens.push(Token::new(
                        TokenType::LBrace,
                        "{".to_string(),
                        token_line,
                        token_column,
                    ));
                    self.advance();
                }
                '}' => {
                    tokens.push(Token::new(
                        TokenType::RBrace,
                        "}".to_string(),
                        token_line,
                        token_column,
                    ));
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::new(
                        TokenType::LParen,
                        "(".to_string(),
                        token_line,
                        token_column,
                    ));
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::new(
                        TokenType::RParen,
                        ")".to_string(),
                        token_line,
                        token_column,
                    ));
                    self.advance();
                }
                '[' => {
                    tokens.push(Token::new(
                        TokenType::LBracket,
                        "[".to_string(),
                        token_line,
                        token_column,
                    ));
                    self.advance();
                }
                ']' => {
                    tokens.push(Token::new(
                        TokenType::RBracket,
                        "]".to_string(),
                        token_line,
                        token_column,
                    ));
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::new(
                        TokenType::Comma,
                        ",".to_string(),
                        token_line,
                        token_column,
                    ));
                    self.advance();
                }
                ':' => {
                    tokens.push(Token::new(
                        TokenType::Colon,
                        ":".to_string(),
                        token_line,
                        token_column,
                    ));
                    self.advance();
                }
                
                // Handle unexpected characters
                _ => {
                    return Err(format!(
                        "Unexpected character '{}' at line {}, column {}",
                        ch, token_line, token_column
                    ));
                }
            }
        }
        
        // Add EOF token
        tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            self.line,
            self.column,
        ));
        
        Ok(tokens)
    }
}