# Publishing Khukuri Extension to VS Code Marketplace

## Prerequisites

1. **Create a Microsoft/Azure Account** (if you don't have one)
   - Go to https://azure.microsoft.com/
   - Sign up for a free account

2. **Create a Publisher Account**
   - Go to https://marketplace.visualstudio.com/manage
   - Click "Create Publisher"
   - Enter Publisher Name: `khukuri-dev` (or your preferred name)
   - Enter Display Name: `Khukuri Development`
   - Fill in other required details

3. **Create a Personal Access Token (PAT)**
   - Go to https://dev.azure.com/
   - Click on your profile → Security → Personal Access Tokens
   - Click "New Token"
   - Name: "VS Code Marketplace"
   - Organization: All accessible organizations
   - Expiration: Custom (1 year recommended)
   - Scopes: Select "Marketplace" → Check "Manage"
   - Click "Create" and **SAVE THE TOKEN** (you won't see it again!)

## Publishing Steps

### First-time Setup

```bash
# Login with your publisher account
vsce login khukuri-dev
# Enter your Personal Access Token when prompted
```

### Publish the Extension

```bash
# Navigate to the extension directory
cd editor-support/vscode

# Verify the package
vsce ls

# Package the extension (optional - creates .vsix file)
vsce package

# Publish to marketplace
vsce publish
```

### Update Version and Republish

```bash
# For patch updates (1.3.0 -> 1.3.1)
vsce publish patch

# For minor updates (1.3.0 -> 1.4.0)
vsce publish minor

# For major updates (1.3.0 -> 2.0.0)
vsce publish major

# Or specify version directly
vsce publish 1.4.0
```

## Current Extension Details

- **Name**: khukuri-language-support
- **Display Name**: Khukuri Language Support
- **Publisher**: khukuri-dev
- **Version**: 1.3.0
- **Repository**: https://github.com/thechandanbhagat/khukuri

## Post-Publishing

1. **Verify on Marketplace**
   - Go to https://marketplace.visualstudio.com/
   - Search for "Khukuri"
   - Check that all information displays correctly

2. **Test Installation**
   ```bash
   code --install-extension khukuri-dev.khukuri-language-support
   ```

3. **Share the Link**
   - Marketplace: `https://marketplace.visualstudio.com/items?itemName=khukuri-dev.khukuri-language-support`
   - Install command: `ext install khukuri-dev.khukuri-language-support`

## Troubleshooting

### "Publisher not found" Error
- Make sure you created the publisher on the marketplace website
- Publisher name in package.json must match exactly

### "Authentication failed" Error
- Verify your Personal Access Token is valid
- Make sure the token has "Marketplace: Manage" scope
- Run `vsce login khukuri-dev` again

### "Missing required field" Error
- Check package.json has all required fields
- Run `vsce ls` to verify files are included
- Ensure README.md exists and has content

## Important Notes

- **Publisher name cannot be changed** after creation
- Extension name and publisher together form the unique ID
- First publish may take 5-10 minutes to appear in marketplace
- Updates are usually live within minutes
