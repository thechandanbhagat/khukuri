#!/bin/bash
# Khukuri Language Editor Support Installation Script

echo "Khukuri Language Editor Support Installer"
echo "=============================================="

# Check for VS Code
if command -v code &> /dev/null; then
    echo "VS Code detected"
    read -p "Install Khukuri syntax highlighting for VS Code? (y/n): " install_vscode
    if [[ $install_vscode == "y" || $install_vscode == "Y" ]]; then
        # Install using .vsix file
        if [ -f "vscode/khukuri-language-support-1.3.0.vsix" ]; then
            code --install-extension "vscode/khukuri-language-support-1.3.0.vsix"
            echo "VS Code extension installed successfully!"
            echo "   Restart VS Code to activate syntax highlighting"
        else
            echo "ERROR: .vsix file not found!"
        fi
    fi
fi

# Check for Vim
if command -v vim &> /dev/null; then
    echo "Vim detected"
    read -p "Install Khukuri syntax highlighting for Vim? (y/n): " install_vim
    if [[ $install_vim == "y" || $install_vim == "Y" ]]; then
        # Create directories
        mkdir -p ~/.vim/syntax
        mkdir -p ~/.vim/ftdetect
        
        # Copy files
        cp vim/khukuri.vim ~/.vim/syntax/
        cp vim/ftdetect/khukuri.vim ~/.vim/ftdetect/
        echo "Vim syntax files installed"
    fi
fi

# Check for Neovim
if command -v nvim &> /dev/null; then
    echo "Neovim detected"
    read -p "Install Khukuri syntax highlighting for Neovim? (y/n): " install_nvim
    if [[ $install_nvim == "y" || $install_nvim == "Y" ]]; then
        # Create directories
        mkdir -p ~/.config/nvim/syntax
        mkdir -p ~/.config/nvim/ftdetect
        
        # Copy files
        cp vim/khukuri.vim ~/.config/nvim/syntax/
        cp vim/ftdetect/khukuri.vim ~/.config/nvim/ftdetect/
        echo "Neovim syntax files installed"
    fi
fi

# Instructions for Sublime Text
echo ""
echo "For Sublime Text:"
echo "   1. Open Sublime Text"
echo "   2. Go to Preferences > Browse Packages..."
echo "   3. Create a 'Khukuri' folder"
echo "   4. Copy files from sublime-text/ folder to the Khukuri folder"
echo "   5. Restart Sublime Text"

echo ""
echo "Installation complete!"
echo "Create a .nep file to test syntax highlighting!"