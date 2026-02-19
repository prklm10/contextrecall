# ContextRecall 🧠

**Stop scrolling through `kubectl` commands when you're working on a React app.**

ContextRecall is a CLI tool written in **Rust** that gives every directory its own isolated shell history. It automatically detects which project you are in and saves your commands to a local SQLite database, keeping your global history clean and your cognitive load low.

## 🚀 The Problem
Modern developers switch contexts constantly. You might be working on a backend Go service, a frontend React app, and Terraform infrastructure all in one day. 

Standard shell history is **global** and **linear**. When you press `Up Arrow` or `Ctrl+R` in your frontend folder, you don't want to see the database migration command you ran 20 minutes ago in the backend folder. That's how production accidents happen.

## ✨ Features
* **Context Aware:** Automatically detects project roots (looks for `.git`, `Cargo.toml`, `package.json`, etc.).
* **Isolated Memory:** Commands run in `~/projects/backend` stay in `~/projects/backend`.
* **Zero Latency:** Written in Rust with a local SQLite engine. It doesn't lag your prompt.
* **Smart Search:** Replaces your standard `Ctrl+R` with a context-specific **fzf** interface.
* **Privacy First:** Data is stored locally in `~/.contextrecall.db`.

## 📦 Installation

### Prerequisites
* **Rust** (via `rustup`)
* **fzf** (for the UI)
* **Zsh** (Support for Bash is experimental)

### Automated Install
We provide a setup script that builds the binary and configures your shell automatically.

```bash
# 1. Clone the repo
git clone [https://github.com/prklm10/contextrecall.git](https://github.com/prklm10/contextrecall.git)
cd contextrecall

# 2. Run the installer
chmod +x install.sh
./install.sh

# 3. Reload your shell
source ~/.zshrc
