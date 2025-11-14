use crate::ast::ASTNode;
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let current_token = if tokens.is_empty() { None } else { Some(tokens[0].clone()) };
        
        Parser {
            tokens,
            pos: 0,
            current_token,
        }
    }
    
    fn advance(&mut self) {
        self.pos += 1;
        if self.pos >= self.tokens.len() {
            self.current_token = None;
        } else {
            self.current_token = Some(self.tokens[self.pos].clone());
        }
    }
    
    fn peek(&self) -> Option<&Token> {
        if self.pos + 1 >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.pos + 1])
        }
    }
    
    fn expect(&mut self, token_type: TokenType) -> Result<Token, String> {
        if let Some(ref token) = self.current_token {
            if token.token_type == token_type {
                let token = token.clone();
                self.advance();
                Ok(token)
            } else {
                Err(format!(
                    "Expected {:?}, found {:?} at line {}",
                    token_type, token.token_type, token.line
                ))
            }
        } else {
            Err(format!("Expected {:?}, found EOF", token_type))
        }
    }
    
    fn expect_keyword(&mut self, keyword: &str) -> Result<Token, String> {
        if let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Keyword && token.value == keyword {
                let token = token.clone();
                self.advance();
                Ok(token)
            } else {
                Err(format!(
                    "Expected keyword '{}', found '{}' at line {}",
                    keyword, token.value, token.line
                ))
            }
        } else {
            Err(format!("Expected keyword '{}', found EOF", keyword))
        }
    }
    
    fn skip_newlines(&mut self) {
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Newline {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    pub fn parse(&mut self) -> Result<ASTNode, String> {
        let mut statements = Vec::new();
        
        self.skip_newlines();
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::EOF {
                break;
            }
            
            if token.token_type == TokenType::Newline {
                self.advance();
                continue;
            }
            
            let stmt = self.parse_statement()?;
            statements.push(Box::new(stmt));
            
            self.skip_newlines();
        }
        
        Ok(ASTNode::new_program(statements))
    }
    
    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        if let Some(ref token) = self.current_token {
            match token.token_type {
                TokenType::Keyword => {
                    match token.value.as_str() {
                        "maanau" => self.parse_var_declaration(),
                        "yedi" => self.parse_if_statement(),
                        "jaba" => self.parse_while_loop(),
                        "pratyek" => self.parse_for_each_loop(),
                        "kaam" => self.parse_function_declaration(),
                        "pathau" => self.parse_return_statement(),
                        "bhan" => self.parse_print_statement(),
                        "rok" => self.parse_break_statement(),
                        "jane" => self.parse_continue_statement(),
                        "aayaat" => self.parse_import_statement(),
                        _ => Err(format!("Unexpected keyword '{}' at line {}", token.value, token.line)),
                    }
                }
                TokenType::Identifier => {
                    // Check if it's an assignment, index assignment, or expression
                    if let Some(next_token) = self.peek() {
                        if next_token.token_type == TokenType::Operator && next_token.value == "=" {
                            self.parse_assignment()
                        } else if next_token.token_type == TokenType::LBracket {
                            // Could be index assignment
                            self.parse_index_assignment_or_expression()
                        } else {
                            // Expression statement (function call)
                            self.parse_expression()
                        }
                    } else {
                        self.parse_expression()
                    }
                }
                _ => self.parse_expression(),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }
    
    fn parse_var_declaration(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("maanau")?;
        
        let name_token = self.expect(TokenType::Identifier)?;
        let name = name_token.value;
        
        let mut type_hint = None;
        
        // Check for optional type hint
        if let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Colon {
                self.advance(); // skip ':'
                let type_token = self.expect(TokenType::Identifier)?;
                type_hint = Some(type_token.value);
            }
        }
        
        self.expect(TokenType::Operator)?; // expect '='
        let value = self.parse_expression()?;
        
        Ok(ASTNode::new_var_declaration(name, type_hint, Box::new(value)))
    }
    
    fn parse_assignment(&mut self) -> Result<ASTNode, String> {
        let name_token = self.expect(TokenType::Identifier)?;
        let name = name_token.value;
        
        self.expect(TokenType::Operator)?; // expect '='
        let value = self.parse_expression()?;
        
        Ok(ASTNode::new_assignment(name, Box::new(value)))
    }
    
    fn parse_index_assignment_or_expression(&mut self) -> Result<ASTNode, String> {
        let expr = self.parse_expression()?;
        
        // Check if this is actually an assignment
        if let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Operator && token.value == "=" {
                // This is an index assignment: obj[index] = value
                if let ASTNode::IndexAccess { object, index } = expr {
                    self.advance(); // skip '='
                    let value = self.parse_expression()?;
                    return Ok(ASTNode::IndexAssignment { object, index, value: Box::new(value) });
                } else {
                    return Err("Invalid left-hand side in assignment".to_string());
                }
            }
        }
        
        Ok(expr)
    }
    
    fn parse_if_statement(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("yedi")?;
        
        let condition = self.parse_expression()?;
        
        self.expect_keyword("bhane")?;
        self.expect(TokenType::LBrace)?;
        
        let mut then_block = Vec::new();
        self.skip_newlines();
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::RBrace {
                break;
            }
            
            if token.token_type == TokenType::Newline {
                self.advance();
                continue;
            }
            
            let stmt = self.parse_statement()?;
            then_block.push(Box::new(stmt));
            
            self.skip_newlines();
        }
        
        self.expect(TokenType::RBrace)?;
        
        let mut else_block = None;
        
        // Check for else clause
        if let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Keyword && token.value == "natra" {
                self.advance(); // skip 'natra'
                self.expect(TokenType::LBrace)?;
                
                let mut else_statements = Vec::new();
                self.skip_newlines();
                
                while let Some(ref token) = self.current_token {
                    if token.token_type == TokenType::RBrace {
                        break;
                    }
                    
                    if token.token_type == TokenType::Newline {
                        self.advance();
                        continue;
                    }
                    
                    let stmt = self.parse_statement()?;
                    else_statements.push(Box::new(stmt));
                    
                    self.skip_newlines();
                }
                
                self.expect(TokenType::RBrace)?;
                else_block = Some(else_statements);
            }
        }
        
        Ok(ASTNode::new_if_statement(
            Box::new(condition),
            then_block,
            else_block,
        ))
    }
    
    fn parse_while_loop(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("jaba")?;
        self.expect_keyword("samma")?;
        
        let condition = self.parse_expression()?;
        
        self.expect(TokenType::LBrace)?;
        
        let mut body = Vec::new();
        self.skip_newlines();
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::RBrace {
                break;
            }
            
            if token.token_type == TokenType::Newline {
                self.advance();
                continue;
            }
            
            let stmt = self.parse_statement()?;
            body.push(Box::new(stmt));
            
            self.skip_newlines();
        }
        
        self.expect(TokenType::RBrace)?;
        
        Ok(ASTNode::new_while_loop(Box::new(condition), body))
    }
    
    fn parse_for_each_loop(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("pratyek")?;
        
        // Get the loop variable name
        let var_token = self.expect(TokenType::Identifier)?;
        let variable = var_token.value;
        
        // Expect 'ma' keyword
        self.expect_keyword("ma")?;
        
        // Parse the iterable expression
        let iterable = self.parse_expression()?;
        
        // Parse the loop body
        self.expect(TokenType::LBrace)?;
        
        let mut body = Vec::new();
        self.skip_newlines();
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::RBrace {
                break;
            }
            
            if token.token_type == TokenType::Newline {
                self.advance();
                continue;
            }
            
            let stmt = self.parse_statement()?;
            body.push(Box::new(stmt));
            
            self.skip_newlines();
        }
        
        self.expect(TokenType::RBrace)?;
        
        Ok(ASTNode::new_for_each_loop(variable, Box::new(iterable), body))
    }
    
    fn parse_function_declaration(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("kaam")?;
        
        let name_token = self.expect(TokenType::Identifier)?;
        let name = name_token.value;
        
        self.expect(TokenType::LParen)?;
        
        let mut parameters = Vec::new();
        
        // Parse parameter list
        if let Some(ref token) = self.current_token {
            if token.token_type != TokenType::RParen {
                loop {
                    let param_token = self.expect(TokenType::Identifier)?;
                    parameters.push(param_token.value);
                    
                    if let Some(ref token) = self.current_token {
                        if token.token_type == TokenType::Comma {
                            self.advance();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        
        self.expect(TokenType::RParen)?;
        self.expect(TokenType::LBrace)?;
        
        let mut body = Vec::new();
        self.skip_newlines();
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::RBrace {
                break;
            }
            
            if token.token_type == TokenType::Newline {
                self.advance();
                continue;
            }
            
            let stmt = self.parse_statement()?;
            body.push(Box::new(stmt));
            
            self.skip_newlines();
        }
        
        self.expect(TokenType::RBrace)?;
        
        Ok(ASTNode::new_function_declaration(name, parameters, body))
    }
    
    fn parse_return_statement(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("pathau")?;
        let expr = self.parse_expression()?;
        Ok(ASTNode::Return(Box::new(expr)))
    }
    
    fn parse_print_statement(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("bhan")?;
        let expr = self.parse_expression()?;
        Ok(ASTNode::Print(Box::new(expr)))
    }
    
    fn parse_break_statement(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("rok")?;
        Ok(ASTNode::Break)
    }
    
    fn parse_continue_statement(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("jane")?;
        Ok(ASTNode::Continue)
    }
    
    fn parse_import_statement(&mut self) -> Result<ASTNode, String> {
        self.expect_keyword("aayaat")?;
        
        let filename_token = self.expect(TokenType::String)?;
        let filename = filename_token.value;
        
        Ok(ASTNode::new_import(filename))
    }
    
    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_logical_or()
    }
    
    fn parse_logical_or(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_logical_and()?;
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Keyword && token.value == "wa" {
                let operator = token.value.clone();
                self.advance();
                let right = self.parse_logical_and()?;
                left = ASTNode::new_binary_op(Box::new(left), operator, Box::new(right));
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_logical_and(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_comparison()?;
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Keyword && token.value == "ra" {
                let operator = token.value.clone();
                self.advance();
                let right = self.parse_comparison()?;
                left = ASTNode::new_binary_op(Box::new(left), operator, Box::new(right));
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_addition()?;
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Operator {
                match token.value.as_str() {
                    "==" | "!=" | ">" | "<" | ">=" | "<=" => {
                        let operator = token.value.clone();
                        self.advance();
                        let right = self.parse_addition()?;
                        left = ASTNode::new_binary_op(Box::new(left), operator, Box::new(right));
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_addition(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_multiplication()?;
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Operator {
                match token.value.as_str() {
                    "+" | "-" => {
                        let operator = token.value.clone();
                        self.advance();
                        let right = self.parse_multiplication()?;
                        left = ASTNode::new_binary_op(Box::new(left), operator, Box::new(right));
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_multiplication(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_unary()?;
        
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Operator {
                match token.value.as_str() {
                    "*" | "/" | "%" => {
                        let operator = token.value.clone();
                        self.advance();
                        let right = self.parse_unary()?;
                        left = ASTNode::new_binary_op(Box::new(left), operator, Box::new(right));
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<ASTNode, String> {
        if let Some(ref token) = self.current_token {
            match token.token_type {
                TokenType::Keyword if token.value == "hoina" => {
                    let operator = token.value.clone();
                    self.advance();
                    let operand = self.parse_unary()?;
                    Ok(ASTNode::new_unary_op(operator, Box::new(operand)))
                }
                TokenType::Operator if token.value == "-" => {
                    let operator = token.value.clone();
                    self.advance();
                    let operand = self.parse_unary()?;
                    Ok(ASTNode::new_unary_op(operator, Box::new(operand)))
                }
                _ => self.parse_primary(),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }
    
    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        if let Some(ref token) = self.current_token {
            match token.token_type {
                TokenType::Number => {
                    let value = token.value.clone();
                    self.advance();
                    Ok(ASTNode::Number(value))
                }
                TokenType::String => {
                    let value = token.value.clone();
                    self.advance();
                    Ok(ASTNode::String(value))
                }
                TokenType::Keyword => {
                    match token.value.as_str() {
                        "sahi" => {
                            self.advance();
                            Ok(ASTNode::Boolean(true))
                        }
                        "galat" => {
                            self.advance();
                            Ok(ASTNode::Boolean(false))
                        }
                        _ => Err(format!("Unexpected keyword '{}' in expression", token.value)),
                    }
                }
                TokenType::Identifier => {
                    let name = token.value.clone();
                    self.advance();
                    
                    let mut result = ASTNode::Identifier(name.clone());
                    
                    // Handle function calls or indexing
                    loop {
                        if let Some(ref token) = self.current_token {
                            if token.token_type == TokenType::LParen {
                                // Function call - only valid for identifiers
                                if let ASTNode::Identifier(func_name) = &result {
                                    self.advance(); // skip '('
                                    
                                    let mut arguments = Vec::new();
                                    
                                    if let Some(ref token) = self.current_token {
                                        if token.token_type != TokenType::RParen {
                                            loop {
                                                let arg = self.parse_expression()?;
                                                arguments.push(Box::new(arg));
                                                
                                                if let Some(ref token) = self.current_token {
                                                    if token.token_type == TokenType::Comma {
                                                        self.advance();
                                                    } else {
                                                        break;
                                                    }
                                                } else {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    
                                    self.expect(TokenType::RParen)?;
                                    result = ASTNode::new_function_call(func_name.clone(), arguments);
                                } else {
                                    return Err("Cannot call function on non-identifier".to_string());
                                }
                            } else if token.token_type == TokenType::LBracket {
                                // Index access
                                self.advance(); // skip '['
                                let index = self.parse_expression()?;
                                self.expect(TokenType::RBracket)?;
                                result = ASTNode::new_index_access(Box::new(result), Box::new(index));
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    
                    Ok(result)
                }
                TokenType::LBracket => {
                    // List literal: [1, 2, 3]
                    self.advance(); // skip '['
                    self.skip_newlines();
                    
                    let mut elements = Vec::new();
                    
                    if let Some(ref token) = self.current_token {
                        if token.token_type != TokenType::RBracket {
                            loop {
                                let element = self.parse_expression()?;
                                elements.push(Box::new(element));
                                
                                self.skip_newlines();
                                
                                if let Some(ref token) = self.current_token {
                                    if token.token_type == TokenType::Comma {
                                        self.advance();
                                        self.skip_newlines();
                                    } else {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    
                    self.expect(TokenType::RBracket)?;
                    Ok(ASTNode::new_list_literal(elements))
                }
                TokenType::LBrace => {
                    // Dictionary literal: {"key": value, "key2": value2}
                    self.advance(); // skip '{'
                    self.skip_newlines();
                    
                    let mut pairs = Vec::new();
                    
                    if let Some(ref token) = self.current_token {
                        if token.token_type != TokenType::RBrace {
                            loop {
                                // Parse key (must be string)
                                let key_token = self.expect(TokenType::String)?;
                                let key = key_token.value;
                                
                                self.expect(TokenType::Colon)?;
                                self.skip_newlines();
                                
                                let value = self.parse_expression()?;
                                pairs.push((key, Box::new(value)));
                                
                                self.skip_newlines();
                                
                                if let Some(ref token) = self.current_token {
                                    if token.token_type == TokenType::Comma {
                                        self.advance();
                                        self.skip_newlines();
                                    } else {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    
                    self.expect(TokenType::RBrace)?;
                    Ok(ASTNode::new_dictionary_literal(pairs))
                }
                TokenType::LParen => {
                    self.advance(); // skip '('
                    let expr = self.parse_expression()?;
                    self.expect(TokenType::RParen)?;
                    Ok(expr)
                }
                _ => Err(format!("Unexpected token {:?} in expression", token)),
            }
        } else {
            Err("Unexpected end of input in expression".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenType};

    // Helper function to create tokens easily
    fn make_token(token_type: TokenType, value: &str) -> Token {
        Token::new(token_type, value.to_string(), 1, 1)
    }

    fn keyword(s: &str) -> Token {
        make_token(TokenType::Keyword, s)
    }

    fn identifier(s: &str) -> Token {
        make_token(TokenType::Identifier, s)
    }

    fn number(s: &str) -> Token {
        make_token(TokenType::Number, s)
    }

    fn string(s: &str) -> Token {
        make_token(TokenType::String, s)
    }

    fn operator(s: &str) -> Token {
        make_token(TokenType::Operator, s)
    }

    fn eof() -> Token {
        make_token(TokenType::EOF, "")
    }

    #[test]
    fn test_parse_empty_program() {
        let tokens = vec![eof()];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        match ast {
            ASTNode::Program(stmts) => assert_eq!(stmts.len(), 0),
            _ => panic!("Expected Program node"),
        }
    }

    #[test]
    fn test_parse_var_declaration() {
        let tokens = vec![
            keyword("maanau"),
            identifier("x"),
            operator("="),
            number("5"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                assert_eq!(stmts.len(), 1);
                match stmts[0].as_ref() {
                    ASTNode::VarDeclaration { name, value, .. } => {
                        assert_eq!(name, "x");
                        match value.as_ref() {
                            ASTNode::Number(n) => assert_eq!(n, "5"),
                            _ => panic!("Expected number"),
                        }
                    }
                    _ => panic!("Expected VarDeclaration"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_assignment() {
        let tokens = vec![
            identifier("x"),
            operator("="),
            number("10"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                assert_eq!(stmts.len(), 1);
                match stmts[0].as_ref() {
                    ASTNode::Assignment { name, value } => {
                        assert_eq!(name, "x");
                        match value.as_ref() {
                            ASTNode::Number(n) => assert_eq!(n, "10"),
                            _ => panic!("Expected number"),
                        }
                    }
                    _ => panic!("Expected Assignment"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_operator_precedence_multiplication_over_addition() {
        // Test that 2 + 3 * 4 is parsed as 2 + (3 * 4) = 14, not (2 + 3) * 4 = 20
        let tokens = vec![
            number("2"),
            operator("+"),
            number("3"),
            operator("*"),
            number("4"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::BinaryOp { left, operator: op, right } => {
                        assert_eq!(op, "+");
                        // Left should be 2
                        match left.as_ref() {
                            ASTNode::Number(n) => assert_eq!(n, "2"),
                            _ => panic!("Expected left to be 2"),
                        }
                        // Right should be (3 * 4)
                        match right.as_ref() {
                            ASTNode::BinaryOp { left: l2, operator: op2, right: r2 } => {
                                assert_eq!(op2, "*");
                                match l2.as_ref() {
                                    ASTNode::Number(n) => assert_eq!(n, "3"),
                                    _ => panic!("Expected 3"),
                                }
                                match r2.as_ref() {
                                    ASTNode::Number(n) => assert_eq!(n, "4"),
                                    _ => panic!("Expected 4"),
                                }
                            }
                            _ => panic!("Expected right to be multiplication"),
                        }
                    }
                    _ => panic!("Expected BinaryOp"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_operator_precedence_division_over_subtraction() {
        // Test that 10 - 6 / 2 is parsed as 10 - (6 / 2) = 7, not (10 - 6) / 2 = 2
        let tokens = vec![
            number("10"),
            operator("-"),
            number("6"),
            operator("/"),
            number("2"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::BinaryOp { left, operator: op, right } => {
                        assert_eq!(op, "-");
                        match left.as_ref() {
                            ASTNode::Number(n) => assert_eq!(n, "10"),
                            _ => panic!("Expected 10"),
                        }
                        match right.as_ref() {
                            ASTNode::BinaryOp { operator: op2, .. } => {
                                assert_eq!(op2, "/");
                            }
                            _ => panic!("Expected division"),
                        }
                    }
                    _ => panic!("Expected BinaryOp"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_operator_precedence_comparison_over_logical_and() {
        // Test that 5 > 3 ra 10 < 20 is parsed as (5 > 3) ra (10 < 20)
        let tokens = vec![
            number("5"),
            operator(">"),
            number("3"),
            keyword("ra"),
            number("10"),
            operator("<"),
            number("20"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::BinaryOp { operator: op, .. } => {
                        assert_eq!(op, "ra");
                    }
                    _ => panic!("Expected BinaryOp with 'ra'"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_operator_precedence_logical_and_over_logical_or() {
        // Test that A wa B ra C is parsed as A wa (B ra C)
        let tokens = vec![
            keyword("sahi"),
            keyword("wa"),
            keyword("galat"),
            keyword("ra"),
            keyword("sahi"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::BinaryOp { operator: op, right, .. } => {
                        assert_eq!(op, "wa");
                        // Right should be (galat ra sahi)
                        match right.as_ref() {
                            ASTNode::BinaryOp { operator: op2, .. } => {
                                assert_eq!(op2, "ra");
                            }
                            _ => panic!("Expected ra operator"),
                        }
                    }
                    _ => panic!("Expected BinaryOp"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_parentheses_override_precedence() {
        // Test that (2 + 3) * 4 is parsed as (2 + 3) * 4 = 20, not 2 + (3 * 4) = 14
        let tokens = vec![
            make_token(TokenType::LParen, "("),
            number("2"),
            operator("+"),
            number("3"),
            make_token(TokenType::RParen, ")"),
            operator("*"),
            number("4"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::BinaryOp { operator: op, left, .. } => {
                        assert_eq!(op, "*");
                        // Left should be (2 + 3)
                        match left.as_ref() {
                            ASTNode::BinaryOp { operator: op2, .. } => {
                                assert_eq!(op2, "+");
                            }
                            _ => panic!("Expected addition"),
                        }
                    }
                    _ => panic!("Expected BinaryOp"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_unary_minus() {
        let tokens = vec![
            operator("-"),
            number("5"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::UnaryOp { operator: op, operand } => {
                        assert_eq!(op, "-");
                        match operand.as_ref() {
                            ASTNode::Number(n) => assert_eq!(n, "5"),
                            _ => panic!("Expected number"),
                        }
                    }
                    _ => panic!("Expected UnaryOp"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_unary_not() {
        let tokens = vec![
            keyword("hoina"),
            keyword("sahi"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::UnaryOp { operator: op, .. } => {
                        assert_eq!(op, "hoina");
                    }
                    _ => panic!("Expected UnaryOp"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_function_declaration() {
        let tokens = vec![
            keyword("kaam"),
            identifier("add"),
            make_token(TokenType::LParen, "("),
            identifier("a"),
            make_token(TokenType::Comma, ","),
            identifier("b"),
            make_token(TokenType::RParen, ")"),
            make_token(TokenType::LBrace, "{"),
            keyword("pathau"),
            identifier("a"),
            make_token(TokenType::RBrace, "}"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::FunctionDeclaration { name, parameters, body } => {
                        assert_eq!(name, "add");
                        assert_eq!(parameters.len(), 2);
                        assert_eq!(parameters[0], "a");
                        assert_eq!(parameters[1], "b");
                        assert_eq!(body.len(), 1);
                    }
                    _ => panic!("Expected FunctionDeclaration"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_function_call() {
        let tokens = vec![
            identifier("add"),
            make_token(TokenType::LParen, "("),
            number("5"),
            make_token(TokenType::Comma, ","),
            number("10"),
            make_token(TokenType::RParen, ")"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::FunctionCall { name, arguments } => {
                        assert_eq!(name, "add");
                        assert_eq!(arguments.len(), 2);
                    }
                    _ => panic!("Expected FunctionCall"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let tokens = vec![
            keyword("yedi"),
            keyword("sahi"),
            keyword("bhane"),
            make_token(TokenType::LBrace, "{"),
            keyword("bhan"),
            string("yes"),
            make_token(TokenType::RBrace, "}"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::IfStatement { condition, then_block, else_block } => {
                        match condition.as_ref() {
                            ASTNode::Boolean(b) => assert_eq!(*b, true),
                            _ => panic!("Expected boolean"),
                        }
                        assert_eq!(then_block.len(), 1);
                        assert!(else_block.is_none());
                    }
                    _ => panic!("Expected IfStatement"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_if_else_statement() {
        let tokens = vec![
            keyword("yedi"),
            keyword("galat"),
            keyword("bhane"),
            make_token(TokenType::LBrace, "{"),
            make_token(TokenType::RBrace, "}"),
            keyword("natra"),
            make_token(TokenType::LBrace, "{"),
            keyword("bhan"),
            string("no"),
            make_token(TokenType::RBrace, "}"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::IfStatement { else_block, .. } => {
                        assert!(else_block.is_some());
                        assert_eq!(else_block.as_ref().unwrap().len(), 1);
                    }
                    _ => panic!("Expected IfStatement"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_while_loop() {
        let tokens = vec![
            keyword("jaba"),
            keyword("samma"),
            keyword("sahi"),
            make_token(TokenType::LBrace, "{"),
            keyword("bhan"),
            string("loop"),
            make_token(TokenType::RBrace, "}"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::WhileLoop { condition, body } => {
                        match condition.as_ref() {
                            ASTNode::Boolean(b) => assert_eq!(*b, true),
                            _ => panic!("Expected boolean"),
                        }
                        assert_eq!(body.len(), 1);
                    }
                    _ => panic!("Expected WhileLoop"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_for_each_loop() {
        let tokens = vec![
            keyword("pratyek"),
            identifier("item"),
            keyword("ma"),
            identifier("list"),
            make_token(TokenType::LBrace, "{"),
            keyword("bhan"),
            identifier("item"),
            make_token(TokenType::RBrace, "}"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::ForEachLoop { variable, iterable, body } => {
                        assert_eq!(variable, "item");
                        match iterable.as_ref() {
                            ASTNode::Identifier(name) => assert_eq!(name, "list"),
                            _ => panic!("Expected identifier"),
                        }
                        assert_eq!(body.len(), 1);
                    }
                    _ => panic!("Expected ForEachLoop"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_list_literal() {
        let tokens = vec![
            make_token(TokenType::LBracket, "["),
            number("1"),
            make_token(TokenType::Comma, ","),
            number("2"),
            make_token(TokenType::Comma, ","),
            number("3"),
            make_token(TokenType::RBracket, "]"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::ListLiteral(elements) => {
                        assert_eq!(elements.len(), 3);
                    }
                    _ => panic!("Expected ListLiteral"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_empty_list() {
        let tokens = vec![
            make_token(TokenType::LBracket, "["),
            make_token(TokenType::RBracket, "]"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::ListLiteral(elements) => {
                        assert_eq!(elements.len(), 0);
                    }
                    _ => panic!("Expected ListLiteral"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_dictionary_literal() {
        let tokens = vec![
            make_token(TokenType::LBrace, "{"),
            string("key"),
            make_token(TokenType::Colon, ":"),
            number("42"),
            make_token(TokenType::RBrace, "}"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::DictionaryLiteral(pairs) => {
                        assert_eq!(pairs.len(), 1);
                        assert_eq!(pairs[0].0, "key");
                    }
                    _ => panic!("Expected DictionaryLiteral"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_index_access() {
        let tokens = vec![
            identifier("list"),
            make_token(TokenType::LBracket, "["),
            number("0"),
            make_token(TokenType::RBracket, "]"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::IndexAccess { object, index } => {
                        match object.as_ref() {
                            ASTNode::Identifier(name) => assert_eq!(name, "list"),
                            _ => panic!("Expected identifier"),
                        }
                        match index.as_ref() {
                            ASTNode::Number(n) => assert_eq!(n, "0"),
                            _ => panic!("Expected number"),
                        }
                    }
                    _ => panic!("Expected IndexAccess"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_index_assignment() {
        let tokens = vec![
            identifier("list"),
            make_token(TokenType::LBracket, "["),
            number("0"),
            make_token(TokenType::RBracket, "]"),
            operator("="),
            number("42"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::IndexAssignment { object, index, value } => {
                        match object.as_ref() {
                            ASTNode::Identifier(name) => assert_eq!(name, "list"),
                            _ => panic!("Expected identifier"),
                        }
                        match index.as_ref() {
                            ASTNode::Number(n) => assert_eq!(n, "0"),
                            _ => panic!("Expected number"),
                        }
                        match value.as_ref() {
                            ASTNode::Number(n) => assert_eq!(n, "42"),
                            _ => panic!("Expected number"),
                        }
                    }
                    _ => panic!("Expected IndexAssignment"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_print_statement() {
        let tokens = vec![
            keyword("bhan"),
            string("hello"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::Print(_) => {}
                    _ => panic!("Expected Print"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_return_statement() {
        let tokens = vec![
            keyword("pathau"),
            number("42"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::Return(value) => {
                        match value.as_ref() {
                            ASTNode::Number(n) => assert_eq!(n, "42"),
                            _ => panic!("Expected number"),
                        }
                    }
                    _ => panic!("Expected Return"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_break_statement() {
        let tokens = vec![
            keyword("rok"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::Break => {}
                    _ => panic!("Expected Break"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_continue_statement() {
        let tokens = vec![
            keyword("jane"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::Continue => {}
                    _ => panic!("Expected Continue"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_import_statement() {
        let tokens = vec![
            keyword("aayaat"),
            string("module.nep"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::Import { filename } => {
                        assert_eq!(filename, "module.nep");
                    }
                    _ => panic!("Expected Import"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }

    #[test]
    fn test_parse_comparison_operators() {
        let operators = vec!["==", "!=", ">", "<", ">=", "<="];

        for op in operators {
            let tokens = vec![
                number("5"),
                operator(op),
                number("10"),
                eof(),
            ];
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();

            match ast {
                ASTNode::Program(stmts) => {
                    match stmts[0].as_ref() {
                        ASTNode::BinaryOp { operator, .. } => {
                            assert_eq!(operator, op);
                        }
                        _ => panic!("Expected BinaryOp for operator {}", op),
                    }
                }
                _ => panic!("Expected Program"),
            }
        }
    }

    #[test]
    fn test_parse_all_arithmetic_operators() {
        let operators = vec!["+", "-", "*", "/", "%"];

        for op in operators {
            let tokens = vec![
                number("10"),
                operator(op),
                number("5"),
                eof(),
            ];
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();

            match ast {
                ASTNode::Program(stmts) => {
                    match stmts[0].as_ref() {
                        ASTNode::BinaryOp { operator, .. } => {
                            assert_eq!(operator, op);
                        }
                        _ => panic!("Expected BinaryOp for operator {}", op),
                    }
                }
                _ => panic!("Expected Program"),
            }
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        // Test: (5 + 3) * 2 - 10 / 5
        let tokens = vec![
            make_token(TokenType::LParen, "("),
            number("5"),
            operator("+"),
            number("3"),
            make_token(TokenType::RParen, ")"),
            operator("*"),
            number("2"),
            operator("-"),
            number("10"),
            operator("/"),
            number("5"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_nested_function_calls() {
        // Test: add(mul(2, 3), 5)
        let tokens = vec![
            identifier("add"),
            make_token(TokenType::LParen, "("),
            identifier("mul"),
            make_token(TokenType::LParen, "("),
            number("2"),
            make_token(TokenType::Comma, ","),
            number("3"),
            make_token(TokenType::RParen, ")"),
            make_token(TokenType::Comma, ","),
            number("5"),
            make_token(TokenType::RParen, ")"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_chained_index_access() {
        // Test: list[0][1]
        let tokens = vec![
            identifier("list"),
            make_token(TokenType::LBracket, "["),
            number("0"),
            make_token(TokenType::RBracket, "]"),
            make_token(TokenType::LBracket, "["),
            number("1"),
            make_token(TokenType::RBracket, "]"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_error_missing_closing_paren() {
        let tokens = vec![
            make_token(TokenType::LParen, "("),
            number("5"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_unexpected_token() {
        let tokens = vec![
            operator("+"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_modulo_operator_precedence() {
        // Test that 10 + 5 % 3 is parsed as 10 + (5 % 3)
        let tokens = vec![
            number("10"),
            operator("+"),
            number("5"),
            operator("%"),
            number("3"),
            eof(),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            ASTNode::Program(stmts) => {
                match stmts[0].as_ref() {
                    ASTNode::BinaryOp { operator: op, right, .. } => {
                        assert_eq!(op, "+");
                        match right.as_ref() {
                            ASTNode::BinaryOp { operator: op2, .. } => {
                                assert_eq!(op2, "%");
                            }
                            _ => panic!("Expected modulo operation"),
                        }
                    }
                    _ => panic!("Expected BinaryOp"),
                }
            }
            _ => panic!("Expected Program"),
        }
    }
}