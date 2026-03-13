## [English](README_EN.md) | [中文](README.md)

<div align="center">
   <img src="assets/logo.jpg" width="30%" alt="JUMPING Logo">
</div>

# 🛰️ JUMPING

> **J**umping **U**nlocks **M**ulti-path **P**recise **I**nstant **N**avigating **G**ear.

JUMPING is a blazingly fast, lightweight TUI directory navigator built in Rust. It solves
the "deep path fatigue" by providing a 3-column Miller column view to preview and teleport between directories
instantly.

## ✨ Key Features

* **💨 Lightning Fast**: Zero-dependency runtime, minimal resource footprint.
* **📂 Miller Columns**: 3-pane layout (Parent, Current, Preview) inspired by macOS Finder/Ranger.
* **⌨️ Vim-like Motion**: Navigate with `h/j/k/l` or arrow keys.
* **🎯 Instant Teleport**: Exit and `cd` to the selected directory automatically via shell wrapper.

## 🚀 Installation & Setup

### Download Pre-built Binaries

Download the pre-built binaries from [GitHub Releases](https://github.com/harkerhand/jumping/releases):

```bash
# Linux
wget https://github.com/harkerhand/jumping/releases/download/v0.1.0/jumping-linux
chmod +x jumping-linux
sudo mv jumping-linux /usr/local/bin/jumping

# macOS
wget https://github.com/harkerhand/jumping/releases/download/v0.1.0/jumping-macos
chmod +x jumping-macos
sudo mv jumping-macos /usr/local/bin/jumping

# Windows
# Download jumping-windows.exe and add to PATH
```

### Build from Source

1. **Build from source:**

    ```bash
    cargo build --release
    cp target/release/jumping /usr/local/bin/
    ```

2. **Add to `.bashrc` or `.zshrc`:**

    ```bash
    jp() {
        [ -f /tmp/jumping_result ] && rm /tmp/jumping_result
        jumping
        if [ -f /tmp/jumping_result ]; then
            local DEST=$(cat /tmp/jumping_result)
            [ -d "$DEST" ] && cd "$DEST" && pwd
            rm /tmp/jumping_result
        fi
    }
    ```

## 🎮 Usage

* `j` / `k` or `↑` / `↓`: Move up and down within the current directory.
* `h` / `l` or `←` / `→`: Traverse through directory hierarchy.
* `Enter`: Confirm selection and teleport.
* `q` / `Esc`: Quit without changing directory.
