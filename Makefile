# Postboy Makefile
# 便捷的开发命令

.PHONY: all build run clean test fmt clippy doc fix release status help

# 默认目标
all: build

# 构建项目
build:
	@echo "🔨 构建 Postboy..."
	@cargo build --workspace

# 运行应用
run: build
	@echo "🚀 启动 Postboy..."
	@if [ ! -f "target/debug/Postboy.app/Contents/MacOS/postboy" ]; then \
		mkdir -p target/debug/Postboy.app/Contents/MacOS; \
		mkdir -p target/debug/Postboy.app/Contents/Resources; \
		echo '<?xml version="1.0" encoding="UTF-8"?>' > target/debug/Postboy.app/Contents/Info.plist; \
		echo '<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">' >> target/debug/Postboy.app/Contents/Info.plist; \
		echo '<plist version="1.0"><dict>' >> target/debug/Postboy.app/Contents/Info.plist; \
		echo '<key>CFBundleExecutable</key><string>postboy</string>' >> target/debug/Postboy.app/Contents/Info.plist; \
		echo '<key>CFBundleIdentifier</key><string>com.postboy.app</string>' >> target/debug/Postboy.app/Contents/Info.plist; \
		echo '<key>CFBundleName</key><string>Postboy</string>' >> target/debug/Postboy.app/Contents/Info.plist; \
		echo '<key>CFBundleVersion</key><string>0.1.0</string>' >> target/debug/Postboy.app/Contents/Info.plist; \
		echo '<key>CFBundleShortVersionString</key><string>0.1.0</string>' >> target/debug/Postboy.app/Contents/Info.plist; \
		echo '<key>CFBundlePackageType</key><string>APPL</string>' >> target/debug/Postboy.app/Contents/Info.plist; \
		echo '<key>NSHighResolutionCapable</key><true/>' >> target/debug/Postboy.app/Contents/Info.plist; \
		echo '</dict></plist>' >> target/debug/Postboy.app/Contents/Info.plist; \
		cp target/debug/postboy target/debug/Postboy.app/Contents/MacOS/; \
		chmod +x target/debug/Postboy.app/Contents/MacOS/postboy; \
	fi
	@open target/debug/Postboy.app

# 清理构建产物
clean:
	@echo "🧹 清理构建产物..."
	@cargo clean

# 运行测试
test:
	@echo "🧪 运行测试..."
	@cargo test --workspace

# 格式化代码
fmt:
	@echo "🎨 格式化代码..."
	@cargo fmt

# 运行 clippy
clippy:
	@echo "🔍 运行 Clippy..."
	@cargo clippy --workspace -- -D warnings

# 生成文档
doc:
	@echo "📚 生成文档..."
	@cargo doc --workspace --no-deps --open

# 自动修复警告
fix:
	@echo "🔧 自动修复警告..."
	@cargo fix --lib --allow-dirty --allow-staged

# 构建 release 版本
release:
	@echo "📦 构建 Release 版本..."
	@cargo build --release --bin postboy
	@mkdir -p target/release/Postboy.app/Contents/MacOS
	@mkdir -p target/release/Postboy.app/Contents/Resources
	@echo '<?xml version="1.0" encoding="UTF-8"?>' > target/release/Postboy.app/Contents/Info.plist
	@echo '<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">' >> target/release/Postboy.app/Contents/Info.plist
	@echo '<plist version="1.0"><dict>' >> target/release/Postboy.app/Contents/Info.plist
	@echo '<key>CFBundleExecutable</key><string>postboy</string>' >> target/release/Postboy.app/Contents/Info.plist
	@echo '<key>CFBundleIdentifier</key><string>com.postboy.app</string>' >> target/release/Postboy.app/Contents/Info.plist
	@echo '<key>CFBundleName</key><string>Postboy</string>' >> target/release/Postboy.app/Contents/Info.plist
	@echo '<key>CFBundleVersion</key><string>0.1.0</string>' >> target/release/Postboy.app/Contents/Info.plist
	@echo '<key>CFBundleShortVersionString</key><string>0.1.0</string>' >> target/release/Postboy.app/Contents/Info.plist
	@echo '<key>CFBundlePackageType</key><string>APPL</string>' >> target/release/Postboy.app/Contents/Info.plist
	@echo '<key>NSHighResolutionCapable</key><true/>' >> target/release/Postboy.app/Contents/Info.plist
	@echo '</dict></plist>' >> target/release/Postboy.app/Contents/Info.plist
	@cp target/release/postboy target/release/Postboy.app/Contents/MacOS/
	@chmod +x target/release/Postboy.app/Contents/MacOS/postboy
	@echo "✅ Release 构建完成: target/release/Postboy.app"

# 显示项目状态
status:
	@echo "📊 Postboy 项目状态"
	@echo "===================="
	@echo ""
	@if [ -f "target/debug/postboy" ]; then \
		echo "✅ 调试版本: 已构建"; \
		ls -lh target/debug/postboy | awk '{print "   大小: " $$5}'; \
	else \
		echo "❌ 调试版本: 未构建"; \
	fi
	@echo ""
	@PROCESS_COUNT=$$(ps aux | grep -i postboy | grep -v grep | wc -l | tr -d ' '); \
	if [ "$$PROCESS_COUNT" -gt 0 ]; then \
		echo "✅ 运行状态: 正在运行 ($$PROCESS_COUNT 个进程)"; \
	else \
		echo "❌ 运行状态: 未运行"; \
	fi

# 帮助信息
help:
	@echo "Postboy Makefile 命令"
	@echo ""
	@echo "使用: make [目标]"
	@echo ""
	@echo "可用目标:"
	@echo "  build     - 构建项目"
	@echo "  run       - 运行应用"
	@echo "  clean     - 清理构建产物"
	@echo "  test      - 运行测试"
	@echo "  fmt       - 格式化代码"
	@echo "  clippy    - 运行 Clippy 检查"
	@echo "  doc       - 生成文档"
	@echo "  fix       - 自动修复警告"
	@echo "  release   - 构建 Release 版本"
	@echo "  status    - 显示项目状态"
	@echo "  help      - 显示此帮助信息"
	@echo ""
	@echo "示例:"
	@echo "  make build    # 构建项目"
	@echo "  make run      # 运行应用"
	@echo "  make clean    # 清理构建产物"
