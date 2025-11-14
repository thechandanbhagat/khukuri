use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Dictionary(HashMap<String, Value>),
    Null,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => s.clone(),
            Value::Boolean(b) => if *b { "sahi" } else { "galat" }.to_string(),
            Value::List(list) => {
                let items: Vec<String> = list.iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Dictionary(dict) => {
                let items: Vec<String> = dict.iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::Null => "null".to_string(),
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(list) => !list.is_empty(),
            Value::Dictionary(dict) => !dict.is_empty(),
        }
    }
    
    pub fn get_type(&self) -> &'static str {
        match self {
            Value::Number(_) => "Number",
            Value::String(_) => "String",
            Value::Boolean(_) => "Boolean",
            Value::List(_) => "List",
            Value::Dictionary(_) => "Dictionary",
            Value::Null => "Null",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_string() {
        let val = Value::Number(42.0);
        assert_eq!(val.to_string(), "42");
    }

    #[test]
    fn test_float_to_string() {
        let val = Value::Number(3.14);
        assert_eq!(val.to_string(), "3.14");
    }

    #[test]
    fn test_string_to_string() {
        let val = Value::String("hello".to_string());
        assert_eq!(val.to_string(), "hello");
    }

    #[test]
    fn test_boolean_true_to_string() {
        let val = Value::Boolean(true);
        assert_eq!(val.to_string(), "sahi");
    }

    #[test]
    fn test_boolean_false_to_string() {
        let val = Value::Boolean(false);
        assert_eq!(val.to_string(), "galat");
    }

    #[test]
    fn test_null_to_string() {
        let val = Value::Null;
        assert_eq!(val.to_string(), "null");
    }

    #[test]
    fn test_list_to_string() {
        let val = Value::List(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);
        assert_eq!(val.to_string(), "[1, 2, 3]");
    }

    #[test]
    fn test_empty_list_to_string() {
        let val = Value::List(vec![]);
        assert_eq!(val.to_string(), "[]");
    }

    #[test]
    fn test_truthiness_boolean_true() {
        let val = Value::Boolean(true);
        assert!(val.is_truthy());
    }

    #[test]
    fn test_truthiness_boolean_false() {
        let val = Value::Boolean(false);
        assert!(!val.is_truthy());
    }

    #[test]
    fn test_truthiness_null() {
        let val = Value::Null;
        assert!(!val.is_truthy());
    }

    #[test]
    fn test_truthiness_number_zero() {
        let val = Value::Number(0.0);
        assert!(!val.is_truthy());
    }

    #[test]
    fn test_truthiness_number_non_zero() {
        let val = Value::Number(42.0);
        assert!(val.is_truthy());
    }

    #[test]
    fn test_truthiness_empty_string() {
        let val = Value::String("".to_string());
        assert!(!val.is_truthy());
    }

    #[test]
    fn test_truthiness_non_empty_string() {
        let val = Value::String("hello".to_string());
        assert!(val.is_truthy());
    }

    #[test]
    fn test_truthiness_empty_list() {
        let val = Value::List(vec![]);
        assert!(!val.is_truthy());
    }

    #[test]
    fn test_truthiness_non_empty_list() {
        let val = Value::List(vec![Value::Number(1.0)]);
        assert!(val.is_truthy());
    }

    #[test]
    fn test_get_type() {
        assert_eq!(Value::Number(42.0).get_type(), "Number");
        assert_eq!(Value::String("test".to_string()).get_type(), "String");
        assert_eq!(Value::Boolean(true).get_type(), "Boolean");
        assert_eq!(Value::List(vec![]).get_type(), "List");
        assert_eq!(Value::Dictionary(HashMap::new()).get_type(), "Dictionary");
        assert_eq!(Value::Null.get_type(), "Null");
    }
}