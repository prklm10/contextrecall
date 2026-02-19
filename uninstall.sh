#!/bin/bash

echo "🗑️  Uninstalling ContextRecall..."

# 1. Remove the binary
if command -v brew &> /dev/null && brew list contextrecall &> /dev/null; then
    echo "🍺 Removing Homebrew installation..."
    brew uninstall contextrecall
elif command -v cargo &> /dev/null; then
    echo "🦀 Removing Cargo installation..."
    cargo uninstall contextrecall 2>/dev/null || true
else
    echo "⚠️  Could not find Homebrew or Cargo installation. Skipping binary removal."
fi

# 2. Remove the init script directory
if [ -d "$HOME/.contextrecall" ]; then
    echo "🧹 Removing shell init scripts..."
    rm -rf "$HOME/.contextrecall"
fi

# 3. Clean up the shell config (.zshrc / .bashrc)
echo "📝 Cleaning up shell configuration..."

if [ -f "$HOME/.zshrc" ]; then
    # Create a temporary file without the contextrecall line, then overwrite the original
    grep -v ".contextrecall/init.zsh" "$HOME/.zshrc" > "$HOME/.zshrc.tmp" && mv "$HOME/.zshrc.tmp" "$HOME/.zshrc"
    echo "✅ Removed from ~/.zshrc"
fi

if [ -f "$HOME/.bashrc" ]; then
    grep -v ".contextrecall/init.zsh" "$HOME/.bashrc" > "$HOME/.bashrc.tmp" && mv "$HOME/.bashrc.tmp" "$HOME/.bashrc"
    echo "✅ Removed from ~/.bashrc"
fi

# 4. Handle the Database (Give the user a choice)
if [ -f "$HOME/.contextrecall.db" ]; then
    echo ""
    read -p "❓ Do you want to delete your command history database (~/.contextrecall.db)? (y/N) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -f "$HOME/.contextrecall.db"
        echo "💥 Database deleted."
    else
        echo "💾 Database preserved. Your history is safe."
    fi
fi

echo ""
echo "✨ Uninstallation complete! Please restart your terminal for changes to take effect."
