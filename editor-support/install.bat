@echo off
REM Khukuri Language Editor Support Installation Script for Windows

echo Khukuri Language Editor Support Installer
echo ==============================================

REM Check for VS Code and install using .vsix file
where code >nul 2>nul
if %errorlevel% == 0 (
    echo VS Code detected
    set /p install_vscode="Install Khukuri syntax highlighting for VS Code? (y/n): "
    if /i "%install_vscode%" == "y" (
        if exist "vscode\khukuri-language-support-1.3.0.vsix" (
            code --install-extension "vscode\khukuri-language-support-1.3.0.vsix"
            echo VS Code extension installed successfully!
            echo    Restart VS Code to activate syntax highlighting
        ) else (
            echo ERROR: .vsix file not found!
        )
    )
)

REM Check for Vim
where vim >nul 2>nul
if %errorlevel% == 0 (
    echo Vim detected
    set /p install_vim="Install Khukuri syntax highlighting for Vim? (y/n): "
    if /i "%install_vim%" == "y" (
        REM Create directories
        mkdir "%USERPROFILE%\.vim\syntax" 2>nul
        mkdir "%USERPROFILE%\.vim\ftdetect" 2>nul
        
        REM Copy files
        copy vim\khukuri.vim "%USERPROFILE%\.vim\syntax\"
        copy vim\ftdetect\khukuri.vim "%USERPROFILE%\.vim\ftdetect\"
        echo Vim syntax files installed
    )
)

echo.
echo For Sublime Text:
echo    1. Open Sublime Text
echo    2. Go to Preferences ^> Browse Packages...
echo    3. Create a 'Khukuri' folder
echo    4. Copy files from sublime-text\ folder to the Khukuri folder
echo    5. Restart Sublime Text

echo.
echo Installation complete!
echo Create a .nep file to test syntax highlighting!
pause