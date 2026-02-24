#!/bin/bash
# Postboy 启动脚本

set -e

# 颜色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 正在启动 Postboy...${NC}"

# 确保二进制文件是最新的
if [ ! -f "target/debug/Postboy.app/Contents/MacOS/postboy" ]; then
    echo "⚠️  应用未构建，正在构建..."
    cargo build --bin postboy
fi

# 创建 .app bundle（如果不存在）
if [ ! -d "target/debug/Postboy.app" ]; then
    echo "📦 创建 .app bundle..."
    mkdir -p target/debug/Postboy.app/Contents/MacOS
    mkdir -p target/debug/Postboy.app/Contents/Resources

    cat > target/debug/Postboy.app/Contents/Info.plist <<'EOF'
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

    cp target/debug/postboy target/debug/Postboy.app/Contents/MacOS/
    chmod +x target/debug/Postboy.app/Contents/MacOS/postboy
fi

# 启动应用
echo -e "${GREEN}✅ Postboy 启动中...${NC}"
open target/debug/Postboy.app
