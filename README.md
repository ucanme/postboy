# Postboy

<div align="center">

**一个现代化的 API 测试工具，使用 Rust 和 GPUI 构建**

[![Rust](https://img.shields.io/badge/Rust-2021%20Edition-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[功能特性](#功能特性) • [快速开始](#快速开始) • [开发指南](#开发指南) • [贡献](#贡献)

</div>

---

## 📖 简介

Postboy 是一个功能强大的 API 测试工具，旨在提供与 Postman 类似的用户体验，但使用 Rust 和 GPUI 构建，带来更好的性能和原生体验。

### ✨ 特点

- 🚀 **高性能** - 使用 Rust 构建，启动快速，运行流畅
- 🎨 **原生 UI** - 基于 GPUI (Zed 编辑器的 UI 框架)
- 💾 **离线优先** - 本地 SQLite 数据库，支持未来云同步
- 🔒 **安全可靠** - Rust 的内存安全保证
- 🧪 **脚本支持** - 集成 Boa JavaScript 引擎
- 🌐 **跨平台** - 支持 macOS、Linux 和 Windows

## 🎯 功能特性

### 已实现 ✅

- ✅ 完整的项目架构
- ✅ GPUI UI 框架集成
- ✅ 数据模型层
- ✅ SQLite 数据持久化
- ✅ HTTP 服务（完整实现）
- ✅ HTTP 请求发送（真实 API 请求）
- ✅ 响应查看器（格式化显示）
- ✅ JSON 自动格式化
- ✅ 方法选择器（GET/POST/PUT/DELETE/PATCH/HEAD/OPTIONS）
- ✅ 异步请求处理
- ✅ URL 快速预设（5个常用测试 API）
- ✅ 请求构建器面板（Headers/Body/Params 标签页）
- ✅ Headers 管理（添加常用请求头、清除、删除单个）
- ✅ Body 预设模板（7个JSON模板：Empty、User、Product、Array、Login、Address、Config）
- ✅ Query 参数管理（添加、清除、删除单个）
- ✅ Headers、Body 和 Query Params 实际应用于 HTTP 请求
- ✅ Query 参数自动追加到 URL (?key=value&key2=value2)
- ✅ 10个URL快速预设（GitHub、HTTPBin、JSONPlaceholder、Reqres、Dog API、Pokemon、Weather、CoinGecko、Joke、IP Info）
- ✅ 响应复制按钮
- ✅ 主题系统（深色/浅色）
- ✅ 应用程序打包

### 未来版本 📋

**v0.2.0 计划**
- ⏳ 数据库集成（保存请求、历史记录）
- ⏳ URL 文本框完整编辑
- ⏳ 方法下拉菜单
- ⏳ 集合管理

**v0.3.0 计划**
- ⏳ JSON 语法高亮
- ⏳ Headers 键值对实时编辑器
- ⏳ Body 实时文本编辑器
- ⏳ 环境变量管理
- ⏳ 测试脚本支持

**v0.4.0 计划**
- ⏳ Postman 集合导入/导出
- ⏳ 云同步
- ⏳ 团队协作功能

### 计划中 📋

- 📋 测试脚本支持
- 📋 请求历史记录
- 📋 数据导入/导出
- 📋 云同步功能
- 📋 WebSocket 支持
- 📋 GraphQL 支持

## 🚀 快速开始

### 环境要求

- Rust 1.75+
- macOS 10.15+ (当前版本)
- Git

### 安装

```bash
# 克隆仓库
git clone https://github.com/your-username/postboy.git
cd postboy

# 构建项目
cargo build --workspace

# 运行应用 (选择一种方式)
./run.sh          # 方式 1: 使用启动脚本
make run          # 方式 2: 使用 Makefile
./dev.sh run      # 方式 3: 使用开发工具脚本
```

### 开发工具

项目提供了三种便捷的开发工具：

#### 1. 启动脚本 (`run.sh`)
最简单的方式，直接启动应用。

```bash
./run.sh
```

#### 2. Makefile
提供丰富的开发命令。

```bash
make build    # 构建项目
make run      # 运行应用
make test     # 运行测试
make fmt      # 格式化代码
make clean    # 清理构建
make status   # 查看状态
make help     # 查看所有命令
```

#### 3. 开发工具脚本 (`dev.sh`)
功能最全的开发工具。

```bash
./dev.sh build     # 构建
./dev.sh run       # 运行
./dev.sh test      # 测试
./dev.sh fmt       # 格式化
./dev.sh clippy    # 代码检查
./dev.sh fix       # 自动修复
./dev.sh status    # 状态
./dev.sh help      # 帮助
```

### 从源代码运行

## 📁 项目结构

```
postboy/
├── crates/
│   ├── models/      # 数据模型和类型定义
│   │   ├── collection.rs
│   │   ├── request.rs
│   │   ├── response.rs
│   │   ├── environment.rs
│   │   └── ...
│   ├── store/       # 数据持久化层
│   │   ├── database.rs
│   │   ├── migrations.rs
│   │   └── ...
│   ├── service/     # 业务逻辑层
│   │   ├── http.rs
│   │   ├── collection.rs
│   │   └── ...
│   ├── ui/          # UI 组件
│   │   ├── theme/
│   │   ├── layout/
│   │   ├── request/
│   │   ├── response/
│   │   └── ...
│   └── app/         # 应用程序入口
│       └── main.rs
├── Cargo.toml       # Workspace 配置
├── run.sh           # 启动脚本
├── README.md        # 本文档
└── DEVELOPMENT.md   # 开发指南
```

## 🛠️ 开发

详细的开发指南请参阅 [DEVELOPMENT.md](DEVELOPMENT.md)。

### 常用命令

```bash
# 构建
cargo build --workspace

# 运行
./run.sh

# 测试
cargo test --workspace

# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 生成文档
cargo doc --open
```

### 添加新功能

1. 在对应的 crate 中添加代码
2. 运行 `cargo build` 检查编译
3. 运行 `./run.sh` 测试功能
4. 提交 Pull Request

## 🏗️ 架构

### 分层架构

```
┌─────────────────────────────────┐
│         UI Layer (GPUI)         │  ← 用户界面
├─────────────────────────────────┤
│       Service Layer             │  ← 业务逻辑
├─────────────────────────────────┤
│       Store Layer               │  ← 数据持久化
├─────────────────────────────────┤
│       Models Layer              │  ← 数据模型
└─────────────────────────────────┘
```

### 技术栈

| 层级 | 技术 |
|------|------|
| UI | GPUI, gpui-component |
| 业务逻辑 | Rust, Tokio |
| 数据库 | SQLite, sqlx |
| HTTP | reqwest |
| JavaScript | Boa Engine |

## 🤝 贡献

欢迎贡献！请查看 [DEVELOPMENT.md](DEVELOPMENT.md) 了解如何参与开发。

### 贡献方式

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢

- [GPUI](https://github.com/zed-industries/zed) - Zed 编辑器的 UI 框架
- [Postman](https://www.postman.com/) - API 测试工具的灵感来源
- [Rust 社区](https://www.rust-lang.org/community) - 提供了优秀的工具和生态

## 📞 联系方式

- 作者: Postboy Contributors
- 问题反馈: [GitHub Issues](https://github.com/your-username/postboy/issues)
- 讨论区: [GitHub Discussions](https://github.com/your-username/postboy/discussions)

---

<div align="center">

**⭐ 如果觉得这个项目有帮助，请给个 Star！**

Made with ❤️ by Postboy Contributors

</div>
