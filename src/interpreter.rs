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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ASTNode;

    #[test]
    fn test_division_by_zero() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Number("10".to_string())),
            "/".to_string(),
            Box::new(ASTNode::Number("0".to_string())),
        );
        let result = interp.interpret(&ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Division by zero"));
    }

    #[test]
    fn test_modulo_by_zero() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Number("10".to_string())),
            "%".to_string(),
            Box::new(ASTNode::Number("0".to_string())),
        );
        let result = interp.interpret(&ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Modulo by zero"));
    }

    #[test]
    fn test_addition() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Number("5".to_string())),
            "+".to_string(),
            Box::new(ASTNode::Number("3".to_string())),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    #[test]
    fn test_subtraction() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Number("10".to_string())),
            "-".to_string(),
            Box::new(ASTNode::Number("3".to_string())),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Number(7.0));
    }

    #[test]
    fn test_multiplication() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Number("5".to_string())),
            "*".to_string(),
            Box::new(ASTNode::Number("3".to_string())),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Number(15.0));
    }

    #[test]
    fn test_division() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Number("10".to_string())),
            "/".to_string(),
            Box::new(ASTNode::Number("2".to_string())),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_modulo() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Number("10".to_string())),
            "%".to_string(),
            Box::new(ASTNode::Number("3".to_string())),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Number(1.0));
    }

    #[test]
    fn test_comparison_greater_than() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Number("5".to_string())),
            ">".to_string(),
            Box::new(ASTNode::Number("3".to_string())),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_comparison_less_than() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Number("3".to_string())),
            "<".to_string(),
            Box::new(ASTNode::Number("5".to_string())),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_comparison_equality() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Number("5".to_string())),
            "==".to_string(),
            Box::new(ASTNode::Number("5".to_string())),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_comparison_not_equal() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Number("5".to_string())),
            "!=".to_string(),
            Box::new(ASTNode::Number("3".to_string())),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_string_concatenation() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::String("hello".to_string())),
            "+".to_string(),
            Box::new(ASTNode::String(" world".to_string())),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::String("hello world".to_string()));
    }

    #[test]
    fn test_string_number_concatenation() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::String("number: ".to_string())),
            "+".to_string(),
            Box::new(ASTNode::Number("42".to_string())),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::String("number: 42".to_string()));
    }

    #[test]
    fn test_logical_and_true() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Boolean(true)),
            "ra".to_string(),
            Box::new(ASTNode::Boolean(true)),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_logical_and_false() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Boolean(true)),
            "ra".to_string(),
            Box::new(ASTNode::Boolean(false)),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_logical_or_true() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Boolean(true)),
            "wa".to_string(),
            Box::new(ASTNode::Boolean(false)),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_logical_or_false() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_binary_op(
            Box::new(ASTNode::Boolean(false)),
            "wa".to_string(),
            Box::new(ASTNode::Boolean(false)),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_unary_not() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_unary_op(
            "hoina".to_string(),
            Box::new(ASTNode::Boolean(true)),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Boolean(false));
    }

    #[test]
    fn test_unary_minus() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_unary_op(
            "-".to_string(),
            Box::new(ASTNode::Number("5".to_string())),
        );
        let result = interp.interpret(&ast).unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    #[test]
    fn test_var_declaration_and_access() {
        let mut interp = Interpreter::new();
        let program = ASTNode::new_program(vec![
            Box::new(ASTNode::new_var_declaration(
                "x".to_string(),
                None,
                Box::new(ASTNode::Number("42".to_string())),
            )),
            Box::new(ASTNode::Identifier("x".to_string())),
        ]);
        let result = interp.interpret(&program).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_undefined_variable() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::Identifier("undefined".to_string());
        let result = interp.interpret(&ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_assignment() {
        let mut interp = Interpreter::new();
        let program = ASTNode::new_program(vec![
            Box::new(ASTNode::new_var_declaration(
                "x".to_string(),
                None,
                Box::new(ASTNode::Number("10".to_string())),
            )),
            Box::new(ASTNode::new_assignment(
                "x".to_string(),
                Box::new(ASTNode::Number("20".to_string())),
            )),
            Box::new(ASTNode::Identifier("x".to_string())),
        ]);
        let result = interp.interpret(&program).unwrap();
        assert_eq!(result, Value::Number(20.0));
    }

    #[test]
    fn test_list_literal() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_list_literal(vec![
            Box::new(ASTNode::Number("1".to_string())),
            Box::new(ASTNode::Number("2".to_string())),
            Box::new(ASTNode::Number("3".to_string())),
        ]);
        let result = interp.interpret(&ast).unwrap();
        match result {
            Value::List(list) => {
                assert_eq!(list.len(), 3);
                assert_eq!(list[0], Value::Number(1.0));
                assert_eq!(list[1], Value::Number(2.0));
                assert_eq!(list[2], Value::Number(3.0));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_list_index_access() {
        let mut interp = Interpreter::new();
        let program = ASTNode::new_program(vec![
            Box::new(ASTNode::new_var_declaration(
                "list".to_string(),
                None,
                Box::new(ASTNode::new_list_literal(vec![
                    Box::new(ASTNode::Number("10".to_string())),
                    Box::new(ASTNode::Number("20".to_string())),
                ])),
            )),
            Box::new(ASTNode::new_index_access(
                Box::new(ASTNode::Identifier("list".to_string())),
                Box::new(ASTNode::Number("0".to_string())),
            )),
        ]);
        let result = interp.interpret(&program).unwrap();
        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn test_list_index_out_of_bounds() {
        let mut interp = Interpreter::new();
        let program = ASTNode::new_program(vec![
            Box::new(ASTNode::new_var_declaration(
                "list".to_string(),
                None,
                Box::new(ASTNode::new_list_literal(vec![
                    Box::new(ASTNode::Number("10".to_string())),
                ])),
            )),
            Box::new(ASTNode::new_index_access(
                Box::new(ASTNode::Identifier("list".to_string())),
                Box::new(ASTNode::Number("5".to_string())),
            )),
        ]);
        let result = interp.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("out of bounds"));
    }

    #[test]
    fn test_dictionary_literal() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_dictionary_literal(vec![
            ("key".to_string(), Box::new(ASTNode::Number("42".to_string()))),
        ]);
        let result = interp.interpret(&ast).unwrap();
        match result {
            Value::Dictionary(dict) => {
                assert_eq!(dict.len(), 1);
                assert_eq!(dict.get("key"), Some(&Value::Number(42.0)));
            }
            _ => panic!("Expected dictionary"),
        }
    }

    #[test]
    fn test_dictionary_key_not_found() {
        let mut interp = Interpreter::new();
        let program = ASTNode::new_program(vec![
            Box::new(ASTNode::new_var_declaration(
                "dict".to_string(),
                None,
                Box::new(ASTNode::new_dictionary_literal(vec![
                    ("key".to_string(), Box::new(ASTNode::Number("42".to_string()))),
                ])),
            )),
            Box::new(ASTNode::new_index_access(
                Box::new(ASTNode::Identifier("dict".to_string())),
                Box::new(ASTNode::String("missing".to_string())),
            )),
        ]);
        let result = interp.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_break_outside_loop() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::Break;
        let result = interp.interpret(&ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Break statement outside loop"));
    }

    #[test]
    fn test_continue_outside_loop() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::Continue;
        let result = interp.interpret(&ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Continue statement outside loop"));
    }

    #[test]
    fn test_if_statement_true_branch() {
        let mut interp = Interpreter::new();
        let program = ASTNode::new_program(vec![
            Box::new(ASTNode::new_var_declaration(
                "x".to_string(),
                None,
                Box::new(ASTNode::Number("0".to_string())),
            )),
            Box::new(ASTNode::new_if_statement(
                Box::new(ASTNode::Boolean(true)),
                vec![Box::new(ASTNode::new_assignment(
                    "x".to_string(),
                    Box::new(ASTNode::Number("1".to_string())),
                ))],
                None,
            )),
            Box::new(ASTNode::Identifier("x".to_string())),
        ]);
        let result = interp.interpret(&program).unwrap();
        assert_eq!(result, Value::Number(1.0));
    }

    #[test]
    fn test_if_statement_false_branch() {
        let mut interp = Interpreter::new();
        let program = ASTNode::new_program(vec![
            Box::new(ASTNode::new_var_declaration(
                "x".to_string(),
                None,
                Box::new(ASTNode::Number("0".to_string())),
            )),
            Box::new(ASTNode::new_if_statement(
                Box::new(ASTNode::Boolean(false)),
                vec![Box::new(ASTNode::new_assignment(
                    "x".to_string(),
                    Box::new(ASTNode::Number("1".to_string())),
                ))],
                Some(vec![Box::new(ASTNode::new_assignment(
                    "x".to_string(),
                    Box::new(ASTNode::Number("2".to_string())),
                ))]),
            )),
            Box::new(ASTNode::Identifier("x".to_string())),
        ]);
        let result = interp.interpret(&program).unwrap();
        assert_eq!(result, Value::Number(2.0));
    }

    #[test]
    fn test_function_declaration_and_call() {
        let mut interp = Interpreter::new();
        let program = ASTNode::new_program(vec![
            Box::new(ASTNode::new_function_declaration(
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                vec![Box::new(ASTNode::Return(Box::new(ASTNode::new_binary_op(
                    Box::new(ASTNode::Identifier("a".to_string())),
                    "+".to_string(),
                    Box::new(ASTNode::Identifier("b".to_string())),
                ))))],
            )),
            Box::new(ASTNode::new_function_call(
                "add".to_string(),
                vec![
                    Box::new(ASTNode::Number("5".to_string())),
                    Box::new(ASTNode::Number("3".to_string())),
                ],
            )),
        ]);
        let result = interp.interpret(&program).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    #[test]
    fn test_undefined_function() {
        let mut interp = Interpreter::new();
        let ast = ASTNode::new_function_call("undefined".to_string(), vec![]);
        let result = interp.interpret(&ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined function"));
    }

    #[test]
    fn test_function_wrong_argument_count() {
        let mut interp = Interpreter::new();
        let program = ASTNode::new_program(vec![
            Box::new(ASTNode::new_function_declaration(
                "add".to_string(),
                vec!["a".to_string(), "b".to_string()],
                vec![Box::new(ASTNode::Return(Box::new(ASTNode::Number("0".to_string()))))],
            )),
            Box::new(ASTNode::new_function_call(
                "add".to_string(),
                vec![Box::new(ASTNode::Number("5".to_string()))],
            )),
        ]);
        let result = interp.interpret(&program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects"));
    }
}