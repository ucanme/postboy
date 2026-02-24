# Postboy 开发指南

## 项目概述

Postboy 是一个使用 Rust 和 GPUI 构建的 Postman 风格 API 测试工具。

## 技术栈

- **UI 框架**: GPUI v0.123.2 (Zed 的 UI 框架)
- **语言**: Rust 2021 Edition
- **数据库**: SQLite (通过 sqlx)
- **异步运行时**: Tokio
- **JavaScript 引擎**: Boa
- **HTTP 客户端**: Reqwest

## 项目结构

```
postboy/
├── crates/
│   ├── models/      # 数据模型
│   ├── store/       # 数据持久化层
│   ├── service/     # 业务逻辑层
│   ├── ui/          # UI 组件
│   └── app/         # 应用程序入口
├── Cargo.toml       # Workspace 配置
├── run.sh           # 启动脚本
└── DEVELOPMENT.md   # 本文档
```

## 快速开始

### 1. 构建项目

```bash
# 构建所有 crates
cargo build --workspace

# 或仅构建应用
cargo build --bin postboy
```

### 2. 运行应用

**方式 1: 使用启动脚本（推荐）**
```bash
./run.sh
```

**方式 2: 直接运行**
```bash
open target/debug/Postboy.app
```

**方式 3: 命令行运行**
```bash
target/debug/Postboy.app/Contents/MacOS/postboy
```

### 3. 开发迭代

```bash
# 修改代码后重新构建
cargo build --bin postboy

# .app bundle 会被自动更新，直接运行即可
./run.sh
```

## macOS 特定配置

### .app Bundle 结构

GPUI 在 macOS 上要求应用程序从 .app bundle 中运行。Bundle 结构如下：

```
Postboy.app/
├── Contents/
│   ├── Info.plist          # 应用元数据
│   ├── MacOS/
│   │   └── postboy         # 可执行文件
│   └── Resources/          # 资源文件
```

### 首次构建说明

首次构建时，`run.sh` 脚本会自动：
1. 构建二进制文件
2. 创建 .app bundle 结构
3. 生成 Info.plist
4. 复制二进制文件到正确位置

## 开发工作流

### 添加新的 UI 组件

1. 在 `crates/ui/src/` 中创建新模块
2. 实现 `Render` trait
3. 在 `crates/ui/src/lib.rs` 中导出
4. 在主窗口中使用组件

示例：
```rust
use gpui::*;

pub struct MyComponent {
    // 字段
}

impl MyComponent {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {}
    }
}

impl Render for MyComponent {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .child("Hello, Postboy!")
    }
}
```

### 添加新的数据模型

1. 在 `crates/models/src/` 中定义结构体
2. 实现必要的 trait (Serialize, Deserialize, Clone)
3. 在 `crates/models/src/lib.rs` 中导出

### 数据库操作

使用 `postboy-store` crate 提供的 API：

```rust
use postboy_store::{Database, CollectionStore};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::new(pool).await?;

    // 创建集合
    let collection = CollectionStore::create(&db, "My Collection").await?;

    Ok(())
}
```

## 常见问题

### Q: 修改代码后应用没有更新？

A: 确保重新构建了二进制文件：
```bash
cargo build --bin postboy
```

### Q: 运行时出现空指针错误？

A: 确保从 .app bundle 运行，而不是直接运行二进制文件：
```bash
# ❌ 错误方式
./target/debug/postboy

# ✅ 正确方式
open target/debug/Postboy.app
```

### Q: 如何调试应用？

A: 可以在终端中直接运行 bundle 中的二进制文件查看日志：
```bash
target/debug/Postboy.app/Contents/MacOS/postboy
```

或使用环境变量：
```bash
RUST_BACKTRACE=1 target/debug/Postboy.app/Contents/MacOS/postboy
```

## 性能优化

### 编译优化

```bash
# Release 构建
cargo build --release --bin postboy

# 更新 Release .app bundle
mkdir -p target/release/Postboy.app/Contents/MacOS
cp target/release/postboy target/release/Postboy.app/Contents/MacOS/
# 复制 Info.plist...
```

### 开发模式优化

`Cargo.toml` 中已配置了开发模式优化：
- GPUI: opt-level = 2
- Boa Engine: opt-level = 2

这平衡了编译速度和运行性能。

## 贡献指南

### 代码风格

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust 命名规范

### 测试

```bash
# 运行所有测试
cargo test --workspace

# 运行特定 crate 的测试
cargo test --package postboy-models
cargo test --package postboy-store
```

### 文档

```bash
# 生成并打开文档
cargo doc --open --workspace
```

## 下一步

### 待实现功能

- [ ] 完整的请求构建器 UI
- [ ] HTTP 请求发送
- [ ] 响应查看器
- [ ] 集合管理
- [ ] 环境变量支持
- [ ] 请求历史记录
- [ ] 测试脚本支持
- [ ] 数据导入/导出
- [ ] 云同步功能

### 路线图

**Phase 1: 基础功能** (当前)
- ✅ 项目架构
- ✅ UI 框架集成
- ✅ 数据模型
- ✅ 数据库层
- ⏳ 基本 UI 组件

**Phase 2: 核心功能**
- HTTP 请求发送
- 响应显示和格式化
- 集合和文件夹管理
- 环境变量

**Phase 3: 高级功能**
- 测试脚本
- 请求历史
- 数据导入/导出
- 云同步

## 资源链接

- [GPUI 文档](https://github.com/zed-industries/zed)
- [Zed 源码](https://github.com/zed-industries/zed)
- [Rust 文档](https://doc.rust-lang.org/)
- [Tokio 教程](https://tokio.rs/tokio/tutorial)

## 许可证

MIT License - 详见 LICENSE 文件
