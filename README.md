## [English](README_EN.md) | [中文](README.md)

<div align="center">
   <img src="assets/logo.jpg" width="30%" alt="JUMPING Logo">
</div>

# 🛰️ JUMPING

> **J**umping **U**nlocks **M**ulti-path **P**recise **I**nstant **N**avigating **G**ear.

JUMPING 是一个基于 Rust 开发的极轻量级 TUI 目录导航工具。它通过三栏式布局彻底解决终端深层路径跳转的痛苦，让你在文件夹间快速“闪现”。

## ✨ 功能亮点

* **💨 极速响应**: 基于 Rust 编写，无运行依赖，极致轻量。
* **📂 三栏视图**: 采用类似 macOS Finder 的父目录-当前目录-预览视图，结构清晰。
* **⌨️ 类 Vim 操作**: 支持 `h/j/k/l` 及方向键导航，符合习惯。
* **🎯 无缝跳转**: 配合 Shell 包装器，实现退出 UI 后自动 `cd` 到目标位置。

## 🚀 安装与配置

### 使用脚本一键安装（推荐）

我们提供了自动安装脚本，可以自动检测系统类型、下载对应二进制文件、安装到合适位置，并配置 Shell 环境：

**方式一：下载脚本后运行**

```bash
# 下载安装脚本
wget https://raw.githubusercontent.com/harkerhand/jumping/main/install.sh
chmod +x install.sh
./install.sh
```

**方式二：直接运行（如果系统支持）**

```bash
# 通过 curl 直接运行
curl -sSL https://raw.githubusercontent.com/harkerhand/jumping/main/install.sh | bash
```

脚本功能：

- ✅ 自动检测操作系统（Linux/macOS）
- ✅ 自动检测 Shell 类型（Bash/Zsh/Fish）
- ✅ 智能选择安装路径（系统目录或用户目录）
- ✅ 自动配置 Shell 环境
- ✅ 交互式安装，可选择自动添加配置

### 手动安装

1. **下载二进制文件:**
   从 [GitHub Releases](https://github.com/harkerhand/jumping/releases) 下载对应平台的二进制文件：

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

2. 如果你想从源代码编译安装，可以使用以下命令：

    ```bash
    cargo build --release
    cp target/release/jumping /usr/local/bin/
    ```

3. **配置 Shell:**

   将以下内容添加到你的 Shell 配置文件中，以启用跳转功能（这会创建一个`jp`函数）：
   对于 Bash 或 Zsh 用户，请添加：
    ```bash
    eval "$(jumping --init)"
    ```
   对于 Fish 用户，请添加：
    ```fish
    jumping --init fish | source
    ```

   你也可以直接运行 `jumping --init` 来查看对应 Shell 的配置命令。
   例如在 zsh 中运行会得到：
   ```bash
   jp() {
      local TMP_FILE="/tmp/jumping-1000"
      [ -f "$TMP_FILE" ] && rm "$TMP_FILE"

      jumping

      if [ -f "$TMP_FILE" ]; then
        local DEST=$(cat "$TMP_FILE")
        if [ -d "$DEST" ]; then
          cd "$DEST"
          pwd
        fi
        rm "$TMP_FILE"
      fi
   }
   ```

## 🎮 操作指南

* `j` / `k` 或 `↑` / `↓`: 在同级目录间上下切换。
* `h` / `l` 或 `←` / `→`: 在父目录与子目录间层级穿梭。
* `Enter`: 确认选择并直接跳转。
* `i`: 切换是否显示隐藏文件夹。
* `q` / `Esc`: 退出工具并保持原位。
