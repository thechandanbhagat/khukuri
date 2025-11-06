# Editor Support for Khukuri Programming Language

This directory contains tested and verified syntax highlighting definitions for various text editors to support the Khukuri programming language (.nep files).

## Quick Installation

### Automated Installation (Recommended)

**Windows:**
```batch
cd editor-support
.\install.bat
```

**Linux/macOS:**
```bash
cd editor-support
chmod +x install.sh
./install.sh
```

## Supported Editors

### Visual Studio Code

**Quick Install:**
```bash
code --install-extension vscode/khukuri-language-support-1.3.0.vsix
```

**Manual Installation:**
1. Download the `.vsix` file from `vscode/khukuri-language-support-1.3.0.vsix`
2. Open VS Code
3. Press `Ctrl+Shift+P` (Windows/Linux) or `Cmd+Shift+P` (macOS)
4. Type "Install from VSIX" and select the downloaded file
5. Restart VS Code

**Features:**
- Syntax highlighting for all Nepali keywords
- Auto-completion and IntelliSense
- Bracket matching and auto-closing
- Comment toggling with Ctrl+/
- Code snippets
- String and number highlighting
- Function name highlighting
- Custom file icon for .nep files

### Vim/Neovim

**Installation:**
1. Copy syntax file to your Vim syntax directory:
   ```bash
   # For Vim
   cp vim/khukuri.vim ~/.vim/syntax/
   mkdir -p ~/.vim/ftdetect
   cp vim/ftdetect/khukuri.vim ~/.vim/ftdetect/
   
   # For Neovim
   cp vim/khukuri.vim ~/.config/nvim/syntax/
   mkdir -p ~/.config/nvim/ftdetect
   cp vim/ftdetect/khukuri.vim ~/.config/nvim/ftdetect/
   ```

2. The filetype will be automatically detected for `.nep` files

**Features:**
- Complete syntax highlighting for Khukuri keywords
- Proper highlighting for Nepali operators (`ra`, `wa`, `hoina`)
- Comments, strings, and numbers highlighting
- Function and variable highlighting

### Sublime Text

**Installation:**
1. Open Sublime Text
2. Go to `Preferences > Browse Packages...`
3. Create a folder named `Khukuri` in the Packages directory
4. Copy both files from the `sublime-text` folder:
   - `Khukuri.sublime-syntax`
   - `Khukuri.sublime-settings`
5. Restart Sublime Text

**Features:**
- Full syntax highlighting support
- Auto-indentation and smart brackets
- Custom settings optimized for Khukuri
- Auto-completion triggers

## Language Features Highlighted

All syntax definitions support highlighting for:

### Keywords
- **Control Flow**: `yedi` (if), `bhane` (then), `natra` (else), `jaba samma` (while), `rok` (break), `jane` (continue)
- **Declarations**: `maanau` (let/var), `kaam` (function), `pathau` (return), `aayaat` (import)
- **Output**: `bhan` (print)
- **Iteration**: `pratyek` (for each), `ma` (in)

### Operators
- **Logical**: `ra` (and), `wa` (or), `hoina` (not)
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`
- **Comparison**: `==`, `!=`, `>`, `<`, `>=`, `<=`
- **Assignment**: `=`

### Literals
- **Booleans**: `sahi` (true), `galat` (false)
- **Numbers**: Integer and floating-point literals
- **Strings**: Double-quoted strings with escape sequence support

### Other Elements
- **Comments**: Single-line comments starting with `//`
- **Functions**: Function name highlighting when followed by parentheses
- **Variables**: Identifier highlighting

## Testing Syntax Highlighting

Create a test file `test.nep` with this content:

```nepali
// This is a comment
maanau name = "Khukuri Language"
maanau version = 1.0

kaam greet(person) {
    yedi person != "" bhane {
        bhan "Namaste, "
        bhan person
    } natra {
        bhan "Namaste, World!"
    }
    pathau sahi
}

aayaat "utils.nep"

maanau numbers = [1, 2, 3, 4, 5]
pratyek num ma numbers {
    yedi num % 2 == 0 bhane {
        bhan num
        bhan " is even"
    }
}

greet(name)
```

Open this file in your editor to see the syntax highlighting in action!

## Contributing

If you'd like to add support for additional editors or improve existing syntax definitions:

1. Follow the existing patterns for keyword and operator definitions
2. Ensure all Nepali keywords are properly highlighted
3. Test with various code samples
4. Submit a pull request with your improvements

## Color Themes

The syntax highlighting works with any color theme in your editor. For best results, we recommend using themes that provide good contrast for:
- Keywords (usually blue or purple)
- Strings (usually green or red)
- Comments (usually gray or green)
- Numbers (usually orange or blue)
- Operators (usually red or white)