# Nepali Slang Programming Language - Compiler Specification

## Project Overview
Build an interpreter for a programming language that uses Nepali Gen-Z slang keywords. The interpreter will directly execute Nepali slang code without converting it to another language.

**Project Name:** Khukuri: A Nepali Interpreter  
**Implementation Language:** Rust  
**Execution Model:** Direct interpretation (no intermediate code generation)  
**Purpose:** Fun project to make programming accessible in Nepali slang

---

## Language Syntax Specification

### 1. Syntax Style
- Mix of Python and C# style
- Uses braces `{}` for code blocks
- No colons before braces
- No semicolons required (optional)
- Optional type hints
- Indentation recommended but not enforced (braces define scope)

### 2. Keywords and Their Meanings

| Nepali Keyword | English Equivalent | Usage |
|----------------|-------------------|-------|
| `maanau` | let/var | Variable declaration |
| `yedi` | if | Conditional statement |
| `bhane` | then | Part of if statement |
| `natra` | else | Else statement |
| `kina bhane` | elif | Else if |
| `jaba samma` | while | While loop |
| `pratyek` | for each | For each loop |
| `kaam` | function | Function declaration |
| `pathau` | return | Return statement |
| `bhan` | print | Output/print |
| `sodha` | input | Input from user |
| `rok` | break | Break loop |
| `jane` | continue | Continue loop |
| `ra` | and | Logical AND |
| `wa` | or | Logical OR |
| `hoina` | not | Logical NOT |
| `sahi` | true | Boolean true |
| `galat` | false | Boolean false |

### 3. Operators
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `>`, `<`, `>=`, `<=`
- Assignment: `=`
- Logical: `ra` (and), `wa` (or), `hoina` (not)

### 4. Data Types
- Numbers (integers and floats)
- Strings (double quotes only: `"text"`)
- Booleans: `sahi`, `galat`
- Optional type hints: `maanau naam: string = "Ram"`

### 5. Comments
- Single line: `// comment`
- No multi-line comments in initial version

---

## Example Programs

### Example 1: Variables and Basic Math
```nepali
maanau price = 500
maanau discount = 50
maanau final_price = price - discount

yedi final_price < 400 bhane {
    bhan "Sasto cha, kinnu parchha!"
} natra {
    bhan "Mehango cha bro"
}
```

**Expected Output when executed:**
```
Sasto cha, kinnu parchha!
```

### Example 2: Conditionals with Nested Logic
```nepali
maanau marks = 75
maanau attendance = 80

yedi marks >= 80 ra attendance >= 75 bhane {
    bhan "A grade! Khatra performance bro"
} natra {
    yedi marks >= 60 ra attendance >= 70 bhane {
        bhan "B grade, thik cha"
    } natra {
        yedi marks >= 40 bhane {
            bhan "Pass bhayo, aaile ramro gara"
        } natra {
            bhan "Fail bro, mehnat gara"
        }
    }
}
```

**Expected Output when executed:**
```
B grade, thik cha
```

### Example 3: Functions and Loops
```nepali
kaam is_even(number) {
    yedi number % 2 == 0 bhane {
        pathau sahi
    } natra {
        pathau galat
    }
}

maanau count = 0
maanau i = 1

jaba samma i <= 10 {
    yedi is_even(i) bhane {
        bhan i
        count = count + 1
    }
    i = i + 1
}

bhan "Total even numbers: "
bhan count
```

**Expected Output when executed:**
```
2
4
6
8
10
Total even numbers: 
5
```

---

## Compiler Architecture

### Phase 1: Lexical Analysis (Lexer/Tokenizer)

**Purpose:** Convert source code into tokens

**Token Types:**
```rust
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
    Comma,            // ,
    Colon,            // : (for optional type hints)
    
    // Special
    Newline,          // \n
    EOF,              // End of file
}
```

**Lexer Structure:**
```rust
pub struct Lexer {
    code: Vec<char>,
    pos: usize,
    current_char: Option<char>,
    keywords: Vec<&'static str>,
}

impl Lexer {
    pub fn new(code: String) -> Self { ... }
    fn advance(&mut self) { ... }
    fn peek(&self) -> Option<char> { ... }
    fn skip_whitespace(&mut self) { ... }
    fn skip_comment(&mut self) { ... }
    fn read_number(&mut self) -> String { ... }
    fn read_identifier(&mut self) -> String { ... }
    fn read_string(&mut self) -> String { ... }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> { ... }
}
```

**Key Lexer Behaviors:**
1. Skip whitespace and tabs
2. Handle comments starting with `//`
3. Recognize multi-character operators (`==`, `!=`, `>=`, `<=`)
4. Distinguish keywords from identifiers
5. Handle string literals with double quotes
6. Support numbers (integers and floats)

---

### Phase 2: Syntax Analysis (Parser)

**Purpose:** Convert tokens into an Abstract Syntax Tree (AST)

**AST Node Types:**
```rust
pub enum ASTNode {
    // Statements
    Program(Vec<Box<ASTNode>>),
    VarDeclaration {
        name: String,
        type_hint: Option<String>,
        value: Box<ASTNode>,
    },
    Assignment {
        name: String,
        value: Box<ASTNode>,
    },
    IfStatement {
        condition: Box<ASTNode>,
        then_block: Vec<Box<ASTNode>>,
        else_block: Option<Vec<Box<ASTNode>>>,
    },
    WhileLoop {
        condition: Box<ASTNode>,
        body: Vec<Box<ASTNode>>,
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Vec<Box<ASTNode>>,
    },
    Return(Box<ASTNode>),
    Print(Box<ASTNode>),
    
    // Expressions
    BinaryOp {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    UnaryOp {
        operator: String,
        operand: Box<ASTNode>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Box<ASTNode>>,
    },
    Identifier(String),
    Number(String),
    String(String),
    Boolean(bool),
}
```

**Parser Structure:**
```rust
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { ... }
    fn advance(&mut self) { ... }
    fn peek(&self) -> Option<&Token> { ... }
    fn expect(&mut self, token_type: TokenType) -> Result<(), String> { ... }
    
    // Parsing methods
    pub fn parse(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_statement(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_var_declaration(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_if_statement(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_while_loop(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_function_declaration(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_expression(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_logical_or(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_logical_and(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_comparison(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_addition(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_multiplication(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_unary(&mut self) -> Result<ASTNode, String> { ... }
    fn parse_primary(&mut self) -> Result<ASTNode, String> { ... }
}
```

**Grammar Rules (in EBNF-like notation):**
```
program = { statement }

statement = var_declaration
          | assignment
          | if_statement
          | while_loop
          | function_declaration
          | return_statement
          | print_statement
          | expression

var_declaration = "maanau" IDENTIFIER [ ":" TYPE ] "=" expression

assignment = IDENTIFIER "=" expression

if_statement = "yedi" expression "bhane" "{" { statement } "}" 
               [ "natra" "{" { statement } "}" ]

while_loop = "jaba" "samma" expression "{" { statement } "}"

function_declaration = "kaam" IDENTIFIER "(" [ parameter_list ] ")" 
                       "{" { statement } "}"

return_statement = "pathau" expression

print_statement = "bhan" expression

expression = logical_or

logical_or = logical_and { "wa" logical_and }

logical_and = comparison { "ra" comparison }

comparison = addition { ( "==" | "!=" | ">" | "<" | ">=" | "<=" ) addition }

addition = multiplication { ( "+" | "-" ) multiplication }

multiplication = unary { ( "*" | "/" | "%" ) unary }

unary = [ "hoina" ] primary

primary = NUMBER
        | STRING
        | "sahi"
        | "galat"
        | IDENTIFIER [ "(" [ argument_list ] ")" ]
        | "(" expression ")"
```

---

### Phase 3: Interpretation (Executor)

**Purpose:** Execute AST directly by evaluating each node

**Value Types:**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => s.clone(),
            Value::Boolean(b) => if *b { "sahi" } else { "galat" }.to_string(),
            Value::Null => "null".to_string(),
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }
}
```

**Environment (Variable Storage):**
```rust
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
}
```

**Interpreter Structure:**
```rust
pub struct Interpreter {
    environment: Environment,
    functions: HashMap<String, (Vec<String>, Vec<Box<ASTNode>>)>, // (params, body)
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
            functions: HashMap::new(),
        }
    }
    
    pub fn interpret(&mut self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::Program(statements) => {
                let mut result = Value::Null;
                for stmt in statements {
                    result = self.interpret(stmt)?;
                }
                Ok(result)
            }
            
            ASTNode::VarDeclaration { name, value, .. } => {
                let val = self.interpret(value)?;
                self.environment.define(name.clone(), val);
                Ok(Value::Null)
            }
            
            ASTNode::Assignment { name, value } => {
                let val = self.interpret(value)?;
                self.environment.set(name, val)?;
                Ok(Value::Null)
            }
            
            ASTNode::IfStatement { condition, then_block, else_block } => {
                let cond_value = self.interpret(condition)?;
                
                if cond_value.is_truthy() {
                    self.environment.push_scope();
                    let mut result = Value::Null;
                    for stmt in then_block {
                        result = self.interpret(stmt)?;
                    }
                    self.environment.pop_scope();
                    Ok(result)
                } else if let Some(else_stmts) = else_block {
                    self.environment.push_scope();
                    let mut result = Value::Null;
                    for stmt in else_stmts {
                        result = self.interpret(stmt)?;
                    }
                    self.environment.pop_scope();
                    Ok(result)
                } else {
                    Ok(Value::Null)
                }
            }
            
            ASTNode::WhileLoop { condition, body } => {
                loop {
                    let cond_value = self.interpret(condition)?;
                    if !cond_value.is_truthy() {
                        break;
                    }
                    
                    self.environment.push_scope();
                    for stmt in body {
                        self.interpret(stmt)?;
                    }
                    self.environment.pop_scope();
                }
                Ok(Value::Null)
            }
            
            ASTNode::FunctionDeclaration { name, parameters, body } => {
                self.functions.insert(
                    name.clone(),
                    (parameters.clone(), body.clone())
                );
                Ok(Value::Null)
            }
            
            ASTNode::Return(expr) => {
                self.interpret(expr)
            }
            
            ASTNode::Print(expr) => {
                let value = self.interpret(expr)?;
                println!("{}", value.to_string());
                Ok(Value::Null)
            }
            
            ASTNode::BinaryOp { left, operator, right } => {
                self.eval_binary_op(left, operator, right)
            }
            
            ASTNode::UnaryOp { operator, operand } => {
                self.eval_unary_op(operator, operand)
            }
            
            ASTNode::FunctionCall { name, arguments } => {
                self.call_function(name, arguments)
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
        }
    }
    
    fn eval_binary_op(&mut self, left: &ASTNode, operator: &str, right: &ASTNode) 
        -> Result<Value, String> {
        let left_val = self.interpret(left)?;
        let right_val = self.interpret(right)?;
        
        match (left_val, operator, right_val) {
            (Value::Number(l), "+", Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::Number(l), "-", Value::Number(r)) => Ok(Value::Number(l - r)),
            (Value::Number(l), "*", Value::Number(r)) => Ok(Value::Number(l * r)),
            (Value::Number(l), "/", Value::Number(r)) => {
                if r == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Number(l / r))
                }
            }
            (Value::Number(l), "%", Value::Number(r)) => Ok(Value::Number(l % r)),
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
            
            _ => Err(format!("Invalid operation: {:?} {} {:?}", left_val, operator, right_val))
        }
    }
    
    fn eval_unary_op(&mut self, operator: &str, operand: &ASTNode) 
        -> Result<Value, String> {
        let val = self.interpret(operand)?;
        
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
            arg_values.push(self.interpret(arg)?);
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
            result = self.interpret(stmt)?;
        }
        
        // Restore scope
        self.environment.pop_scope();
        
        Ok(result)
    }
}
```

---

## Implementation Guide

### Step 1: Project Setup
```bash
cargo new nepali-compiler
cd nepali-compiler
```

Add dependencies to `Cargo.toml`:
```toml
[package]
name = "nepali-compiler"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### Step 2: Module Structure
```
src/
├── main.rs           # Entry point, CLI interface
├── lexer.rs          # Lexical analyzer
├── parser.rs         # Syntax analyzer
├── ast.rs            # AST node definitions
├── interpreter.rs    # Interpreter/Executor
├── environment.rs    # Variable storage and scoping
├── value.rs          # Runtime value types
└── error.rs          # Error handling
```

### Step 3: Implementation Order

1. **Define Value and Token structures** (`value.rs`, `ast.rs`)
2. **Implement Lexer** (`lexer.rs`)
   - Test with simple inputs
   - Handle all token types
3. **Implement Parser** (`parser.rs`)
   - Start with expressions
   - Add statements
   - Add control flow
   - Add functions
4. **Implement Environment** (`environment.rs`)
   - Variable storage
   - Scoping rules
5. **Implement Interpreter** (`interpreter.rs`)
   - Execute each AST node type
   - Handle function calls
   - Manage scope
6. **Build CLI** (`main.rs`)
   - Read `.nep` files
   - Execute directly
   - Support REPL mode (optional)
7. **Error Handling** (`error.rs`)
   - Lexer errors (invalid characters)
   - Parser errors (syntax errors)
   - Runtime errors (undefined variables, division by zero)
   - Helpful error messages with line numbers

### Step 4: Testing Strategy

Create test files in `tests/` directory:

**tests/test_lexer.rs:**
```rust
#[cfg(test)]
mod tests {
    use nepali_compiler::lexer::*;
    
    #[test]
    fn test_variable_declaration() {
        let code = "maanau x = 5".to_string();
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 5); // maanau, x, =, 5, EOF
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        assert_eq!(tokens[0].value, "maanau");
    }
    
    // Add more tests...
}
```

**Create test .nep files:**
```
tests/
├── simple.nep
├── conditionals.nep
├── loops.nep
└── functions.nep
```

---

## CLI Interface

### Usage
```bash
# Run a Nepali file directly
cargo run -- program.nep

# Or with the built executable
./nepali-interpreter program.nep

# Interactive REPL mode (optional feature)
./nepali-interpreter --repl
```

### Main.rs Structure
```rust
use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod ast;
mod interpreter;
mod environment;
mod value;
mod error;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: nepali-interpreter <program.nep>");
        eprintln!("   or: nepali-interpreter --repl");
        process::exit(1);
    }
    
    if args[1] == "--repl" {
        run_repl();
        return;
    }
    
    let input_file = &args[1];
    
    // Read source code
    let source_code = fs::read_to_string(input_file)
        .expect("Failed to read input file");
    
    // Execute the program
    if let Err(e) = run_program(&source_code) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run_program(source_code: &str) -> Result<(), String> {
    // Lexical analysis
    let mut lexer = lexer::Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;
    
    // Syntax analysis
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse()
        .map_err(|e| format!("Parser error: {}", e))?;
    
    // Interpret and execute
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(&ast)
        .map_err(|e| format!("Runtime error: {}", e))?;
    
    Ok(())
}

fn run_repl() {
    use std::io::{self, Write};
    
    println!("Nepali Interpreter REPL");
    println!("Type 'exit' to quit\n");
    
    let mut interpreter = interpreter::Interpreter::new();
    
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        if input == "exit" {
            break;
        }
        
        match run_line(&mut interpreter, input) {
            Ok(value) => {
                if value != value::Value::Null {
                    println!("{}", value.to_string());
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn run_line(interpreter: &mut interpreter::Interpreter, line: &str) 
    -> Result<value::Value, String> {
    let mut lexer = lexer::Lexer::new(line.to_string());
    let tokens = lexer.tokenize()?;
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse()?;
    interpreter.interpret(&ast)
}
```

---

## Error Handling

### Error Types
```rust
#[derive(Debug)]
pub enum CompilerError {
    LexerError { message: String, line: usize, column: usize },
    ParserError { message: String, line: usize, column: usize },
    RuntimeError { message: String, line: usize },
}

impl CompilerError {
    pub fn display(&self, source_code: &str) {
        match self {
            CompilerError::LexerError { message, line, column } => {
                eprintln!("Lexer Error at line {}, column {}: {}", line, column, message);
                // Show the line of code with error
            }
            CompilerError::ParserError { message, line, column } => {
                eprintln!("Syntax Error at line {}, column {}: {}", line, column, message);
            }
            CompilerError::RuntimeError { message, line } => {
                eprintln!("Runtime Error at line {}: {}", line, message);
            }
        }
    }
}
```

### Common Runtime Errors to Handle:
- **Undefined variables:** Variable used before declaration
- **Undefined functions:** Function called but not defined
- **Type errors:** Invalid operations (e.g., "hello" - 5)
- **Division by zero:** Attempting to divide by 0
- **Wrong argument count:** Function called with incorrect number of arguments
- **Return outside function:** Using `pathau` outside a function body

---

## Future Enhancements (Optional)

1. **More data structures:**
   - Lists: `maanau numbers = [1, 2, 3]`
   - Dictionaries: `maanau person = {"naam": "Ram"}`

2. **For loops:**
   - `pratyek i in range(10) { ... }`

3. **Class support:**
   - `class Person { ... }`

4. **Import system:**
   - `import "math.nep"`

5. **Better error messages:**
   - Suggestions for typos
   - "Did you mean...?" hints

6. **REPL (Interactive mode):**
   - Type Nepali code interactively

7. **Standard library:**
   - Built-in functions in Nepali
   - File I/O operations

8. **Syntax highlighting:**
   - VS Code extension
   - Vim plugin

---

## Complete Keyword Reference

```rust
// In lexer.rs
let keywords = vec![
    "maanau",      // Variable declaration
    "yedi",        // If
    "bhane",       // Then
    "natra",       // Else
    "kina bhane",  // Elif (handle as two tokens or special case)
    "jaba",        // While (part 1)
    "samma",       // While (part 2)
    "pratyek",     // For each
    "kaam",        // Function
    "pathau",      // Return
    "bhan",        // Print
    "sodha",       // Input
    "rok",         // Break
    "jane",        // Continue
    "ra",          // And
    "wa",          // Or
    "hoina",       // Not
    "sahi",        // True
    "galat",       // False
];
```

---

## Testing Checklist

Before considering the compiler complete, test these scenarios:

- [ ] Variable declarations with different types
- [ ] Arithmetic operations (+, -, *, /, %)
- [ ] Comparison operators (==, !=, >, <, >=, <=)
- [ ] Logical operators (ra, wa, hoina)
- [ ] If statements
- [ ] If-else statements
- [ ] Nested if-else
- [ ] While loops
- [ ] Nested loops
- [ ] Function declarations
- [ ] Function calls
- [ ] Recursive functions
- [ ] Return statements
- [ ] Print statements
- [ ] Comments (single-line)
- [ ] String literals
- [ ] Numbers (integers and floats)
- [ ] Boolean values
- [ ] Complex expressions with operator precedence
- [ ] Error handling for invalid syntax
- [ ] Error handling for undefined variables
- [ ] Multi-line programs
- [ ] Programs with multiple functions

---

## Example Test Cases

### Test 1: Fibonacci
```nepali
kaam fibonacci(n) {
    yedi n <= 1 bhane {
        pathau n
    }
    pathau fibonacci(n - 1) + fibonacci(n - 2)
}

maanau result = fibonacci(10)
bhan result
```

### Test 2: Factorial
```nepali
kaam factorial(n) {
    yedi n == 0 wa n == 1 bhane {
        pathau 1
    }
    pathau n * factorial(n - 1)
}

maanau num = 5
bhan factorial(num)
```

### Test 3: Even/Odd Checker
```nepali
maanau number = 42

yedi number % 2 == 0 bhane {
    bhan "Even number ho"
} natra {
    bhan "Odd number ho"
}
```

---

## Performance Considerations

1. **Lexer:** Should be fast - O(n) where n is input length
2. **Parser:** Recursive descent - O(n) for most cases
3. **Code Generator:** Simple traversal - O(n) where n is AST nodes
4. **Memory:** Keep token and AST representations minimal

---

## Documentation Requirements

1. **README.md:**
   - Project description
   - Installation instructions
   - Usage examples
   - Language syntax guide

2. **CONTRIBUTING.md:**
   - How to add new keywords
   - How to extend grammar
   - Testing guidelines

3. **EXAMPLES.md:**
   - Sample programs
   - Common patterns
   - Best practices

---

## Success Criteria

The interpreter is complete when:
1. All 3 example programs execute correctly and produce expected output
2. All test cases pass
3. Runtime error messages are helpful
4. Variable scoping works correctly
5. Functions can be called recursively
6. Code is well-documented
7. README has clear usage instructions
8. REPL mode works (optional but recommended)

---

## GitHub Copilot Prompts

Use these prompts when working with Copilot:

1. "Implement the Lexer struct with tokenize method following the specification"
2. "Create AST node definitions for all statement and expression types"
3. "Implement recursive descent parser for Nepali language grammar"
4. "Create Value enum to represent runtime values (Number, String, Boolean, Null)"
5. "Implement Environment struct for variable storage with scoping support"
6. "Build Interpreter that executes AST nodes directly"
7. "Add error handling with line and column numbers for runtime errors"
8. "Create tests for lexer tokenization"
9. "Implement CLI interface for direct file execution"
10. "Handle operator precedence in expression parsing"
11. "Add support for function calls and recursion"
12. "Implement REPL mode for interactive execution"

---

## Quick Start for Copilot

**Prompt to get started:**
"Create a Rust interpreter for a programming language called Nepali that uses these keywords: maanau (var), yedi (if), bhane (then), natra (else), jaba samma (while), kaam (function), pathau (return), bhan (print), ra (and), wa (or). The language uses C-style braces {} for blocks and directly executes code without transpilation. Implement: 1) Lexer with tokenization, 2) Parser with AST generation, 3) Interpreter that executes the AST directly with proper variable scoping. Include support for functions, recursion, and a REPL mode. Start with the lexer module."

---

END OF SPECIFICATION