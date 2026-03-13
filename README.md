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

### 下载预编译二进制文件

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

### 从源码编译

1. **编译源码:**

    ```bash
    cargo build --release
    cp target/release/jumping /usr/local/bin/
    ```

2. **配置 Shell:**

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
* `q` / `Esc`: 退出工具并保持原位。
