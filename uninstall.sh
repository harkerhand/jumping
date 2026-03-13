#!/bin/bash

# JUMPING 简易卸载脚本
# 仅移除二进制文件，不更改 Shell 配置

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARNING]${NC} $1"; }

main() {
    log_info "开始移除 JUMPING 二进制文件..."

    # 定义可能的安装路径
    INSTALL_PATHS=(
        "/usr/local/bin/jumping"
        "$HOME/.local/bin/jumping"
    )

    FOUND=false

    for path in "${INSTALL_PATHS[@]}"; do
        if [ -f "$path" ]; then
            log_info "检测到文件: $path"
            FOUND=true

            # 检查是否有写入权限，没有则尝试 sudo
            if [ ! -w "$(dirname "$path")" ]; then
                log_warn "需要 sudo 权限移除 $path"
                if sudo rm -f "$path"; then
                    log_success "成功移除: $path"
                else
                    echo -e "${RED}[ERROR]${NC} 无法移除 $path"
                fi
            else
                rm -f "$path"
                log_success "成功移除: $path"
            fi
        fi
    done

    if [ "$FOUND" = false ]; then
        log_warn "未在预设目录中找到 jumping 可执行文件。"
    fi

    echo ""
    log_success "清理完成！"
    log_info "注意：Shell 配置文件中的 eval 或 jp 函数需手动（或保留）处理。"
}

main "$@"