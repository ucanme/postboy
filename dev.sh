#!/bin/bash
# Postboy 开发工具脚本
# 用于简化常见的开发任务

set -e

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# 显示帮助信息
show_help() {
    cat << EOF
Postboy 开发工具

用法: ./dev.sh [命令]

命令:
  build       构建项目
  run         运行应用
  clean       清理构建产物
  test        运行测试
  fmt         格式化代码
  clippy      运行 clippy 检查
  doc         生成文档
  fix         自动修复警告
  release     构建 release 版本
  status      显示项目状态
  help        显示此帮助信息

示例:
  ./dev.sh build     # 构建项目
  ./dev.sh run       # 运行应用
  ./dev.sh clean     # 清理构建产物

EOF
}

# 构建项目
cmd_build() {
    print_info "构建 Postboy..."
    cargo build --workspace
    print_success "构建完成"
}

# 运行应用
cmd_run() {
    print_info "启动 Postboy..."

    # 确保应用已构建
    if [ ! -f "target/debug/Postboy.app/Contents/MacOS/postboy" ]; then
        print_warning "应用未构建，正在构建..."
        cmd_build
    fi

    # 运行应用
    open target/debug/Postboy.app
    print_success "应用已启动"
}

# 清理构建产物
cmd_clean() {
    print_info "清理构建产物..."
    cargo clean
    print_success "清理完成"
}

# 运行测试
cmd_test() {
    print_info "运行测试..."
    cargo test --workspace
    print_success "测试完成"
}

# 格式化代码
cmd_fmt() {
    print_info "格式化代码..."
    cargo fmt
    print_success "格式化完成"
}

# 运行 clippy
cmd_clippy() {
    print_info "运行 Clippy 检查..."
    cargo clippy --workspace -- -D warnings
    print_success "检查完成"
}

# 生成文档
cmd_doc() {
    print_info "生成文档..."
    cargo doc --workspace --no-deps --open
    print_success "文档已生成"
}

# 自动修复警告
cmd_fix() {
    print_info "自动修复警告..."
    cargo fix --lib --allow-dirty --allow-staged
    print_success "修复完成"
}

# 构建 release 版本
cmd_release() {
    print_info "构建 Release 版本..."
    cargo build --release --bin postboy

    # 创建 .app bundle
    print_info "创建 .app bundle..."
    mkdir -p target/release/Postboy.app/Contents/MacOS
    mkdir -p target/release/Postboy.app/Contents/Resources

    # 复制 Info.plist
    cat > target/release/Postboy.app/Contents/Info.plist <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>postboy</string>
    <key>CFBundleIdentifier</key>
    <string>com.postboy.app</string>
    <key>CFBundleName</key>
    <string>Postboy</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

    # 复制二进制文件
    cp target/release/postboy target/release/Postboy.app/Contents/MacOS/
    chmod +x target/release/Postboy.app/Contents/MacOS/postboy

    print_success "Release 构建完成: target/release/Postboy.app"
}

# 显示项目状态
cmd_status() {
    echo "📊 Postboy 项目状态"
    echo "===================="
    echo ""

    # 检查是否构建
    if [ -f "target/debug/postboy" ]; then
        echo "✅ 调试版本: 已构建"
        ls -lh target/debug/postboy | awk '{print "   大小: " $5}'
    else
        echo "❌ 调试版本: 未构建"
    fi

    if [ -f "target/release/postboy" ]; then
        echo "✅ Release 版本: 已构建"
        ls -lh target/release/postboy | awk '{print "   大小: " $5}'
    else
        echo "❌ Release 版本: 未构建"
    fi

    echo ""

    # 检查进程
    PROCESS_COUNT=$(ps aux | grep -i postboy | grep -v grep | wc -l | tr -d ' ')
    if [ "$PROCESS_COUNT" -gt 0 ]; then
        echo "✅ 运行状态: 正在运行 ($PROCESS_COUNT 个进程)"
    else
        echo "❌ 运行状态: 未运行"
    fi

    echo ""

    # Git 状态
    if [ -d ".git" ]; then
        echo "📝 Git 状态:"
        git status -s 2>/dev/null | head -5 || echo "   无变更"
    fi
}

# 主逻辑
main() {
    case "${1:-help}" in
        build)
            cmd_build
            ;;
        run)
            cmd_run
            ;;
        clean)
            cmd_clean
            ;;
        test)
            cmd_test
            ;;
        fmt)
            cmd_fmt
            ;;
        clippy)
            cmd_clippy
            ;;
        doc)
            cmd_doc
            ;;
        fix)
            cmd_fix
            ;;
        release)
            cmd_release
            ;;
        status)
            cmd_status
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "未知命令: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

main "$@"
