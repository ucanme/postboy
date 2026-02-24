# Postboy 快速参考

## 🚀 快速开始

```bash
# 运行应用
./run.sh

# 重新构建
cargo build --workspace

# 清理重建
cargo clean && cargo build --workspace
```

## 📂 关键文件

| 文件 | 用途 |
|------|------|
| `run.sh` | 启动应用 |
| `Cargo.toml` | 依赖配置 |
| `crates/app/src/main.rs` | 应用入口 |
| `crates/ui/src/layout/main_window.rs` | 主窗口布局 |
| `DEVELOPMENT.md` | 详细开发指南 |

## 🎨 当前 UI 结构

```
MainWindow
├── Sidebar (左侧)
│   └── Collections 面板
├── Main Content (中间)
│   ├── Header (顶部)
│   ├── Request Builder (上半部分)
│   └── Response Viewer (下半部分)
└── StatusBar (底部)
```

## 🛠️ 常用命令

### 开发
```bash
# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 运行测试
cargo test

# 生成文档
cargo doc --open
```

### 调试
```bash
# 查看日志
target/debug/Postboy.app/Contents/MacOS/postboy

# 带堆栈跟踪
RUST_BACKTRACE=1 target/debug/Postboy.app/Contents/MacOS/postboy
```

### 发布构建
```bash
# 构建优化版本
cargo build --release --bin postboy

# 手动创建 release .app bundle
mkdir -p target/release/Postboy.app/Contents/MacOS
cp target/release/postboy target/release/Postboy.app/Contents/MacOS/
# (需要复制 Info.plist)
```

## 📊 项目状态

### ✅ 已完成
- [x] 项目架构
- [x] 数据模型层
- [x] 数据持久化层
- [x] UI 框架集成
- [x] 基础 UI 组件
- [x] 主题系统
- [x] 应用打包

### 🚧 开发中
- [ ] HTTP 请求发送
- [ ] 请求构建器 UI
- [ ] 响应查看器
- [ ] 集合管理
- [ ] 环境变量

### 📋 计划中
- [ ] 测试脚本
- [ ] 请求历史
- [ ] 数据导入/导出
- [ ] 云同步

## 🔧 故障排除

### 应用无法启动
```bash
# 确保从 .app bundle 运行
open target/debug/Postboy.app  # ✅ 正确
./target/debug/postboy          # ❌ 错误 (macOS)
```

### 编译错误
```bash
# 清理并重新构建
cargo clean
cargo build --workspace
```

### 依赖问题
```bash
# 更新依赖
cargo update

# 重新解析
rm Cargo.lock
cargo build
```

## 📝 代码结构速查

### 添加新 UI 组件
```rust
// 1. 在 crates/ui/src/ 创建文件
// 2. 定义结构体
pub struct MyComponent;

// 3. 实现 Render
impl Render for MyComponent {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().child("Content")
    }
}

// 4. 在 lib.rs 导出
```

### 添加数据模型
```rust
// 在 crates/models/src/ 添加
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyModel {
    pub field: String,
}
```

### 数据库操作
```rust
use postboy_store::Database;

// 使用已有的 store API
let collections = CollectionStore::list(&db).await?;
```

## 🎯 下一步

1. **完善请求构建器**
   - 实现方法选择
   - 添加 URL 输入
   - Headers 编辑器

2. **实现 HTTP 请求**
   - 集成 reqwest
   - 显示响应
   - 错误处理

3. **完善 UI 交互**
   - 添加按钮点击事件
   - 表单验证
   - 状态管理

## 📞 获取帮助

- 📖 阅读 [DEVELOPMENT.md](DEVELOPMENT.md)
- 📝 查看 [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)
- 💻 检查示例代码: `~/.cargo/git/checkouts/zed-*/crates/gpui/examples/`

---

**提示**: 运行 `./run.sh` 启动应用！
