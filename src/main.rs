use std::env;
use std::fs;
use std::process;
use std::io::{self, Write};

mod token;
mod value;
mod lexer;
mod ast;
mod parser;
mod environment;
mod interpreter;
mod error;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use crate::value::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: khukuri <program.nep>");
        eprintln!("   wa: khukuri --repl");
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
    let mut lexer = Lexer::new(source_code.to_string());
    let tokens = lexer.tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;
    
    // Syntax analysis
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()
        .map_err(|e| format!("Parser error: {}", e))?;
    
    // Interpret and execute
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&ast)
        .map_err(|e| format!("Runtime error: {}", e))?;
    
    Ok(())
}

fn run_repl() {
    println!("Khukuri Interpreter REPL");
    println!("Nepali Gen-Z Programming Language");
    println!("'exit' type gara bandha garna\n");
    
    let mut interpreter = Interpreter::new();
    
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
                if value != Value::Null {
                    println!("{}", value.to_string());
                }
            }
            Err(e) => eprintln!("Error bhayo: {}", e),
        }
    }
}

fn run_line(interpreter: &mut Interpreter, line: &str) -> Result<Value, String> {
    let mut lexer = Lexer::new(line.to_string());
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    interpreter.interpret(&ast)
}