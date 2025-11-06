# Khukuri: A Nepali Gen-Z Slang Programming Language

Khukuri is a fun, educational interpreter for a programming language that uses Nepali Gen-Z slang keywords. The language combines familiar programming concepts with Nepali slang to make programming more accessible and entertaining for Nepali speakers.

## Features

- **Direct Interpretation**: No compilation to intermediate code - executes Nepali slang directly
- **Familiar Syntax**: C-style braces with Python/C# inspired syntax
- **Function Support**: First-class functions with recursion support
- **Variable Scoping**: Proper lexical scoping with nested environments
- **REPL Mode**: Interactive mode for testing code snippets
- **Nepali Gen-Z Output**: All console output and error messages in Nepali Gen-Z style
- **Import System**: Modular programming with `aayaat` keyword for code organization
- **Error Handling**: Clear error messages with line and column information in Nepali

## Language Keywords

| Nepali Keyword | English Equivalent | Usage |
|----------------|-------------------|-------|
| `maanau` | let/var | Variable declaration |
| `yedi` | if | Conditional statement |
| `bhane` | then | Part of if statement |
| `natra` | else | Else statement |
| `jaba samma` | while | While loop |
| `pratyek` | for each | For each loop |
| `ma` | in | In (for iteration) |
| `kaam` | function | Function declaration |
| `pathau` | return | Return statement |
| `bhan` | print | Output/print |
| `rok` | break | Break loop |
| `jane` | continue | Continue loop |
| `aayaat` | import | Import module |
| `ra` | and | Logical AND |
| `wa` | or | Logical OR |
| `hoina` | not | Logical NOT |
| `sahi` | true | Boolean true |
| `galat` | false | Boolean false |

## Installation

```bash
git clone <repository-url>
cd khukuri
cargo build --release
```

## Usage

### Run a Khukuri Program
```bash
cargo run -- program.nep
```

### Interactive REPL Mode
```bash
cargo run -- --repl
```

## Language Examples

### Variables and Basic Math
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

### Functions and Recursion
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

### Loops and Conditionals
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

### Lists and Dictionaries
```nepali
// Create and manipulate lists
maanau fruits = ["aam", "kera", "syau"]
bhan fruits[0]  // Access first element

fruits[1] = "naranghi"  // Modify element
bhan fruits

// Create and use dictionaries
maanau person = {"naam": "Ram", "umar": 25, "thau": "Kathmandu"}
bhan person["naam"]

person["kaam"] = "Engineer"  // Add new key-value
bhan person

// Nested structures
maanau students = [
    {"naam": "Sita", "marks": 85},
    {"naam": "Gita", "marks": 92}
]

bhan students[0]["naam"]  // Access nested values
```

### For-Each Loops
```nepali
// Iterate through lists
maanau numbers = [1, 2, 3, 4, 5]
pratyek num ma numbers {
    bhan num
}

// Iterate through strings (character by character)
maanau word = "Namaste"
pratyek char ma word {
    bhan char
}

// Iterate through dictionary keys
maanau person = {"naam": "Ram", "umar": 25}
pratyek key ma person {
    bhan key
    bhan person[key]  // Access the value
}

// Use in functions
kaam sum_list(items) {
    maanau total = 0
    pratyek item ma items {
        total = total + item
    }
    pathau total
}
```

### Break and Continue Statements
```nepali
// Break - exit loop early
maanau i = 1
jaba samma i <= 10 {
    yedi i == 5 bhane {
        bhan "Breaking at 5"
        rok  // Exit the loop
    }
    bhan i
    i = i + 1
}

// Continue - skip to next iteration
pratyek num ma [1, 2, 3, 4, 5] {
    yedi num % 2 == 0 bhane {
        jane  // Skip even numbers
    }
    bhan num  // Only prints odd numbers
}

// Works in nested loops
jaba samma outer <= 3 {
    pratyek item ma items {
        yedi item == target bhane {
            rok  // Break inner loop only
        }
    }
    outer = outer + 1
}
```

### Import System (Modular Programming)
```nepali
// math_utils.nep - Utility file
kaam square(x) {
    pathau x * x
}

kaam factorial(n) {
    yedi n <= 1 bhane {
        pathau 1
    }
    pathau n * factorial(n - 1)
}

maanau PI = 3.14159

// main.nep - Main program
aayaat "math_utils.nep"  // Import external module

maanau num = 5
bhan "Number:"
bhan num

bhan "Squared:"
bhan square(num)  // Use imported function

bhan "Factorial:"
bhan factorial(num)  // Use imported function

bhan "PI constant:"
bhan PI  // Use imported constant

// Multiple imports
aayaat "string_utils.nep"
aayaat "other_module.nep"
```

## Language Features

### Data Types
- **Numbers**: Integers and floating-point (`42`, `3.14`)
- **Strings**: Double-quoted text (`"Hello World"`)
- **Booleans**: `sahi` (true) and `galat` (false)
- **Lists**: Ordered collections (`[1, 2, 3]`, `["a", "b", "c"]`)
- **Dictionaries**: Key-value pairs (`{"key": "value", "age": 25}`)

### Operators
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`
- **Comparison**: `==`, `!=`, `>`, `<`, `>=`, `<=`
- **Logical**: `ra` (and), `wa` (or), `hoina` (not)
- **Assignment**: `=`

### Comments
- Single-line comments: `// This is a comment`

### Import System Features
- **Module Importing**: Use `aayaat "filename.nep"` to import external modules
- **Circular Import Protection**: Prevents infinite import loops with clear error messages
- **Shared Environment**: Imported modules share the same variable and function environment
- **Error Propagation**: Import errors are clearly reported with file context

## Project Structure

```
src/
├── main.rs          # CLI interface and REPL
├── token.rs         # Token definitions
├── value.rs         # Runtime value types
├── lexer.rs         # Lexical analyzer
├── ast.rs          # Abstract Syntax Tree
├── parser.rs       # Recursive descent parser
├── environment.rs  # Variable scoping
├── interpreter.rs  # AST executor
└── error.rs        # Error handling

examples/
├── basic_math.nep
├── conditionals.nep  
├── functions_loops.nep
├── fibonacci.nep
├── collections.nep
├── advanced_collections.nep
├── foreach_loops.nep
├── advanced_foreach.nep
├── break_continue.nep
├── advanced_break_continue.nep
├── import_demo.nep
├── simple_import.nep
├── math_utils.nep
└── string_utils.nep

editor-support/
├── README.md                    # Editor installation guide
├── vscode/                      # VS Code extension
│   ├── package.json
│   ├── khukuri.tmLanguage.json
│   └── language-configuration.json
├── vim/                         # Vim syntax support
│   ├── khukuri.vim
│   └── ftdetect/khukuri.vim
└── sublime-text/                # Sublime Text support
    ├── Khukuri.sublime-syntax
    └── Khukuri.sublime-settings
```

## Architecture

The interpreter follows a three-phase architecture:

1. **Lexical Analysis**: Converts source code into tokens
2. **Syntax Analysis**: Builds an Abstract Syntax Tree (AST) from tokens
3. **Interpretation**: Directly executes the AST with proper scoping

## Implementation Details

### Lexer
- Tokenizes Nepali keywords, identifiers, numbers, strings, and operators
- Handles single-line comments (`//`)
- Tracks line and column numbers for error reporting

### Parser
- Recursive descent parser following operator precedence
- Supports all language constructs including nested scopes
- Proper error handling with context information

### Interpreter
- Direct AST execution without intermediate code generation
- Lexical scoping with environment stack
- Function calls with parameter binding
- Recursion support

## Testing

The project includes comprehensive examples that demonstrate:
- Variable declarations and assignments
- Conditional statements (if/else)
- While loops
- Function definitions and calls
- Recursive functions
- Mathematical operations
- String handling
- Lists and dictionaries (creation, access, modification)
- Nested data structures
- Collection indexing and manipulation
- For-each loops over lists, dictionaries, and strings
- Advanced iteration patterns with functions
- Break and continue statements for loop control
- Nested loop control flow management

All examples produce the expected outputs as defined in the language specification.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new features
4. Ensure all examples still work
5. Submit a pull request

## Editor Support

The Khukuri language includes complete and tested syntax highlighting for popular text editors!

### Available Syntax Highlighting
- Visual Studio Code - Full extension with auto-completion and snippets
- Vim/Neovim - Complete syntax highlighting with auto-detection
- Sublime Text - Full syntax definition with smart features

All syntax definitions include:
- Nepali keyword highlighting (`maanau`, `yedi`, `kaam`, etc.)
- Operator highlighting (`ra`, `wa`, `hoina`)
- String, number, and comment highlighting
- Function and variable recognition
- Auto-indentation and bracket matching

**Installation:** See `editor-support/README.md` for detailed setup instructions.  
**Testing:** See `editor-support/TESTING.md` for verification details.

## Future Enhancements

- Lists and dictionaries - COMPLETED!
- For-each loops (`pratyek`) - COMPLETED!
- Break and continue statements - COMPLETED!
- Import system (`aayaat`) - COMPLETED!
- Syntax highlighting for editors - COMPLETED!
- Standard library functions
- Better error messages with suggestions
- Language server protocol (LSP) support

## License

This is an educational project for learning interpreter implementation concepts.

---

**Disclaimer**: This is a fun educational project. Khukuri is designed to make programming concepts accessible through familiar Nepali slang, not for production use.