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