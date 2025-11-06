# Khukuri Language Support for VS Code

This extension provides comprehensive language support for the Khukuri programming language - a fun, educational interpreter that uses Nepali Gen-Z slang keywords.

## Features

- **Syntax Highlighting**: Complete syntax highlighting for all Khukuri keywords, operators, and constructs
- **Code Snippets**: Intelligent code snippets for common patterns:
  - `maanau` - Variable declaration
  - `yedi` - If statement  
  - `yedi-natra` - If-else statement
  - `jaba samma` - While loop
  - `pratyek` - For each loop
  - `kaam` - Function declaration
  - `bhan` - Print statement
  - And many more!
- **Auto-completion**: Smart bracket matching and auto-closing pairs
- **Indentation**: Automatic indentation for code blocks
- **File Icons**: Custom file icons for `.nep` files

## Khukuri Keywords

The extension highlights these Nepali keywords:

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

## Usage

1. Install the extension
2. Open or create a `.nep` file
3. Start coding in Khukuri!
4. Use Ctrl+Space to trigger code snippets

## Example

```khukuri
// Example Khukuri program
maanau naam = "Ram"
maanau umar = 25

yedi umar >= 18 bhane {
    bhan "Adult cha"
} natra {
    bhan "Minor cha"
}

kaam greet(person) {
    bhan "Namaste"
    bhan person
    pathau sahi
}

greet(naam)
```

## Repository

This extension is part of the Khukuri programming language project. Visit the repository for more information about the language itself.

## License

MIT License - Feel free to contribute!