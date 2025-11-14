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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_empty_input() {
        let mut lexer = Lexer::new("".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::EOF);
    }

    #[test]
    fn test_tokenize_single_keyword() {
        let mut lexer = Lexer::new("maanau".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // keyword + EOF
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        assert_eq!(tokens[0].value, "maanau");
    }

    #[test]
    fn test_tokenize_multiple_keywords() {
        let mut lexer = Lexer::new("maanau yedi bhane natra".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5); // 4 keywords + EOF
        assert_eq!(tokens[0].value, "maanau");
        assert_eq!(tokens[1].value, "yedi");
        assert_eq!(tokens[2].value, "bhane");
        assert_eq!(tokens[3].value, "natra");
    }

    #[test]
    fn test_tokenize_integer() {
        let mut lexer = Lexer::new("42".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "42");
    }

    #[test]
    fn test_tokenize_float() {
        let mut lexer = Lexer::new("3.14".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_tokenize_float_with_trailing_dot() {
        let mut lexer = Lexer::new("5.".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5.");
    }

    #[test]
    fn test_tokenize_multiple_numbers() {
        let mut lexer = Lexer::new("10 20.5 30".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "10");
        assert_eq!(tokens[1].value, "20.5");
        assert_eq!(tokens[2].value, "30");
    }

    #[test]
    fn test_tokenize_simple_string() {
        let mut lexer = Lexer::new("\"hello\"".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "hello");
    }

    #[test]
    fn test_tokenize_string_with_newline_escape() {
        let mut lexer = Lexer::new(r#""hello\nworld""#.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "hello\nworld");
    }

    #[test]
    fn test_tokenize_string_with_tab_escape() {
        let mut lexer = Lexer::new(r#""hello\tworld""#.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "hello\tworld");
    }

    #[test]
    fn test_tokenize_string_with_backslash_escape() {
        let mut lexer = Lexer::new(r#""path\\to\\file""#.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "path\\to\\file");
    }

    #[test]
    fn test_tokenize_string_with_quote_escape() {
        let mut lexer = Lexer::new(r#""say \"hello\"""#.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "say \"hello\"");
    }

    #[test]
    fn test_tokenize_string_all_escapes() {
        let mut lexer = Lexer::new(r#""line1\nline2\ttab\r\n\"quote\"""#.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "line1\nline2\ttab\r\n\"quote\"");
    }

    #[test]
    fn test_tokenize_unterminated_string() {
        let mut lexer = Lexer::new("\"unterminated".to_string());
        let result = lexer.tokenize();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unterminated string"));
    }

    #[test]
    fn test_tokenize_string_with_newline() {
        let mut lexer = Lexer::new("\"hello\nworld\"".to_string());
        let result = lexer.tokenize();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unterminated string"));
    }

    #[test]
    fn test_tokenize_empty_string() {
        let mut lexer = Lexer::new("\"\"".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "");
    }

    #[test]
    fn test_tokenize_identifier() {
        let mut lexer = Lexer::new("myVar".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "myVar");
    }

    #[test]
    fn test_tokenize_identifier_with_underscore() {
        let mut lexer = Lexer::new("my_var_123".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "my_var_123");
    }

    #[test]
    fn test_tokenize_single_char_operators() {
        let mut lexer = Lexer::new("+ - * / %".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "+");
        assert_eq!(tokens[1].value, "-");
        assert_eq!(tokens[2].value, "*");
        assert_eq!(tokens[3].value, "/");
        assert_eq!(tokens[4].value, "%");
    }

    #[test]
    fn test_tokenize_comparison_operators() {
        let mut lexer = Lexer::new("== != >= <=".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "==");
        assert_eq!(tokens[1].value, "!=");
        assert_eq!(tokens[2].value, ">=");
        assert_eq!(tokens[3].value, "<=");
    }

    #[test]
    fn test_tokenize_assignment_operator() {
        let mut lexer = Lexer::new("x = 5".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "x");
        assert_eq!(tokens[1].value, "=");
        assert_eq!(tokens[2].value, "5");
    }

    #[test]
    fn test_tokenize_greater_and_less_than() {
        let mut lexer = Lexer::new("> <".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, ">");
        assert_eq!(tokens[1].value, "<");
    }

    #[test]
    fn test_tokenize_delimiters() {
        let mut lexer = Lexer::new("{ } ( ) [ ] , :".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::LBrace);
        assert_eq!(tokens[1].token_type, TokenType::RBrace);
        assert_eq!(tokens[2].token_type, TokenType::LParen);
        assert_eq!(tokens[3].token_type, TokenType::RParen);
        assert_eq!(tokens[4].token_type, TokenType::LBracket);
        assert_eq!(tokens[5].token_type, TokenType::RBracket);
        assert_eq!(tokens[6].token_type, TokenType::Comma);
        assert_eq!(tokens[7].token_type, TokenType::Colon);
    }

    #[test]
    fn test_tokenize_newline() {
        let mut lexer = Lexer::new("maanau\nx".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "maanau");
        assert_eq!(tokens[1].token_type, TokenType::Newline);
        assert_eq!(tokens[2].value, "x");
    }

    #[test]
    fn test_line_tracking() {
        let mut lexer = Lexer::new("maanau\nx = 5".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line, 1); // maanau
        assert_eq!(tokens[1].line, 1); // newline
        assert_eq!(tokens[2].line, 2); // x
    }

    #[test]
    fn test_column_tracking() {
        let mut lexer = Lexer::new("abc def".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].column, 1); // abc starts at column 1
        assert_eq!(tokens[1].column, 5); // def starts at column 5
    }

    #[test]
    fn test_skip_single_line_comment() {
        let mut lexer = Lexer::new("// comment\nmaanau".to_string());
        let tokens = lexer.tokenize().unwrap();
        // Should have: newline, maanau, EOF
        assert_eq!(tokens[0].token_type, TokenType::Newline);
        assert_eq!(tokens[1].value, "maanau");
    }

    #[test]
    fn test_comment_at_end_of_file() {
        let mut lexer = Lexer::new("maanau // comment".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "maanau");
        assert_eq!(tokens[1].token_type, TokenType::EOF);
    }

    #[test]
    fn test_comment_on_own_line() {
        let mut lexer = Lexer::new("maanau\n// this is a comment\nx = 5".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "maanau");
        // Should skip comment, have newlines and then x = 5
        assert!(tokens.iter().all(|t| t.value != "comment"));
    }

    #[test]
    fn test_invalid_character() {
        let mut lexer = Lexer::new("@".to_string());
        let result = lexer.tokenize();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character"));
    }

    #[test]
    fn test_invalid_character_in_code() {
        let mut lexer = Lexer::new("maanau x @ 5".to_string());
        let result = lexer.tokenize();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character"));
    }

    #[test]
    fn test_whitespace_only() {
        let mut lexer = Lexer::new("   \t   ".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1); // Only EOF
        assert_eq!(tokens[0].token_type, TokenType::EOF);
    }

    #[test]
    fn test_mixed_whitespace() {
        let mut lexer = Lexer::new("  \t  maanau  \t  x  ".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "maanau");
        assert_eq!(tokens[1].value, "x");
    }

    #[test]
    fn test_all_nepali_keywords() {
        let keywords = vec![
            "maanau", "yedi", "bhane", "natra", "jaba", "samma",
            "pratyek", "ma", "kaam", "pathau", "bhan", "sodha",
            "rok", "jane", "ra", "wa", "hoina", "sahi", "galat", "aayaat"
        ];

        for keyword in keywords {
            let mut lexer = Lexer::new(keyword.to_string());
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens[0].token_type, TokenType::Keyword);
            assert_eq!(tokens[0].value, keyword);
        }
    }

    #[test]
    fn test_boolean_keywords() {
        let mut lexer = Lexer::new("sahi galat".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "sahi");
        assert_eq!(tokens[1].value, "galat");
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        assert_eq!(tokens[1].token_type, TokenType::Keyword);
    }

    #[test]
    fn test_complex_expression() {
        let mut lexer = Lexer::new("maanau x = (10 + 20) * 3".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "maanau");
        assert_eq!(tokens[1].value, "x");
        assert_eq!(tokens[2].value, "=");
        assert_eq!(tokens[3].token_type, TokenType::LParen);
        assert_eq!(tokens[4].value, "10");
        assert_eq!(tokens[5].value, "+");
        assert_eq!(tokens[6].value, "20");
        assert_eq!(tokens[7].token_type, TokenType::RParen);
        assert_eq!(tokens[8].value, "*");
        assert_eq!(tokens[9].value, "3");
    }

    #[test]
    fn test_list_literal() {
        let mut lexer = Lexer::new("[1, 2, 3]".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::LBracket);
        assert_eq!(tokens[1].value, "1");
        assert_eq!(tokens[2].token_type, TokenType::Comma);
        assert_eq!(tokens[3].value, "2");
        assert_eq!(tokens[4].token_type, TokenType::Comma);
        assert_eq!(tokens[5].value, "3");
        assert_eq!(tokens[6].token_type, TokenType::RBracket);
    }

    #[test]
    fn test_dictionary_literal() {
        let mut lexer = Lexer::new(r#"{"key": "value"}"#.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::LBrace);
        assert_eq!(tokens[1].token_type, TokenType::String);
        assert_eq!(tokens[1].value, "key");
        assert_eq!(tokens[2].token_type, TokenType::Colon);
        assert_eq!(tokens[3].token_type, TokenType::String);
        assert_eq!(tokens[3].value, "value");
        assert_eq!(tokens[4].token_type, TokenType::RBrace);
    }

    #[test]
    fn test_function_declaration() {
        let mut lexer = Lexer::new("kaam add(a, b) { pathau a + b }".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "kaam");
        assert_eq!(tokens[1].value, "add");
        assert_eq!(tokens[2].token_type, TokenType::LParen);
        assert_eq!(tokens[3].value, "a");
        assert_eq!(tokens[4].token_type, TokenType::Comma);
        assert_eq!(tokens[5].value, "b");
        assert_eq!(tokens[6].token_type, TokenType::RParen);
    }

    #[test]
    fn test_if_statement() {
        let mut lexer = Lexer::new("yedi x == 5 bhane { bhan x }".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "yedi");
        assert_eq!(tokens[1].value, "x");
        assert_eq!(tokens[2].value, "==");
        assert_eq!(tokens[3].value, "5");
        assert_eq!(tokens[4].value, "bhane");
    }

    #[test]
    fn test_while_loop() {
        let mut lexer = Lexer::new("jaba samma x < 10".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "jaba");
        assert_eq!(tokens[1].value, "samma");
        assert_eq!(tokens[2].value, "x");
        assert_eq!(tokens[3].value, "<");
        assert_eq!(tokens[4].value, "10");
    }

    #[test]
    fn test_division_operator_not_confused_with_comment() {
        let mut lexer = Lexer::new("10 / 2".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "10");
        assert_eq!(tokens[1].value, "/");
        assert_eq!(tokens[2].value, "2");
    }

    #[test]
    fn test_multiple_dots_in_number() {
        // This tests edge case - lexer will tokenize it, parser should catch it
        let mut lexer = Lexer::new("1.2.3".to_string());
        let tokens = lexer.tokenize().unwrap();
        // Should tokenize as "1.2" (number) then ".3" (invalid or another number)
        assert_eq!(tokens[0].value, "1.2");
    }

    #[test]
    fn test_unicode_in_string() {
        let mut lexer = Lexer::new("\"नमस्ते\"".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].value, "नमस्ते");
    }

    #[test]
    fn test_unicode_identifier() {
        let mut lexer = Lexer::new("नमस्ते".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].value, "नमस्ते");
    }

    #[test]
    fn test_eof_always_present() {
        let mut lexer = Lexer::new("maanau x = 5".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.last().unwrap().token_type, TokenType::EOF);
    }

    #[test]
    fn test_multiple_newlines() {
        let mut lexer = Lexer::new("maanau\n\n\nx".to_string());
        let tokens = lexer.tokenize().unwrap();
        let newline_count = tokens.iter().filter(|t| t.token_type == TokenType::Newline).count();
        assert_eq!(newline_count, 3);
    }
}