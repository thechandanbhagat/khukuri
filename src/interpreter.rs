use crate::ast::ASTNode;
use crate::environment::Environment;
use crate::value::Value;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum ControlFlow {
    Return(Value),
    Break,
    Continue,
    None,
}

pub struct Interpreter {
    environment: Environment,
    functions: HashMap<String, (Vec<String>, Vec<Box<ASTNode>>)>, // (params, body)
    imported_modules: HashMap<String, bool>, // Track imported modules to prevent circular imports
    importing_stack: Vec<String>, // Track current import chain to prevent circular imports
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
            functions: HashMap::new(),
            imported_modules: HashMap::new(),
            importing_stack: Vec::new(),
        }
    }
    
    pub fn interpret(&mut self, node: &ASTNode) -> Result<Value, String> {
        match self.interpret_with_control(node)? {
            ControlFlow::Return(value) => Ok(value),
            ControlFlow::None => Ok(Value::Null),
            ControlFlow::Break => Err("Break statement outside loop".to_string()),
            ControlFlow::Continue => Err("Continue statement outside loop".to_string()),
        }
    }
    
    fn interpret_with_control(&mut self, node: &ASTNode) -> Result<ControlFlow, String> {
        match node {
            ASTNode::Program(statements) => {
                for stmt in statements {
                    match self.interpret_with_control(stmt)? {
                        ControlFlow::None => continue,
                        flow => return Ok(flow),
                    }
                }
                Ok(ControlFlow::None)
            }
            
            ASTNode::VarDeclaration { name, value, .. } => {
                let val = self.evaluate_expression(value)?;
                self.environment.define(name.clone(), val);
                Ok(ControlFlow::None)
            }
            
            ASTNode::Assignment { name, value } => {
                let val = self.evaluate_expression(value)?;
                self.environment.set(name, val)?;
                Ok(ControlFlow::None)
            }
            
            ASTNode::IndexAssignment { object, index, value } => {
                let index_val = self.evaluate_expression(index)?;
                let new_value = self.evaluate_expression(value)?;
                
                // Get the object to modify
                if let ASTNode::Identifier(name) = object.as_ref() {
                    if let Some(mut obj) = self.environment.get(name) {
                        match (&mut obj, &index_val) {
                            (Value::List(list), Value::Number(n)) => {
                                let idx = *n as usize;
                                if idx < list.len() {
                                    list[idx] = new_value;
                                    self.environment.set(name, obj)?;
                                } else {
                                    return Err(format!("List index {} out of bounds", idx));
                                }
                            }
                            (Value::Dictionary(dict), Value::String(key)) => {
                                dict.insert(key.clone(), new_value);
                                self.environment.set(name, obj)?;
                            }
                            _ => return Err("Invalid index assignment".to_string()),
                        }
                    } else {
                        return Err(format!("Undefined variable: {}", name));
                    }
                } else {
                    return Err("Invalid left-hand side in index assignment".to_string());
                }
                
                Ok(ControlFlow::None)
            }
            
            ASTNode::IfStatement { condition, then_block, else_block } => {
                let cond_value = self.evaluate_expression(condition)?;
                
                if cond_value.is_truthy() {
                    self.environment.push_scope();
                    let mut result = ControlFlow::None;
                    
                    for stmt in then_block {
                        result = self.interpret_with_control(stmt)?;
                        if !matches!(result, ControlFlow::None) {
                            break;
                        }
                    }
                    
                    self.environment.pop_scope();
                    Ok(result)
                } else if let Some(else_stmts) = else_block {
                    self.environment.push_scope();
                    let mut result = ControlFlow::None;
                    
                    for stmt in else_stmts {
                        result = self.interpret_with_control(stmt)?;
                        if !matches!(result, ControlFlow::None) {
                            break;
                        }
                    }
                    
                    self.environment.pop_scope();
                    Ok(result)
                } else {
                    Ok(ControlFlow::None)
                }
            }
            
            ASTNode::WhileLoop { condition, body } => {
                loop {
                    let cond_value = self.evaluate_expression(condition)?;
                    if !cond_value.is_truthy() {
                        break;
                    }
                    
                    self.environment.push_scope();
                    let mut should_break = false;
                    
                    for stmt in body {
                        match self.interpret_with_control(stmt)? {
                            ControlFlow::None => continue,
                            ControlFlow::Break => {
                                should_break = true;
                                break;
                            }
                            ControlFlow::Continue => break,
                            flow @ ControlFlow::Return(_) => {
                                self.environment.pop_scope();
                                return Ok(flow);
                            }
                        }
                    }
                    
                    self.environment.pop_scope();
                    
                    if should_break {
                        break;
                    }
                }
                Ok(ControlFlow::None)
            }
            
            ASTNode::ForEachLoop { variable, iterable, body } => {
                let iterable_value = self.evaluate_expression(iterable)?;
                
                match iterable_value {
                    Value::List(list) => {
                        for item in list {
                            self.environment.push_scope();
                            self.environment.define(variable.clone(), item);
                            
                            let mut should_break = false;
                            for stmt in body {
                                match self.interpret_with_control(stmt)? {
                                    ControlFlow::None => continue,
                                    ControlFlow::Break => {
                                        should_break = true;
                                        break;
                                    }
                                    ControlFlow::Continue => break,
                                    flow @ ControlFlow::Return(_) => {
                                        self.environment.pop_scope();
                                        return Ok(flow);
                                    }
                                }
                            }
                            
                            self.environment.pop_scope();
                            
                            if should_break {
                                break;
                            }
                        }
                    }
                    Value::Dictionary(dict) => {
                        for (key, _value) in dict {
                            self.environment.push_scope();
                            // For dictionaries, iterate over keys
                            self.environment.define(variable.clone(), Value::String(key));
                            
                            let mut should_break = false;
                            for stmt in body {
                                match self.interpret_with_control(stmt)? {
                                    ControlFlow::None => continue,
                                    ControlFlow::Break => {
                                        should_break = true;
                                        break;
                                    }
                                    ControlFlow::Continue => break,
                                    flow @ ControlFlow::Return(_) => {
                                        self.environment.pop_scope();
                                        return Ok(flow);
                                    }
                                }
                            }
                            
                            self.environment.pop_scope();
                            
                            if should_break {
                                break;
                            }
                        }
                    }
                    Value::String(s) => {
                        for ch in s.chars() {
                            self.environment.push_scope();
                            self.environment.define(variable.clone(), Value::String(ch.to_string()));
                            
                            let mut should_break = false;
                            for stmt in body {
                                match self.interpret_with_control(stmt)? {
                                    ControlFlow::None => continue,
                                    ControlFlow::Break => {
                                        should_break = true;
                                        break;
                                    }
                                    ControlFlow::Continue => break,
                                    flow @ ControlFlow::Return(_) => {
                                        self.environment.pop_scope();
                                        return Ok(flow);
                                    }
                                }
                            }
                            
                            self.environment.pop_scope();
                            
                            if should_break {
                                break;
                            }
                        }
                    }
                    _ => return Err(format!("Cannot iterate over {}", iterable_value.get_type())),
                }
                
                Ok(ControlFlow::None)
            }
            
            ASTNode::FunctionDeclaration { name, parameters, body } => {
                self.functions.insert(
                    name.clone(),
                    (parameters.clone(), body.clone())
                );
                Ok(ControlFlow::None)
            }
            
            ASTNode::Return(expr) => {
                let value = self.evaluate_expression(expr)?;
                Ok(ControlFlow::Return(value))
            }
            
            ASTNode::Print(expr) => {
                let value = self.evaluate_expression(expr)?;
                println!("{}", value.to_string());
                Ok(ControlFlow::None)
            }
            
            ASTNode::Import { filename } => {
                self.execute_import(filename)?;
                Ok(ControlFlow::None)
            }
            
            ASTNode::Break => Ok(ControlFlow::Break),
            
            ASTNode::Continue => Ok(ControlFlow::Continue),
            
            // Expression statements
            _ => {
                self.evaluate_expression(node)?;
                Ok(ControlFlow::None)
            }
        }
    }
    
    fn evaluate_expression(&mut self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::BinaryOp { left, operator, right } => {
                self.eval_binary_op(left, operator, right)
            }
            
            ASTNode::UnaryOp { operator, operand } => {
                self.eval_unary_op(operator, operand)
            }
            
            ASTNode::FunctionCall { name, arguments } => {
                self.call_function(name, arguments)
            }
            
            ASTNode::ListLiteral(elements) => {
                let mut list = Vec::new();
                for element in elements {
                    let value = self.evaluate_expression(element)?;
                    list.push(value);
                }
                Ok(Value::List(list))
            }
            
            ASTNode::DictionaryLiteral(pairs) => {
                let mut dict = HashMap::new();
                for (key, value_expr) in pairs {
                    let value = self.evaluate_expression(value_expr)?;
                    dict.insert(key.clone(), value);
                }
                Ok(Value::Dictionary(dict))
            }
            
            ASTNode::IndexAccess { object, index } => {
                let obj_val = self.evaluate_expression(object)?;
                let index_val = self.evaluate_expression(index)?;
                
                match (&obj_val, &index_val) {
                    (Value::List(list), Value::Number(n)) => {
                        let idx = *n as usize;
                        if idx < list.len() {
                            Ok(list[idx].clone())
                        } else {
                            Err(format!("List index {} out of bounds", idx))
                        }
                    }
                    (Value::Dictionary(dict), Value::String(key)) => {
                        dict.get(key)
                            .cloned()
                            .ok_or_else(|| format!("Key '{}' not found in dictionary", key))
                    }
                    (Value::String(s), Value::Number(n)) => {
                        let idx = *n as usize;
                        if idx < s.len() {
                            let ch = s.chars().nth(idx).unwrap();
                            Ok(Value::String(ch.to_string()))
                        } else {
                            Err(format!("String index {} out of bounds", idx))
                        }
                    }
                    _ => Err(format!("Cannot index {} with {}", 
                                   obj_val.get_type(), index_val.get_type()))
                }
            }
            
            ASTNode::Identifier(name) => {
                self.environment.get(name)
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            
            ASTNode::Number(val) => {
                val.parse::<f64>()
                    .map(Value::Number)
                    .map_err(|_| format!("Invalid number: {}", val))
            }
            
            ASTNode::String(val) => Ok(Value::String(val.clone())),
            
            ASTNode::Boolean(val) => Ok(Value::Boolean(*val)),
            
            _ => Err("Invalid expression".to_string()),
        }
    }
    
    fn eval_binary_op(&mut self, left: &ASTNode, operator: &str, right: &ASTNode) 
        -> Result<Value, String> {
        let left_val = self.evaluate_expression(left)?;
        let right_val = self.evaluate_expression(right)?;
        
        match (&left_val, operator, &right_val) {
            (Value::Number(l), "+", Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::Number(l), "-", Value::Number(r)) => Ok(Value::Number(l - r)),
            (Value::Number(l), "*", Value::Number(r)) => Ok(Value::Number(l * r)),
            (Value::Number(l), "/", Value::Number(r)) => {
                if *r == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Number(l / r))
                }
            }
            (Value::Number(l), "%", Value::Number(r)) => {
                if *r == 0.0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Number(l % r))
                }
            }
            (Value::Number(l), ">", Value::Number(r)) => Ok(Value::Boolean(l > r)),
            (Value::Number(l), "<", Value::Number(r)) => Ok(Value::Boolean(l < r)),
            (Value::Number(l), ">=", Value::Number(r)) => Ok(Value::Boolean(l >= r)),
            (Value::Number(l), "<=", Value::Number(r)) => Ok(Value::Boolean(l <= r)),
            (Value::Number(l), "==", Value::Number(r)) => Ok(Value::Boolean(l == r)),
            (Value::Number(l), "!=", Value::Number(r)) => Ok(Value::Boolean(l != r)),
            
            (Value::String(l), "+", Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
            (Value::String(l), "==", Value::String(r)) => Ok(Value::Boolean(l == r)),
            (Value::String(l), "!=", Value::String(r)) => Ok(Value::Boolean(l != r)),
            
            (Value::Boolean(l), "==", Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
            (Value::Boolean(l), "!=", Value::Boolean(r)) => Ok(Value::Boolean(l != r)),
            
            // Logical operators (ra = and, wa = or)
            (l, "ra", r) => Ok(Value::Boolean(l.is_truthy() && r.is_truthy())),
            (l, "wa", r) => Ok(Value::Boolean(l.is_truthy() || r.is_truthy())),
            
            // String and number concatenation
            (Value::String(l), "+", Value::Number(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::Number(l), "+", Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            
            _ => Err(format!("Invalid operation: {} {} {}", 
                           left_val.to_string(), operator, right_val.to_string()))
        }
    }
    
    fn eval_unary_op(&mut self, operator: &str, operand: &ASTNode) 
        -> Result<Value, String> {
        let val = self.evaluate_expression(operand)?;
        
        match operator {
            "hoina" => Ok(Value::Boolean(!val.is_truthy())),
            "-" => {
                if let Value::Number(n) = val {
                    Ok(Value::Number(-n))
                } else {
                    Err("Cannot negate non-number".to_string())
                }
            }
            _ => Err(format!("Unknown unary operator: {}", operator))
        }
    }
    
    fn call_function(&mut self, name: &str, arguments: &[Box<ASTNode>]) 
        -> Result<Value, String> {
        // Get function definition
        let (params, body) = self.functions.get(name)
            .ok_or_else(|| format!("Undefined function: {}", name))?
            .clone();
        
        // Check argument count
        if arguments.len() != params.len() {
            return Err(format!(
                "Function {} expects {} arguments, got {}",
                name, params.len(), arguments.len()
            ));
        }
        
        // Evaluate arguments
        let mut arg_values = Vec::new();
        for arg in arguments {
            arg_values.push(self.evaluate_expression(arg)?);
        }
        
        // Create new scope for function
        self.environment.push_scope();
        
        // Bind parameters
        for (param, value) in params.iter().zip(arg_values.iter()) {
            self.environment.define(param.clone(), value.clone());
        }
        
        // Execute function body
        let mut result = Value::Null;
        
        for stmt in &body {
            match self.interpret_with_control(stmt)? {
                ControlFlow::Return(value) => {
                    result = value;
                    break;
                }
                ControlFlow::None => continue,
                ControlFlow::Break => return Err("Break statement outside loop".to_string()),
                ControlFlow::Continue => return Err("Continue statement outside loop".to_string()),
            }
        }
        
        // Restore scope
        self.environment.pop_scope();
        
        Ok(result)
    }
    
    fn execute_import(&mut self, filename: &str) -> Result<(), String> {
        // Check if already imported - if so, skip
        if self.imported_modules.contains_key(filename) {
            return Ok(()); // Already imported, skip
        }
        
        // Check for circular imports in current import chain
        if self.importing_stack.contains(&filename.to_string()) {
            return Err(format!("Circular import bhettayo bro: {}", filename));
        }
        
        // Add to import stack
        self.importing_stack.push(filename.to_string());
        
        // Read the file
        let file_path = Path::new(filename);
        let source_code = fs::read_to_string(file_path)
            .map_err(|e| format!("Import error: File '{}' padhna sakiyena: {}", filename, e))?;
        
        // Lexical analysis
        let mut lexer = Lexer::new(source_code);
        let tokens = lexer.tokenize()
            .map_err(|e| format!("Import error '{}' ma: {}", filename, e))?;
        
        // Syntax analysis  
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()
            .map_err(|e| format!("Import error '{}' ma: {}", filename, e))?;
        
        // Execute the imported module in current environment
        let result = self.interpret_with_control(&ast)
            .map_err(|e| format!("Runtime error imported file '{}' ma: {}", filename, e));
        
        // Remove from import stack and mark as imported
        self.importing_stack.pop();
        self.imported_modules.insert(filename.to_string(), true);
        
        result?;
        Ok(())
    }
}