use crate::value::Value;
use std::collections::HashMap;

pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()], // Global scope
        }
    }
    
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    
    pub fn define(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }
    
    pub fn get(&self, name: &str) -> Option<Value> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }
    
    pub fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(format!("Undefined variable: {}", name))
    }
    
    pub fn current_scope_size(&self) -> usize {
        self.scopes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_and_get_variable() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(42.0));
        assert_eq!(env.get("x"), Some(Value::Number(42.0)));
    }

    #[test]
    fn test_undefined_variable() {
        let env = Environment::new();
        assert_eq!(env.get("undefined"), None);
    }

    #[test]
    fn test_set_existing_variable() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(10.0));
        let result = env.set("x", Value::Number(20.0));
        assert!(result.is_ok());
        assert_eq!(env.get("x"), Some(Value::Number(20.0)));
    }

    #[test]
    fn test_set_undefined_variable() {
        let mut env = Environment::new();
        let result = env.set("x", Value::Number(10.0));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_nested_scopes() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(1.0));

        env.push_scope();
        env.define("y".to_string(), Value::Number(2.0));

        // Both variables should be accessible
        assert_eq!(env.get("x"), Some(Value::Number(1.0)));
        assert_eq!(env.get("y"), Some(Value::Number(2.0)));

        env.pop_scope();

        // Only x should be accessible now
        assert_eq!(env.get("x"), Some(Value::Number(1.0)));
        assert_eq!(env.get("y"), None);
    }

    #[test]
    fn test_variable_shadowing() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(1.0));

        env.push_scope();
        env.define("x".to_string(), Value::Number(2.0));

        // Inner scope shadows outer scope
        assert_eq!(env.get("x"), Some(Value::Number(2.0)));

        env.pop_scope();

        // Outer scope value restored
        assert_eq!(env.get("x"), Some(Value::Number(1.0)));
    }

    #[test]
    fn test_set_in_nested_scope() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(1.0));

        env.push_scope();
        let result = env.set("x", Value::Number(2.0));
        assert!(result.is_ok());

        // Value should be updated
        assert_eq!(env.get("x"), Some(Value::Number(2.0)));

        env.pop_scope();

        // Value should still be updated in outer scope
        assert_eq!(env.get("x"), Some(Value::Number(2.0)));
    }

    #[test]
    fn test_cannot_pop_global_scope() {
        let mut env = Environment::new();
        assert_eq!(env.current_scope_size(), 1);

        env.pop_scope();

        // Should still have global scope
        assert_eq!(env.current_scope_size(), 1);
    }

    #[test]
    fn test_multiple_scopes() {
        let mut env = Environment::new();
        env.define("a".to_string(), Value::Number(1.0));

        env.push_scope();
        env.define("b".to_string(), Value::Number(2.0));

        env.push_scope();
        env.define("c".to_string(), Value::Number(3.0));

        // All variables accessible
        assert_eq!(env.get("a"), Some(Value::Number(1.0)));
        assert_eq!(env.get("b"), Some(Value::Number(2.0)));
        assert_eq!(env.get("c"), Some(Value::Number(3.0)));

        env.pop_scope();

        // c is gone
        assert_eq!(env.get("c"), None);
        assert_eq!(env.get("b"), Some(Value::Number(2.0)));

        env.pop_scope();

        // b is gone
        assert_eq!(env.get("b"), None);
        assert_eq!(env.get("a"), Some(Value::Number(1.0)));
    }
}