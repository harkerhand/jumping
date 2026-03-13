## [English](https://www.google.com/search?q=README_EN.md) | [中文](README.md)

<div align="center">
<img src="assets/logo.jpg" width="30%" alt="JUMPING Logo">
</div>

# 🛰️ JUMPING

> **J**umping **U**nlocks **M**ulti-path **P**recise **I**nstant **N**avigating **G**ear.

JUMPING is an ultra-lightweight TUI directory navigation tool built with Rust. It eliminates the friction of navigating
deep terminal paths via a three-column layout, allowing you to "teleport" between folders instantly.

---

## ✨ Features

* **💨 Blazing Fast**: Written in Rust with zero runtime dependencies—pure performance.
* **📂 Three-Column View**: A macOS Finder-style layout (Parent-Current-Preview) for clear structural context.
* **⌨️ Vim-like Motion**: Supports `h/j/k/l` and arrow keys for intuitive navigation.
* **🎯 Seamless Jump**: Integrated Shell wrapper enables automatic `cd` to the target location upon exit.

---

## 🚀 Installation & Configuration

### One-Click Installation (Recommended)

We provide an automated script that detects your system, downloads the correct binary, installs it, and configures your
Shell environment:

**Option 1: Download and Run**

```bash
# Download the installer
wget https://raw.githubusercontent.com/harkerhand/jumping/master/install.sh
chmod +x install.sh
./install.sh

```

**Option 2: Direct Pipe (if supported)**

```bash
# Run via curl
curl -sSL https://raw.githubusercontent.com/harkerhand/jumping/master/install.sh | bash

```

**Script Capabilities:**

* ✅ Auto-detects OS (Linux/macOS)
* ✅ Auto-detects Shell type (Bash/Zsh/Fish)
* ✅ Intelligent path selection (System or User bin)
* ✅ Automatic Shell environment configuration
* ✅ Interactive setup for profile updates

---

### Manual Installation

1. **Download Binary:**
   Download the specific version for your platform
   from [GitHub Releases](https://github.com/harkerhand/jumping/releases):

   ```bash
   # Linux
   wget https://github.com/harkerhand/jumping/releases/latest/download/jumping-linux
   chmod +x jumping-linux
   sudo mv jumping-linux /usr/local/bin/jumping
   
   # macOS
   wget https://github.com/harkerhand/jumping/releases/latest/download/jumping-macos
   chmod +x jumping-macos
   sudo mv jumping-macos /usr/local/bin/jumping
   
   ```

2. **Build from Source (Optional):**

   ```bash
   cargo build --release
   cp target/release/jumping /usr/local/bin/
   
   ```

3. **Configure Shell:**
   Add the following to your Shell profile to enable the `jp` jump function:
   **For Bash or Zsh:**

   ```bash
   eval "$(jumping --init)"
   ```

   **For Fish:**

   ```fish
   jumping --init fish | source
   ```

> [!TIP]
> Run `jumping --init` manually to see the underlying script. For example, in Zsh, it creates a `jp()` function that
> handles the temporary file hand-off for the `cd` command.

---

## 🎮 Controls

* **`j` / `k**` or **`↑` / `↓**`: Move up and down within the current directory.
* **`h` / `l**` or **`←` / `→**`: Traverse between parent and child directories.
* **`Enter`**: Confirm selection and jump to the directory.
* **`i`**: Toggle visibility of hidden files/folders.
* **`q` / `Esc**`: Quit the tool and stay in the current directory.
