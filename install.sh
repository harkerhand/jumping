#!/bin/bash

# JUMPING 安装脚本
# 下载预编译二进制文件并配置 Shell 环境

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 输出函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检测系统类型
detect_system() {
    case "$(uname -s)" in
        Linux*)     SYSTEM="linux";;
        Darwin*)    SYSTEM="macos";;
        *)          SYSTEM="unknown"
    esac

    echo "$SYSTEM"
}

# 检测系统架构（可选）
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)   ARCH="x86_64";;
        arm64|aarch64)  ARCH="aarch64";;
        *)              ARCH="unknown"
    esac

    echo "$ARCH"
}

# 检测下载工具
detect_download_tool() {
    if command -v curl &> /dev/null; then
        echo "curl"
    elif command -v wget &> /dev/null; then
        echo "wget"
    else
        echo "none"
    fi
}

# 下载文件
download_file() {
    local url="$1"
    local output="$2"
    local tool="$3"

    log_info "下载: $url"

    if ! case "$tool" in
        curl)
            curl -L -o "$output" "$url"
            ;;
        wget)
            wget -O "$output" "$url"
            ;;
        *)
            log_error "未找到 curl 或 wget，请安装其中一个下载工具"
            exit 1
            ;;
    esac; then
        log_error "下载失败: $url"
        exit 1
    fi
}

# 检测 Shell 类型
detect_shell() {
    local shell_name
    shell_name="$(basename "$SHELL")"

    case "$shell_name" in
        bash)   echo "bash";;
        zsh)    echo "zsh";;
        fish)   echo "fish";;
        *)      echo "unknown";;
    esac
}

# 获取 Shell 配置文件路径
get_shell_config() {
    local shell_type="$1"

    case "$shell_type" in
        bash)
            if [ -f "$HOME/.bashrc" ]; then
                echo "$HOME/.bashrc"
            elif [ -f "$HOME/.bash_profile" ]; then
                echo "$HOME/.bash_profile"
            else
                echo "$HOME/.bashrc"
            fi
            ;;
        zsh)
            if [ -f "$HOME/.zshrc" ]; then
                echo "$HOME/.zshrc"
            else
                echo "$HOME/.zshrc"
            fi
            ;;
        fish)
            echo "$HOME/.config/fish/config.fish"
            ;;
        *)
            echo ""
            ;;
    esac
}

# 主安装函数
main() {
    log_info "开始安装 JUMPING..."

    # 检测系统
    SYSTEM=$(detect_system)
    if [ "$SYSTEM" = "unknown" ]; then
        log_error "不支持的操作系统: $(uname -s)"
        exit 1
    fi
    log_info "检测到系统: $SYSTEM"

    # 检测下载工具
    DOWNLOAD_TOOL=$(detect_download_tool)
    log_info "使用下载工具: $DOWNLOAD_TOOL"

    # 定义下载链接
    BINARY_NAME="jumping-$SYSTEM"
    DOWNLOAD_URL="https://github.com/harkerhand/jumping/releases/latest/download/$BINARY_NAME"
    log_info "下载链接: $DOWNLOAD_URL"

    # 临时目录
    TEMP_DIR=$(mktemp -d)
    DOWNLOAD_PATH="$TEMP_DIR/jumping"

    # 下载二进制文件
    log_info "下载预编译二进制文件..."
    download_file "$DOWNLOAD_URL" "$DOWNLOAD_PATH" "$DOWNLOAD_TOOL"

    # 设置执行权限
    chmod +x "$DOWNLOAD_PATH"
    log_success "二进制文件已下载并设置可执行权限"

    # 安装到系统路径
    INSTALL_DIR="/usr/local/bin"
    INSTALL_PATH="$INSTALL_DIR/jumping"

    log_info "尝试安装到: $INSTALL_PATH"

    if [ ! -w "$INSTALL_DIR" ]; then
        log_warn "需要 sudo 权限来安装到 $INSTALL_DIR"
        if ! sudo -v; then
            log_error "sudo 认证失败，尝试安装到用户目录"
            USER_INSTALL_DIR="$HOME/.local/bin"
            mkdir -p "$USER_INSTALL_DIR"
            INSTALL_DIR="$USER_INSTALL_DIR"
            INSTALL_PATH="$INSTALL_DIR/jumping"
            log_info "将安装到: $INSTALL_PATH"
        else
            sudo cp "$DOWNLOAD_PATH" "$INSTALL_PATH"
            sudo chmod +x "$INSTALL_PATH"
        fi
    else
        cp "$DOWNLOAD_PATH" "$INSTALL_PATH"
        chmod +x "$INSTALL_PATH"
    fi

    if [ -f "$INSTALL_PATH" ]; then
        log_success "JUMPING 已安装到: $INSTALL_PATH"
    else
        cp "$DOWNLOAD_PATH" "$INSTALL_PATH"
        chmod +x "$INSTALL_PATH"
        log_success "JUMPING 已安装到: $INSTALL_PATH"
    fi

    # 清理临时文件
    rm -rf "$TEMP_DIR"

    # 检测 Shell 类型
    SHELL_TYPE=$(detect_shell)
    log_info "检测到 Shell: $SHELL_TYPE"

    # 配置 Shell
    log_info "配置 Shell 环境..."

    if [ "$SHELL_TYPE" = "unknown" ]; then
        log_warn "未知 Shell 类型，请手动配置"
        echo ""
        echo "请手动运行以下命令来配置 Shell:"
        echo "  jumping --init"
        echo ""
        echo "然后将输出内容添加到您的 Shell 配置文件中。"
    else
        CONFIG_FILE=$(get_shell_config "$SHELL_TYPE")

        echo ""
        echo "================================================"
        log_success "JUMPING 安装完成！"
        echo ""
        echo "接下来请将以下配置添加到您的 Shell 配置文件中:"
        echo "配置文件: $CONFIG_FILE"
        echo ""

        # 运行 jumping --init 获取配置
        if command -v jumping &> /dev/null; then
            echo "运行以下命令获取配置:"
            echo "  jumping --init"
            echo ""
            echo "或者直接运行以下命令自动添加配置:"
            echo "  echo 'eval \"\$(jumping --init)\"' >> $CONFIG_FILE"

            # 询问是否自动添加配置
            echo ""
            read -p "是否自动添加配置到 $CONFIG_FILE? [Y/n]: " -n 1 -r < /dev/tty
            echo ""
            if [[ -z $REPLY ||  $REPLY =~ ^[Yy]$ ]]; then
                if [ -f "$CONFIG_FILE" ]; then
                    {
                      echo "" >> "$CONFIG_FILE"
                    }
                    echo "# JUMPING configuration" >> "$CONFIG_FILE"
                    jumping --init >> "$CONFIG_FILE"
                    log_success "配置已添加到 $CONFIG_FILE"
                    echo ""
                    echo "请重新启动终端或运行: source $CONFIG_FILE"
                else
                    log_warn "配置文件 $CONFIG_FILE 不存在，创建新文件"
                    jumping --init > "$CONFIG_FILE"
                    log_success "配置文件已创建: $CONFIG_FILE"
                fi
            fi
        else
            log_error "jumping 命令未找到，请检查安装路径是否在 PATH 中"
            echo "请手动将 $INSTALL_DIR 添加到 PATH 环境变量"
        fi
    fi

    echo ""
    echo "使用说明:"
    echo "  1. 在终端中输入 'jp' 启动 JUMPING"
    echo "  2. 使用方向键或 h/j/k/l 导航"
    echo "  3. 按 Enter 选择目录并跳转"
    echo "  4. 按 q 或 Esc 退出"
    echo "  5. 按 I/i 切换隐藏文件夹显示"
    echo ""
    log_success "安装完成！"
}

# 运行主函数
main "$@"