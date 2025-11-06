#[derive(Debug)]
pub enum CompilerError {
    LexerError { message: String, line: usize, column: usize },
    ParserError { message: String, line: usize, column: usize },
    RuntimeError { message: String, line: usize },
}

impl CompilerError {
    pub fn display(&self, source_code: &str) {
        let lines: Vec<&str> = source_code.lines().collect();
        
        match self {
            CompilerError::LexerError { message, line, column } => {
                eprintln!("Lexer Error line {} ma, column {}: {}", line, column, message);
                if *line > 0 && *line <= lines.len() {
                    eprintln!("  {}", lines[*line - 1]);
                    eprintln!("  {}^", " ".repeat(*column - 1));
                }
            }
            CompilerError::ParserError { message, line, column } => {
                eprintln!("Syntax Error line {} ma, column {}: {}", line, column, message);
                if *line > 0 && *line <= lines.len() {
                    eprintln!("  {}", lines[*line - 1]);
                    eprintln!("  {}^", " ".repeat(*column - 1));
                }
            }
            CompilerError::RuntimeError { message, line } => {
                eprintln!("Runtime Error line {} ma: {}", line, message);
                if *line > 0 && *line <= lines.len() {
                    eprintln!("  {}", lines[*line - 1]);
                }
            }
        }
    }
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::LexerError { message, line, column } => {
                write!(f, "Lexer Error line {} ma, column {}: {}", line, column, message)
            }
            CompilerError::ParserError { message, line, column } => {
                write!(f, "Syntax Error line {} ma, column {}: {}", line, column, message)
            }
            CompilerError::RuntimeError { message, line } => {
                write!(f, "Runtime Error line {} ma: {}", line, message)
            }
        }
    }
}

impl std::error::Error for CompilerError {}