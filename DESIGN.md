# Postboy 设计方案 (Rust + GPUI)

## 1. 项目概述

Postboy 是一个使用 **Rust + GPUI** 构建的高性能 API 测试工具，类似 Postman，具有原生应用的性能和 Web 应用的灵活性。

### 核心特性
- 🚀 **高性能**: 基于 GPUI 的 GPU 加速渲染
- 🔌 **API 测试**: 完整的 HTTP/HTTPS 请求支持
- 📜 **脚本 Hook**: JavaScript 请求前/后脚本
- 📁 **目录管理**: Collection 和 Folder 组织
- 💾 **离线优先**: 本地 SQLite 存储，可选云同步
- 🤖 **MCP 支持**: Model Context Protocol 集成

### 相关文档
- [API 设计文档](./API_DESIGN.md) - 详细的脚本 Hook API、MCP 协议实现、内部服务 API 和存储 API

---

## 2. 技术架构

### 2.1 技术栈

| 层级 | 技术 | 说明 |
|------|------|------|
| **UI 框架** | GPUI | Zoom 开发的高性能 Rust GUI 框架 |
| **语言** | Rust | 系统级性能和内存安全 |
| **脚本引擎** | Boa / QuickJS | JavaScript 执行环境 |
| **HTTP 客户端** | Reqwest | 异步 HTTP 客户端 |
| **数据库** | SQLite / SurrealDB | 本地数据持久化 |
| **序列化** | Serde | 高效的序列化/反序列化 |
| **异步运行时** | Tokio | 异步 I/O 和定时器 |
| **代码编辑器** | Druid 内嵌 / Syntect | 语法高亮编辑器 |
| **MCP 协议** | 自实现 | MCP Server/Client |

### 2.2 架构分层

```
┌─────────────────────────────────────────────────────────────┐
│                      UI Layer (GPUI)                        │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐      │
│  │ Sidebar  │ │ Request  │ │ Response │ │ Script   │      │
│  │  Tree    │ │  Panel   │ │  Panel   │ │  Editor  │      │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘      │
└─────────────────────────────────────────────────────────────┘
                              ↕
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────────────┐ │
│  │ Request      │ │ Collection   │ │ Script Execution     │ │
│  │ Manager      │ │ Manager      │ │ Engine               │ │
│  └──────────────┘ └──────────────┘ └──────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              ↕
┌─────────────────────────────────────────────────────────────┐
│                      Service Layer                          │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐   │
│  │ HTTP     │ │ Storage  │ │ Sync     │ │ MCP          │   │
│  │ Service  │ │ Service  │ │ Service  │ │ Service      │   │
│  └──────────┘ └──────────┘ └──────────┘ └──────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              ↕
┌─────────────────────────────────────────────────────────────┐
│                      Data Layer                             │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐   │
│  │ SQLite   │ │ File     │ │ Memory   │ │ HTTP Cache  │   │
│  │ Database │ │ System   │ │ Store    │ │              │   │
│  └──────────┘ └──────────┘ └──────────┘ └──────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. 项目结构

```
postboy/
├── Cargo.toml                    # 项目配置
├── Cargo.lock                    # 依赖锁定
├── README.md
├── DESIGN.md
│
├── src/                          # 源代码
│   ├── main.rs                   # 应用入口
│   ├── app.rs                    # 应用根组件
│   │
│   ├── ui/                       # UI 组件 (GPUI)
│   │   ├── mod.rs
│   │   ├── layout/
│   │   │   ├── mod.rs
│   │   │   ├── main_window.rs   # 主窗口
│   │   │   ├── sidebar.rs       # 侧边栏
│   │   │   ├── header.rs        # 顶部栏
│   │   │   └── status_bar.rs    # 状态栏
│   │   ├── request/
│   │   │   ├── mod.rs
│   │   │   ├── request_bar.rs   # 请求栏
│   │   │   ├── method_dropdown.rs
│   │   │   ├── url_input.rs
│   │   │   ├── tabs.rs          # 请求选项卡
│   │   │   ├── params_view.rs   # 参数视图
│   │   │   ├── headers_view.rs  # 头部视图
│   │   │   ├── body_view.rs     # 请求体视图
│   │   │   └── auth_view.rs     # 认证视图
│   │   ├── response/
│   │   │   ├── mod.rs
│   │   │   ├── response_panel.rs
│   │   │   ├── response_body.rs
│   │   │   ├── response_headers.rs
│   │   │   └── test_results.rs  # 测试结果视图
│   │   ├── collection/
│   │   │   ├── mod.rs
│   │   │   ├── tree_view.rs     # 集合树视图
│   │   │   ├── collection_item.rs
│   │   │   └── drag_drop.rs     # 拖拽支持
│   │   ├── editor/
│   │   │   ├── mod.rs
│   │   │   ├── code_editor.rs   # 代码编辑器
│   │   │   └── script_editor.rs # 脚本编辑器
│   │   ├── environment/
│   │   │   ├── mod.rs
│   │   │   ├── env_modal.rs     # 环境变量弹窗
│   │   │   └── env_manager.rs
│   │   └── common/
│   │       ├── mod.rs
│   │       ├── button.rs
│   │       ├── input.rs
│   │       ├── table.rs
│   │       └── modal.rs
│   │
│   ├── stores/                   # 状态管理
│   │   ├── mod.rs
│   │   ├── request_store.rs     # 请求状态
│   │   ├── collection_store.rs  # 集合状态
│   │   ├── environment_store.rs # 环境变量状态
│   │   ├── settings_store.rs    # 设置状态
│   │   └── response_store.rs    # 响应状态
│   │
│   ├── services/                 # 业务逻辑
│   │   ├── mod.rs
│   │   ├── http.rs              # HTTP 服务
│   │   ├── storage.rs           # 存储服务
│   │   ├── script.rs            # 脚本执行服务
│   │   ├── sync.rs              # 同步服务
│   │   ├── export.rs            # 导入导出
│   │   └── mcp.rs               # MCP 服务
│   │
│   ├── models/                   # 数据模型
│   │   ├── mod.rs
│   │   ├── request.rs
│   │   ├── collection.rs
│   │   ├── environment.rs
│   │   ├── response.rs
│   │   ├── script.rs
│   │   └── mcp.rs
│   │
│   ├── script_engine/            # 脚本引擎
│   │   ├── mod.rs
│   │   ├── context.rs           # pm 对象上下文
│   │   ├── runtime.rs           # 运行时
│   │   └── sandbox.rs           # 沙箱环境
│   │
│   ├── mcp/                      # MCP 实现
│   │   ├── mod.rs
│   │   ├── server.rs            # MCP Server
│   │   ├── client.rs            # MCP Client
│   │   ├── transport.rs         # 传输层
│   │   ├── protocol.rs          # 协议定义
│   │   └── tools/               # MCP 工具
│   │       ├── mod.rs
│   │       ├── send_request.rs
│   │       ├── list_collections.rs
│   │       └── get_request.rs
│   │
│   ├── utils/                    # 工具函数
│   │   ├── mod.rs
│   │   ├── variable.rs          # 变量解析
│   │   ├── formatter.rs         # 格式化
│   │   └── crypto.rs            # 加密工具
│   │
│   └── theme/                    # 主题
│       ├── mod.rs
│       ├── colors.rs
│       └── typography.rs
│
├── resources/                    # 资源文件
│   ├── icons/
│   └── themes/
│
├── migrations/                   # 数据库迁移
│   └── init.sql
│
└── tests/                        # 测试
    ├── integration/
    └── unit/
```

---

## 4. 数据模型

### 4.1 核心数据结构

```rust
// 请求模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<Header>,
    pub params: Vec<Param>,
    pub body: RequestBody,
    pub auth: Option<AuthConfig>,
    pub pre_request_script: Option<String>,
    pub post_response_script: Option<String>,
    pub collection_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestBody {
    None,
    Json(serde_json::Value),
    FormData(Vec<FormField>),
    UrlEncoded(Vec<FormField>),
    Raw(String),
    Binary(Vec<u8>),
}

// 集合模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub auth: Option<AuthConfig>,
    pub variables: Vec<Variable>,
    pub folders: Vec<Folder>,
    pub requests: Vec<Request>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub children: Vec<Folder>,
    pub requests: Vec<Request>,
}

// 环境变量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub id: Uuid,
    pub name: String,
    pub values: Vec<Variable>,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub key: String,
    pub value: String,
    pub enabled: bool,
    pub variable_type: VariableType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    Normal,
    Secret,
}

// 响应模型
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<Header>,
    pub body: ResponseBody,
    pub duration: u64,
    pub size: u64,
    pub test_results: Vec<TestResult>,
}

#[derive(Debug, Clone)]
pub enum ResponseBody {
    Json(serde_json::Value),
    Text(String),
    Binary(Vec<u8>),
}
```

---

## 5. 核心功能设计

### 5.1 HTTP 请求服务

```rust
// src/services/http.rs

use reqwest::Client;
use tokio::time::Instant;

pub struct HttpService {
    client: Client,
}

impl HttpService {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap(),
        }
    }

    pub async fn send_request(
        &self,
        request: &Request,
        variables: &HashMap<String, String>,
    ) -> Result<Response, HttpError> {
        let start = Instant::now();
        
        // 1. 解析变量
        let resolved_url = self.resolve_variables(&request.url, variables)?;
        let resolved_headers = self.resolve_headers(&request.headers, variables)?;
        
        // 2. 构建 HTTP 请求
        let mut req = self.client.request(
            request.method.into(),
            resolved_url,
        );
        
        // 添加 headers
        for header in resolved_headers {
            req = req.header(&header.key, &header.value);
        }
        
        // 添加 body
        if let Some(body) = request.body.to_bytes() {
            req = req.body(body);
        }
        
        // 3. 发送请求
        let resp = req.send().await?;
        let status = resp.status();
        let headers = resp.headers().clone();
        let body = resp.bytes().await?;
        
        let duration = start.elapsed().as_millis() as u64;
        
        // 4. 解析响应
        Ok(Response {
            status: status.as_u16(),
            status_text: status.canonical_reason().unwrap_or("Unknown").to_string(),
            headers: Self::parse_headers(headers),
            body: Self::parse_body(body)?,
            duration,
            size: body.len() as u64,
            test_results: vec![],
        })
    }
    
    fn resolve_variables(&self, text: &str, vars: &HashMap<String, String>) -> Result<String, HttpError> {
        // 解析 {{variable}} 语法
        let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
        let result = re.replace_all(text, |caps: &regex::Captures| {
            let key = &caps[1];
            vars.get(key).map(|s| s.as_str()).unwrap_or("")
        });
        Ok(result.to_string())
    }
}
```

### 5.2 脚本执行引擎

```rust
// src/script_engine/runtime.rs

use boa_engine::{Context, Source};
use boa_gc::Gc;

pub struct ScriptRuntime {
    context: Context,
}

impl ScriptRuntime {
    pub fn new() -> Self {
        let context = Context::default();
        Self { context }
    }
    
    pub fn execute_pre_request(
        &mut self,
        script: &str,
        context: &ScriptContext,
    ) -> Result<(), ScriptError> {
        // 注入 pm 对象
        self.inject_pm_object(context);
        
        // 执行脚本
        let source = Source::from_bytes(script);
        self.context.eval(source)?;
        
        // 提取修改后的值
        self.extract_context(context)?;
        
        Ok(())
    }
    
    pub fn execute_post_response(
        &mut self,
        script: &str,
        context: &ScriptContext,
        response: &Response,
    ) -> Result<Vec<TestResult>, ScriptError> {
        // 注入 pm 对象（包含 response）
        self.inject_pm_object_with_response(context, response);
        
        // 执行脚本
        let source = Source::from_bytes(script);
        self.context.eval(source)?;
        
        // 收集测试结果
        Ok(self.extract_test_results())
    }
    
    fn inject_pm_object(&mut self, ctx: &ScriptContext) {
        // 创建 pm 对象并注入到 JS 环境
        // pm.environment
        // pm.globals
        // pm.request
        // pm.sendRequest()
    }
}
```

### 5.3 存储服务

```rust
// src/services/storage.rs

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use sqlx::migrate::MigrateDatabase;

pub struct StorageService {
    pool: SqlitePool,
}

impl StorageService {
    pub async fn new(db_path: &str) -> Result<Self, StorageError> {
        // 创建数据库
        if !Sqlite::database_exists(db_path).await.unwrap_or(false) {
            Sqlite::create_database(db_path).await?;
        }
        
        // 连接池
        let pool = SqlitePool::connect(db_path).await?;
        
        // 运行迁移
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Self { pool })
    }
    
    // Collection 操作
    pub async fn create_collection(&self, col: &Collection) -> Result<(), StorageError> {
        sqlx::query!(
            "INSERT INTO collections (id, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
            col.id, col.name, col.description, col.created_at, col.updated_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    
    pub async fn list_collections(&self) -> Result<Vec<Collection>, StorageError> {
        let rows = sqlx::query_as::<_, CollectionRow>(
            "SELECT * FROM collections ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        // 转换为 Collection...
    }
    
    // Request 操作
    pub async fn save_request(&self, req: &Request) -> Result<(), StorageError> {
        // UPSERT 逻辑
    }
    
    pub async fn get_request(&self, id: Uuid) -> Result<Option<Request>, StorageError> {
        // 查询逻辑
    }
    
    // Environment 操作
    pub async fn save_environment(&self, env: &Environment) -> Result<(), StorageError> {
        // 保存环境变量
    }
    
    pub async fn get_active_environment(&self) -> Result<Option<Environment>, StorageError> {
        // 获取当前激活的环境
    }
}
```

### 5.4 MCP 服务

```rust
// src/mcp/server.rs

use serde_json::Value;
use tokio::net::UnixListener;

pub struct McpServer {
    tools: Vec<Tool>,
    resources: Vec<Resource>,
}

#[derive(Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

impl McpServer {
    pub fn new() -> Self {
        Self {
            tools: vec![
                Tool {
                    name: "send_request".to_string(),
                    description: "Send an HTTP request".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "method": {"type": "string"},
                            "url": {"type": "string"},
                            "headers": {"type": "object"},
                            "body": {"type": "object"}
                        }
                    }),
                },
                Tool {
                    name: "list_collections".to_string(),
                    description: "List all API collections".to_string(),
                    input_schema: json!({"type": "object"}),
                },
            ],
            resources: vec![],
        }
    }
    
    pub async fn run_stdio(&self) -> Result<(), McpError> {
        // 从 stdin 读取请求，写入 stdout 响应
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        
        let mut reader = BufReader::new(stdin);
        let mut writer = BufWriter::new(stdout);
        
        loop {
            // 读取 JSON-RPC 请求
            let request = self.read_message(&mut reader).await?;
            
            // 处理请求
            let response = match request.method.as_str() {
                "tools/list" => self.handle_list_tools(request).await?,
                "tools/call" => self.handle_tool_call(request).await?,
                "initialize" => self.handle_initialize(request).await?,
                _ => Err(McpError::MethodNotFound),
            };
            
            // 写入响应
            self.write_message(&mut writer, &response).await?;
        }
    }
    
    async fn handle_tool_call(&self, request: JsonRpcRequest) -> Result<JsonRpcResponse, McpError> {
        let tool_name = request.params.get("name")
            .and_then(|v| v.as_str())
            .ok_or(McpError::InvalidParams)?;
            
        let arguments = request.params.get("arguments");
        
        match tool_name {
            "send_request" => {
                // 调用 HTTP 服务发送请求
                let result = self.execute_send_request(arguments).await?;
                Ok(JsonRpcResponse::success(request.id, result))
            }
            "list_collections" => {
                // 获取所有集合
                let result = self.execute_list_collections().await?;
                Ok(JsonRpcResponse::success(request.id, result))
            }
            _ => Err(McpError::MethodNotFound),
        }
    }
}
```

---

## 6. UI 组件设计

### 6.1 主窗口布局

```rust
// src/ui/layout/main_window.rs

use gpui::*;

pub struct MainWindow {
    sidebar: View<Sidebar>,
    request_panel: View<RequestPanel>,
    response_panel: View<ResponsePanel>,
}

impl MainWindow {
    pub fn new(window: &mut WindowContext) -> Self {
        let sidebar = Sidebar::new(window);
        let request_panel = RequestPanel::new(window);
        let response_panel = ResponsePanel::new(window);
        
        Self {
            sidebar,
            request_panel,
            response_panel,
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .child(self.sidebar.clone())
            .child(
                div()
                    .flex_1()
                    .flex_col()
                    .child(self.request_panel.clone())
                    .child(self.response_panel.clone())
            )
    }
}
```

### 6.2 请求面板

```rust
// src/ui/request/request_bar.rs

pub struct RequestBar {
    method: HttpMethod,
    url: String,
    send_button: bool,
}

impl RequestBar {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            method: HttpMethod::GET,
            url: String::new(),
            send_button: false,
        }
    }
    
    pub fn set_url(&mut self, url: String, cx: &mut ViewContext<Self>) {
        self.url = url;
        cx.notify();
    }
    
    fn on_send(&mut self, cx: &mut ViewContext<Self>) {
        // 触发请求发送
        cx.emit(RequestEvent::Send);
    }
}

impl Render for RequestBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .p_2()
            .gap_2()
            .child(self.render_method_dropdown(cx))
            .child(self.render_url_input(cx))
            .child(
                div()
                    .px_4()
                    .py_2()
                    .bg(rgb(0x007acc))
                    .rounded_md()
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, cx| this.on_send(cx)))
                    .child("Send")
            )
    }
}
```

---

## 7. 请求执行流程

```
┌─────────────────────────────────────────────────────────────┐
│ 1. 用户点击 Send 按钮                                        │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ 2. 收集请求配置                                              │
│    - 方法、URL、Headers、Body                                │
│    - 从环境变量中获取当前激活的环境                          │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ 3. 变量解析                                                  │
│    - 解析 URL 中的 {{variable}}                              │
│    - 解析 Headers 中的变量                                   │
│    - 解析 Body 中的变量                                      │
└─────────────────────────────────────────────────────────────┐
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ 4. 执行 Pre-request Script                                  │
│    - 创建 JS 沙箱环境                                        │
│    - 注入 pm 对象                                            │
│    - 执行用户脚本                                            │
│    - 收集修改后的变量和请求配置                              │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ 5. 发送 HTTP 请求                                           │
│    - 使用 reqwest 客户端                                     │
│    - 记录开始时间                                            │
│    - 显示 Loading 状态                                       │
└─────────────────────────────────────────────────────────────┐
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ 6. 接收响应                                                  │
│    - 解析状态码、Headers、Body                               │
│    - 计算请求耗时                                            │
│    - 格式化响应内容                                          │
└─────────────────────────────────────────────────────────────┐
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ 7. 执行 Post-response Script                                │
│    - 注入 pm.response 对象                                  │
│    - 执行测试断言                                            │
│    - 收集测试结果                                            │
│    - 更新环境变量                                            │
└─────────────────────────────────────────────────────────────┐
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ 8. 显示响应                                                  │
│    - 更新响应面板                                            │
│    - 显示测试结果                                            │
│    - 保存到历史记录                                          │
└─────────────────────────────────────────────────────────────┘
```

---

## 8. MCP 集成方案

### 8.1 MCP Server 模式

Postboy 作为 MCP Server，为 AI 模型提供 API 测试能力：

```rust
// 启动 MCP Server
#[tokio::main]
async fn main() -> Result<()> {
    let server = McpServer::new();
    
    // 支持 stdio 传输
    server.run_stdio().await?;
    
    Ok(())
}
```

### 8.2 暴露的工具

| 工具名 | 描述 | 参数 |
|--------|------|------|
| `send_request` | 发送 HTTP 请求 | method, url, headers, body |
| `list_collections` | 列出所有集合 | - |
| `get_request` | 获取特定请求 | request_id |
| `create_request` | 创建新请求 | request_data |
| `update_request` | 更新请求 | request_id, request_data |
| `delete_request` | 删除请求 | request_id |
| `set_environment` | 设置环境变量 | environment_data |
| `run_test` | 运行测试集合 | collection_id |

### 8.3 暴露的资源

| URI | 名称 | 描述 |
|-----|------|------|
| `postboy://collections` | 所有集合 | 完整的 API 集合列表 |
| `postboy://environments` | 环境变量 | 当前环境配置 |
| `postboy://history` | 请求历史 | 最近的请求记录 |

---

## 9. 离线/在线模式

### 9.1 离线模式 (默认)

- **本地存储**: SQLite 数据库存储所有数据
- **自动保存**: 每次修改立即持久化
- **无网络依赖**: 完全离线可用
- **导入/导出**: 支持 Postman Collection v2.1 格式

### 9.2 在线模式 (可选)

- **云端同步**: 可选的云存储后端
- **增量同步**: 只同步变更的数据
- **冲突解决**: 最后写入胜出或手动合并
- **协作功能**: 团队共享集合
### 9.2 在线模式 (可选)

#### 9.2.1 同步架构设计

在线模式提供云端同步和团队协作功能，支持多种同步策略：

```
┌─────────────────────────────────────────────────────────────────┐
│                        Sync Architecture                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐     │
│  │   Client 1   │    │   Client 2   │    │   Client N   │     │
│  │  (Postboy)   │    │  (Postboy)   │    │  (Postboy)   │     │
│  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘     │
│         │                   │                   │              │
│         └───────────────────┼───────────────────┘              │
│                             │                                  │
│                    ┌────────▼────────┐                         │
│                    │  Sync Service   │                         │
│                    │  (WebSocket)    │                         │
│                    └────────┬────────┘                         │
│                             │                                  │
│                    ┌────────▼────────┐                         │
│                    │   Sync Server   │                         │
│                    │                 │                         │
│                    │  ┌───────────┐  │                         │
│                    │  │  Auth     │  │                         │
│                    │  │  Service  │  │                         │
│                    │  └───────────┘  │                         │
│                    │  ┌───────────┐  │                         │
│                    │  │  Sync     │  │                         │
│                    │  │  Engine   │  │                         │
│                    │  └───────────┘  │                         │
│                    │  ┌───────────┐  │                         │
│                    │  │  Conflict │  │                         │
│                    │  │ Resolver  │  │                         │
│                    │  └───────────┘  │                         │
│                    └────────┬────────┘                         │
│                             │                                  │
│                    ┌────────▼────────┐                         │
│                    │   Database      │                         │
│                    │   (PostgreSQL)  │                         │
│                    └─────────────────┘                         │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

#### 9.2.2 同步模式

```rust
// src/services/sync/mod.rs

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 同步模式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncMode {
    /// 完全离线模式
    Offline,
    
    /// 在线模式 - 自动同步
    OnlineAuto,
    
    /// 在线模式 - 手动同步
    OnlineManual,
    
    /// 混合模式 - 本地优先，定期同步
    Hybrid,
}

/// 同步状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    Idle,
    Syncing,
    Success { timestamp: i64 },
    Error { message: String },
    Conflict { conflicts: Vec<ConflictInfo> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub item_type: String,
    pub item_id: String,
    pub local_version: i64,
    pub remote_version: i64,
    pub local_value: serde_json::Value,
    pub remote_value: serde_json::Value,
}

/// 同步服务
pub struct SyncService {
    /// 当前同步模式
    mode: RwLock<SyncMode>,
    
    /// 本地存储
    local_storage: Arc<StorageService>,
    
    /// 远程客户端
    remote_client: Option<Arc<RemoteSyncClient>>,
    
    /// 同步状态
    status: Arc<RwLock<SyncStatus>>,
    
    /// 事件总线
    event_bus: Arc<EventBus>,
    
    /// 冲突解决策略
    conflict_strategy: ConflictStrategy,
}

/// 冲突解决策略
#[derive(Debug, Clone, Copy)]
pub enum ConflictStrategy {
    /// 本地优先（覆盖远程）
    LocalWins,
    
    /// 远程优先（覆盖本地）
    RemoteWins,
    
    /// 最后写入胜出（基于时间戳）
    LastWriteWins,
    
    /// 手动解决（提示用户选择）
    Manual,
}

impl SyncService {
    pub fn new(
        local_storage: Arc<StorageService>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            mode: RwLock::new(SyncMode::Offline),
            local_storage,
            remote_client: None,
            status: Arc::new(RwLock::new(SyncStatus::Idle)),
            event_bus,
            conflict_strategy: ConflictStrategy::LastWriteWins,
        }
    }
    
    /// 配置在线模式
    pub async fn configure_online(
        &self,
        server_url: String,
        api_key: String,
        mode: SyncMode,
    ) -> Result<(), SyncError> {
        // 创建远程客户端
        let client = RemoteSyncClient::new(server_url, api_key)?;
        
        // 验证连接
        client.health_check().await?;
        
        // 更新状态
        *self.mode.write().await = mode;
        self.remote_client = Some(Arc::new(client));
        
        // 发布事件
        self.event_bus.publish(AppEvent::SyncModeChanged(mode));
        
        Ok(())
    }
    
    /// 执行同步
    pub async fn sync(&self) -> Result<SyncResult, SyncError> {
        let mode = *self.mode.read().await;
        
        match mode {
            SyncMode::Offline => {
                Ok(SyncResult::Offline)
            }
            SyncMode::OnlineAuto | SyncMode::OnlineManual | SyncMode::Hybrid => {
                self.perform_sync().await
            }
        }
    }
    
    /// 执行实际的同步操作
    async fn perform_sync(&self) -> Result<SyncResult, SyncError> {
        let client = self.remote_client.as_ref()
            .ok_or(SyncError::NotConfigured)?;
        
        // 更新状态为同步中
        *self.status.write().await = SyncStatus::Syncing;
        self.event_bus.publish(AppEvent::SyncStarted);
        
        let result = async {
            // 1. 获取本地变更
            let local_changes = self.local_storage.get_pending_changes().await?;
            
            // 2. 推送本地变更到服务器
            let push_result = client.push_changes(local_changes).await?;
            
            // 3. 从服务器拉取远程变更
            let remote_changes = client.pull_changes().await?;
            
            // 4. 检测并解决冲突
            let conflicts = self.detect_conflicts(&remote_changes).await?;
            
            if !conflicts.is_empty() {
                match self.conflict_strategy {
                    ConflictStrategy::LocalWins => {
                        self.resolve_conflicts_local_wins(&conflicts).await?;
                    }
                    ConflictStrategy::RemoteWins => {
                        self.resolve_conflicts_remote_wins(&conflicts).await?;
                    }
                    ConflictStrategy::LastWriteWins => {
                        self.resolve_conflicts_last_write_wins(&conflicts).await?;
                    }
                    ConflictStrategy::Manual => {
                        // 返回冲突信息，等待用户处理
                        *self.status.write().await = SyncStatus::Conflict { 
                            conflicts: conflicts.clone(),
                        };
                        return Ok(SyncResult::Conflict { conflicts });
                    }
                }
            }
            
            // 5. 应用远程变更到本地
            self.local_storage.apply_remote_changes(remote_changes).await?;
            
            // 6. 确认同步完成
            client.acknowledge_sync(push_result.sync_id).await?;
            
            Ok(SyncResult::Success {
                timestamp: chrono::Utc::now().timestamp_millis(),
                changes_pushed: push_result.changes_count,
                changes_pulled: remote_changes.len(),
            })
        }.await;
        
        // 更新最终状态
        match &result {
            Ok(success) => {
                *self.status.write().await = SyncStatus::Success {
                    timestamp: chrono::Utc::now().timestamp_millis(),
                };
                self.event_bus.publish(AppEvent::SyncCompleted(success.clone()));
            }
            Err(e) => {
                *self.status.write().await = SyncStatus::Error {
                    message: e.to_string(),
                };
                self.event_bus.publish(AppEvent::SyncFailed(e.clone()));
            }
        }
        
        result
    }
    
    /// 检测冲突
    async fn detect_conflicts(
        &self,
        remote_changes: &[SyncChange],
    ) -> Result<Vec<ConflictInfo>, SyncError> {
        let mut conflicts = Vec::new();
        
        for change in remote_changes {
            // 检查本地是否存在相同 ID 的项
            if let Some(local_item) = self.local_storage
                .get_item_by_id(&change.item_type, &change.item_id).await?
            {
                let local_version: i64 = local_item.get("version")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                
                // 如果版本号不同，说明存在冲突
                if local_version != change.version - 1 {
                    conflicts.push(ConflictInfo {
                        item_type: change.item_type.clone(),
                        item_id: change.item_id.clone(),
                        local_version,
                        remote_version: change.version,
                        local_value: local_item,
                        remote_value: change.data.clone(),
                    });
                }
            }
        }
        
        Ok(conflicts)
    }
    
    /// 解决冲突 - 本地优先
    async fn resolve_conflicts_local_wins(
        &self,
        conflicts: &[ConflictInfo],
    ) -> Result<(), SyncError> {
        for conflict in conflicts {
            // 推送本地版本到服务器，覆盖远程
            self.remote_client.as_ref()
                .unwrap()
                .push_item(
                    &conflict.item_type,
                    &conflict.item_id,
                    &conflict.local_value,
                    conflict.local_version + 1,
                )
                .await?;
        }
        Ok(())
    }
    
    /// 解决冲突 - 远程优先
    async fn resolve_conflicts_remote_wins(
        &self,
        conflicts: &[ConflictInfo],
    ) -> Result<(), SyncError> {
        // 直接应用远程变更
        for conflict in conflicts {
            self.local_storage
                .update_item(
                    &conflict.item_type,
                    &conflict.item_id,
                    &conflict.remote_value,
                    conflict.remote_version,
                )
                .await?;
        }
        Ok(())
    }
    
    /// 解决冲突 - 最后写入胜出
    async fn resolve_conflicts_last_write_wins(
        &self,
        conflicts: &[ConflictInfo],
    ) -> Result<(), SyncError> {
        for conflict in conflicts {
            // 比较时间戳，选择较新的版本
            let local_timestamp = conflict.local_value.get("updated_at")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            
            let remote_timestamp = conflict.remote_value.get("updated_at")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            
            if remote_timestamp > local_timestamp {
                // 远程更新，应用到本地
                self.local_storage
                    .update_item(
                        &conflict.item_type,
                        &conflict.item_id,
                        &conflict.remote_value,
                        conflict.remote_version,
                    )
                    .await?;
            } else {
                // 本地更新，推送到远程
                self.remote_client.as_ref()
                    .unwrap()
                    .push_item(
                        &conflict.item_type,
                        &conflict.item_id,
                        &conflict.local_value,
                        conflict.local_version + 1,
                    )
                    .await?;
            }
        }
        Ok(())
    }
}

/// 同步结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncResult {
    Offline,
    Success {
        timestamp: i64,
        changes_pushed: usize,
        changes_pulled: usize,
    },
    Conflict {
        conflicts: Vec<ConflictInfo>,
    },
}

/// 同步变更
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncChange {
    pub item_type: String,
    pub item_id: String,
    pub version: i64,
    pub operation: SyncOperation,
    pub data: serde_json::Value,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncOperation {
    Create,
    Update,
    Delete,
}

/// 同步错误
#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("Sync not configured")]
    NotConfigured,
    
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Server error: {0}")]
    ServerError(String),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),
}
```

#### 9.2.3 远程同步客户端

```rust
// src/services/sync/client.rs

use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 远程同步客户端
pub struct RemoteSyncClient {
    /// HTTP 客户端
    client: Client,
    
    /// 服务器 URL
    server_url: String,
    
    /// API 密钥
    api_key: String,
    
    /// 设备 ID
    device_id: String,
    
    /// 最后同步时间戳
    last_sync: Arc<std::sync::RwLock<Option<i64>>>,
}

impl RemoteSyncClient {
    pub fn new(server_url: String, api_key: String) -> Result<Self, SyncError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        
        let device_id = Self::get_or_create_device_id()?;
        
        Ok(Self {
            client,
            server_url,
            api_key,
            device_id,
            last_sync: Arc::new(std::sync::RwLock::new(None)),
        })
    }
    
    /// 获取或创建设备 ID
    fn get_or_create_device_id() -> Result<String, SyncError> {
        // 尝试从本地配置读取
        if let Ok(config) = std::fs::read_to_string("postboy_config.json") {
            if let Ok(config) = serde_json::from_str::<Config>(&config) {
                return Ok(config.device_id);
            }
        }
        
        // 生成新的设备 ID
        let device_id = uuid::Uuid::new_v4().to_string();
        
        // 保存配置
        let config = Config {
            device_id: device_id.clone(),
        };
        std::fs::write("postboy_config.json", 
            serde_json::to_string_pretty(&config)?)?;
        
        Ok(device_id)
    }
    
    /// 健康检查
    pub async fn health_check(&self) -> Result<(), SyncError> {
        let response = self.client
            .get(format!("{}/health", self.server_url))
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            Err(SyncError::ConnectionFailed(
                "Health check failed".into()
            ))
        }
    }
    
    /// 推送变更到服务器
    pub async fn push_changes(
        &self,
        changes: Vec<SyncChange>,
    ) -> Result<PushResult, SyncError> {
        let request = PushRequest {
            device_id: self.device_id.clone(),
            changes,
        };
        
        let response = self.client
            .post(format!("{}/sync/push", self.server_url))
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&request)
            .send()
            .await?
            .error_for_status()?;
        
        let result: PushResponse = response.json().await?;
        Ok(PushResult {
            sync_id: result.sync_id,
            changes_count: result.accepted_count,
        })
    }
    
    /// 拉取服务器变更
    pub async fn pull_changes(&self) -> Result<Vec<SyncChange>, SyncError> {
        let last_sync = *self.last_sync.read().unwrap();
        
        let response = self.client
            .get(format!("{}/sync/pull", self.server_url))
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .query(&[
                ("device_id", &self.device_id),
                ("since", &last_sync.map(|t| t.to_string()).unwrap_or("0".to_string())),
            ])
            .send()
            .await?
            .error_for_status()?;
        
        let result: PullResponse = response.json().await?;
        
        // 更新最后同步时间
        *self.last_sync.write().unwrap() = Some(result.timestamp);
        
        Ok(result.changes)
    }
    
    /// 确认同步完成
    pub async fn acknowledge_sync(
        &self,
        sync_id: String,
    ) -> Result<(), SyncError> {
        self.client
            .post(format!("{}/sync/acknowledge", self.server_url))
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(&AcknowledgeRequest { sync_id })
            .send()
            .await?
            .error_for_status()?;
        
        Ok(())
    }
    
    /// 推送单个项目（用于冲突解决）
    pub async fn push_item(
        &self,
        item_type: &str,
        item_id: &str,
        data: &serde_json::Value,
        version: i64,
    ) -> Result<(), SyncError> {
        let request = PushItemRequest {
            device_id: self.device_id.clone(),
            item_type: item_type.to_string(),
            item_id: item_id.to_string(),
            data: data.clone(),
            version,
        };
        
        self.client
            .post(format!("{}/sync/item", self.server_url))
            .header(header::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?
            .error_for_status()?;
        
        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct PushRequest {
    device_id: String,
    changes: Vec<SyncChange>,
}

#[derive(Debug, Deserialize)]
struct PushResponse {
    sync_id: String,
    accepted_count: usize,
}

#[derive(Debug)]
pub struct PushResult {
    pub sync_id: String,
    pub changes_count: usize,
}

#[derive(Debug, Deserialize)]
struct PullResponse {
    timestamp: i64,
    changes: Vec<SyncChange>,
}

#[derive(Debug, Serialize)]
struct AcknowledgeRequest {
    sync_id: String,
}

#[derive(Debug, Serialize)]
struct PushItemRequest {
    device_id: String,
    item_type: String,
    item_id: String,
    data: serde_json::Value,
    version: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    device_id: String,
}
```

#### 9.2.4 WebSocket 实时同步

```rust
// src/services/sync/websocket.rs

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// WebSocket 同步客户端
pub struct WebSocketSyncClient {
    /// WebSocket URL
    ws_url: String,
    
    /// API 密钥
    api_key: String,
    
    /// 设备 ID
    device_id: String,
    
    /// 事件发送器
    event_tx: tokio::sync::mpsc::UnboundedSender<SyncEvent>,
}

impl WebSocketSyncClient {
    pub fn new(
        server_url: String,
        api_key: String,
        device_id: String,
    ) -> Self {
        let ws_url = server_url.replace("http", "ws");
        let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();
        
        // 启动事件处理任务
        tokio::spawn(async move {
            while let Some(event) = event_rx.recv().await {
                // 处理同步事件
                Self::handle_event(event).await;
            }
        });
        
        Self {
            ws_url,
            api_key,
            device_id,
            event_tx,
        }
    }
    
    /// 连接到同步服务器
    pub async fn connect(&self) -> Result<(), SyncError> {
        let url = format!(
            "{}/sync/ws?device_id={}&token={}",
            self.ws_url, self.device_id, self.api_key
        );
        
        let (ws_stream, _) = connect_async(&url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        // 处理接收的消息
        let device_id = self.device_id.clone();
        tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(sync_msg) = serde_json::from_str::<SyncMessage>(&text) {
                            Self::handle_sync_message(sync_msg, &device_id).await;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        break;
                    }
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });
        
        Ok(())
    }
    
    /// 发送变更通知
    pub fn notify_change(&self, change: SyncChange) {
        let msg = SyncMessage {
            r#type: MessageType::Change,
            device_id: self.device_id.clone(),
            payload: serde_json::to_value(&change).unwrap(),
        };
        
        if let Ok(text) = serde_json::to_string(&msg) {
            self.event_tx.send(SyncEvent::Send(text)).ok();
        }
    }
    
    /// 处理同步消息
    async fn handle_sync_message(msg: SyncMessage, device_id: &str) {
        // 忽略自己发送的消息
        if msg.device_id == device_id {
            return;
        }
        
        match msg.r#type {
            MessageType::Change => {
                if let Ok(change) = serde_json::from_value::<SyncChange>(msg.payload) {
                    // 通知应用有远程变更
                }
            }
            MessageType::Presence => {
                // 处理在线状态更新
            }
            MessageType::Conflict => {
                // 处理冲突通知
            }
        }
    }
    
    async fn handle_event(event: SyncEvent) {
        match event {
            SyncEvent::Send(text) => {
                // 发送到 WebSocket
            }
            SyncEvent::Reconnect => {
                // 重连逻辑
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SyncMessage {
    #[serde(rename = "type")]
    r#type: MessageType,
    device_id: String,
    payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageType {
    Change,
    Presence,
    Conflict,
}

pub enum SyncEvent {
    Send(String),
    Reconnect,
}
```

#### 9.2.5 协作功能

```rust
// src/services/sync/collaboration.rs

/// 协作服务
pub struct CollaborationService {
    /// 当前用户
    current_user: Option<User>,
    
    /// 团队成员
    team_members: Vec<TeamMember>,
    
    /// 共享的集合
    shared_collections: Vec<SharedCollection>,
    
    /// 实时协作会话
    active_sessions: HashMap<String, CollaborationSession>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub user: User,
    pub role: TeamRole,
    pub status: MemberStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamRole {
    Owner,
    Admin,
    Editor,
    Viewer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemberStatus {
    Online,
    Away,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedCollection {
    pub collection_id: String,
    pub name: String,
    pub owner: User,
    pub permissions: Vec<CollectionPermission>,
    pub members: Vec<TeamMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionPermission {
    pub user_id: String,
    pub role: TeamRole,
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_share: bool,
}

#[derive(Debug, Clone)]
pub struct CollaborationSession {
    pub session_id: String,
    pub collection_id: String,
    pub participants: Vec<Participant>,
    pub cursors: HashMap<String, CursorPosition>,
    pub operations: Vec<Operation>,
}

#[derive(Debug, Clone)]
pub struct Participant {
    pub user: User,
    pub color: rgb::RGB<u8>,
    pub joined_at: i64,
}

#[derive(Debug, Clone)]
pub struct CursorPosition {
    pub user_id: String,
    pub item_type: String,
    pub item_id: String,
    pub position: Option<(usize, usize)>, // line, column
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Operation {
    #[serde(rename = "insert")]
    Insert { path: Vec<String>, value: serde_json::Value },
    #[serde(rename = "update")]
    Update { path: Vec<String>, value: serde_json::Value },
    #[serde(rename = "delete")]
    Delete { path: Vec<String> },
}

impl CollaborationService {
    /// 分享集合给团队成员
    pub async fn share_collection(
        &self,
        collection_id: &str,
        user_emails: &[String],
        role: TeamRole,
    ) -> Result<Vec<TeamMember>, SyncError> {
        // 调用远程 API 分享集合
        Ok(vec![])
    }
    
    /// 获取共享的集合列表
    pub async fn get_shared_collections(&self) -> Result<Vec<SharedCollection>, SyncError> {
        // 从远程获取共享集合
        Ok(vec![])
    }
    
    /// 加入协作会话
    pub async fn join_session(
        &mut self,
        collection_id: &str,
    ) -> Result<String, SyncError> {
        let session_id = uuid::Uuid::new_v4().to_string();
        
        // 通过 WebSocket 加入协作房间
        Ok(session_id)
    }
    
    /// 离开协作会话
    pub async fn leave_session(&mut self, session_id: &str) -> Result<(), SyncError> {
        self.active_sessions.remove(session_id);
        Ok(())
    }
    
    /// 广播操作到其他参与者
    pub async fn broadcast_operation(
        &self,
        session_id: &str,
        operation: Operation,
    ) -> Result<(), SyncError> {
        if let Some(session) = self.active_sessions.get(session_id) {
            // 通过 WebSocket 发送操作
        }
        Ok(())
    }
    
    /// 处理远程操作
    pub async fn handle_remote_operation(
        &mut self,
        session_id: &str,
        operation: Operation,
        from_user: &User,
    ) -> Result<(), SyncError> {
        // 应用操作转换（OT）或 CRDT 算法
        Ok(())
    }
    
    /// 更新光标位置
    pub async fn update_cursor(
        &self,
        session_id: &str,
        cursor: CursorPosition,
    ) -> Result<(), SyncError> {
        // 广播光标位置
        Ok(())
    }
}
```

#### 9.2.6 本地存储扩展（支持同步）

```rust
// src/services/storage/sync_ext.rs

impl StorageService {
    /// 获取待同步的变更
    pub async fn get_pending_changes(&self) -> Result<Vec<SyncChange>, StorageError> {
        let rows = sqlx::query_as!(
            SyncChangeRow,
            r#"
            SELECT * FROM sync_changes
            WHERE synced = 0
            ORDER BY timestamp ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        rows.into_iter()
            .map(|row| self.row_to_change(row))
            .collect()
    }
    
    /// 应用远程变更
    pub async fn apply_remote_changes(
        &self,
        changes: Vec<SyncChange>,
    ) -> Result<(), StorageError> {
        let mut tx = self.pool.begin().await?;
        
        for change in changes {
            match change.operation {
                SyncOperation::Create => {
                    self.apply_create(&mut tx, &change).await?;
                }
                SyncOperation::Update => {
                    self.apply_update(&mut tx, &change).await?;
                }
                SyncOperation::Delete => {
                    self.apply_delete(&mut tx, &change).await?;
                }
            }
        }
        
        tx.commit().await?;
        Ok(())
    }
    
    /// 记录本地变更
    pub async fn record_change(
        &self,
        item_type: &str,
        item_id: &str,
        operation: SyncOperation,
        data: &serde_json::Value,
    ) -> Result<(), StorageError> {
        sqlx::query!(
            r#"
            INSERT INTO sync_changes (id, item_type, item_id, operation, data, timestamp, synced)
            VALUES (?, ?, ?, ?, ?, ?, 0)
            "#,
            Uuid::new_v4().to_string(),
            item_type,
            item_id,
            operation.to_string(),
            data,
            chrono::Utc::now().timestamp_millis(),
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// 标记变更已同步
    pub async fn mark_changes_synced(&self, change_ids: &[String]) -> Result<(), StorageError> {
        for id in change_ids {
            sqlx::query!(
                "UPDATE sync_changes SET synced = 1 WHERE id = ?",
                id
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }
}
```

#### 9.2.7 同步配置 UI

```rust
// src/ui/settings/sync_settings.rs

use gpui::*;

pub struct SyncSettingsView {
    /// 当前同步模式
    mode: SyncMode,
    
    /// 服务器 URL
    server_url: String,
    
    /// API 密钥
    api_key: String,
    
    /// 同步状态
    status: SyncStatus,
    
    /// 是否正在连接
    connecting: bool,
}

impl SyncSettingsView {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            mode: SyncMode::Offline,
            server_url: String::new(),
            api_key: String::new(),
            status: SyncStatus::Idle,
            connecting: false,
        }
    }
    
    fn test_connection(&mut self, cx: &mut ViewContext<Self>) {
        let server_url = self.server_url.clone();
        let api_key = self.api_key.clone();
        
        self.connecting = true;
        cx.notify();
        
        cx.spawn(|this, mut cx| async move {
            let result = test_server_connection(&server_url, &api_key).await;
            
            this.update(&mut cx, |this, cx| {
                this.connecting = false;
                match result {
                    Ok(_) => {
                        cx.notify("Connection successful!");
                    }
                    Err(e) => {
                        cx.notify(&format!("Connection failed: {}", e));
                    }
                }
                cx.notify();
            }).ok();
        }).detach();
    }
    
    fn save_settings(&mut self, cx: &mut ViewContext<Self>) {
        let sync_service = cx.global::<AppState>().sync.clone();
        
        cx.spawn(|this, mut cx| async move {
            let result = sync_service.configure_online(
                this.server_url.clone(),
                this.api_key.clone(),
                this.mode,
            ).await;
            
            this.update(&mut cx, |this, cx| {
                match result {
                    Ok(_) => {
                        cx.notify("Sync configured successfully!");
                    }
                    Err(e) => {
                        cx.notify(&format!("Configuration failed: {}", e));
                    }
                }
                cx.notify();
            }).ok();
        }).detach();
    }
}

impl Render for SyncSettingsView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p_4()
            .gap_4()
            .child(
                div()
                    .text_xl()
                    .font_semibold()
                    .child("Sync Settings")
            )
            .child(self.render_mode_selector(cx))
            .child(self.render_server_config(cx))
            .child(self.render_status(cx))
            .child(self.render_actions(cx))
    }
}
```

---

#### 9.2.8 服务器端 API 设计

```typescript
// 同步服务器 API 规范 (TypeScript 接口定义)

interface SyncServerConfig {
  // 数据库配置
  database: {
    host: string;
    port: number;
    name: string;
    user: string;
    password: string;
  };
  
  // Redis 配置（用于缓存和实时同步）
  redis: {
    host: string;
    port: number;
    password?: string;
  };
  
  // JWT 配置
  jwt: {
    secret: string;
    expiration: string;
  };
  
  // WebSocket 配置
  websocket: {
    port: number;
    path: string;
    pingInterval: number;
    pingTimeout: number;
  };
}

// API 端点定义
interface SyncServerAPI {
  // 认证
  'POST /auth/register': {
    request: { email: string; password: string; name: string };
    response: { user_id: string; token: string };
  };
  
  'POST /auth/login': {
    request: { email: string; password: string };
    response: { user_id: string; token: string };
  };
  
  'POST /auth/refresh': {
    request: { refresh_token: string };
    response: { token: string };
  };
  
  // 同步
  'GET /sync/pull': {
    query: { device_id: string; since?: string };
    response: {
      timestamp: number;
      changes: SyncChange[];
    };
  };
  
  'POST /sync/push': {
    request: {
      device_id: string;
      changes: SyncChange[];
    };
    response: {
      sync_id: string;
      accepted_count: number;
      rejected: Array<{ index: number; reason: string }>;
    };
  };
  
  'POST /sync/acknowledge': {
    request: { sync_id: string };
    response: { success: boolean };
  };
  
  // 集合共享
  'POST /collections/:id/share': {
    request: {
      emails: string[];
      role: 'viewer' | 'editor' | 'admin';
    };
    response: { invited: Array<{ email: string; token: string }> };
  };
  
  'GET /collections/shared': {
    response: SharedCollection[];
  };
  
  'PUT /collections/:id/permissions/:user_id': {
    request: { role: 'viewer' | 'editor' | 'admin' };
    response: { success: boolean };
  };
  
  'DELETE /collections/:id/permissions/:user_id': {
    response: { success: boolean };
  };
  
  // 协作
  'WebSocket /sync/ws': {
    query: { device_id: string; token: string };
    messages: {
      // 客户端 -> 服务器
      subscribe: { collection_id: string };
      unsubscribe: { collection_id: string };
      operation: { collection_id: string; operation: Operation };
      cursor: { collection_id: string; position: CursorPosition };
      presence: { status: 'online' | 'away' | 'offline' };
      
      // 服务器 -> 客户端
      user_joined: { user_id: string; user: User; color: string };
      user_left: { user_id: string };
      operation_received: { user_id: string; operation: Operation };
      cursor_moved: { user_id: string; position: CursorPosition };
      conflict_detected: { conflicts: ConflictInfo[] };
    };
  };
  
  // 版本历史
  'GET /collections/:id/history': {
    query: { limit?: number; offset?: number };
    response: {
      versions: Array<{
        version_id: string;
        version: number;
        created_at: number;
        created_by: User;
        description?: string;
      }>;
    };
  };
  
  'POST /collections/:id/restore': {
    request: { version_id: string };
    response: { success: boolean };
  };
}
```

#### 9.2.9 冲突解决 UI

```rust
// src/ui/conflict/resolution_dialog.rs

use gpui::*;

pub struct ConflictResolutionDialog {
    /// 冲突列表
    conflicts: Vec<ConflictInfo>,
    
    /// 当前选中的冲突索引
    current_index: usize,
    
    /// 每个冲突的选择
    resolutions: HashMap<String, ConflictResolution>,
    
    /// 是否已全部解决
    all_resolved: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum ConflictResolution {
    Local,
    Remote,
    Merge,
}

impl ConflictResolutionDialog {
    pub fn new(conflicts: Vec<ConflictInfo>, cx: &mut ViewContext<Self>) -> Self {
        Self {
            conflicts,
            current_index: 0,
            resolutions: HashMap::new(),
            all_resolved: false,
        }
    }
    
    fn select_local(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(conflict) = self.conflicts.get(self.current_index) {
            self.resolutions.insert(
                format!("{}:{}", conflict.item_type, conflict.item_id),
                ConflictResolution::Local,
            );
            self.advance_or_close(cx);
        }
    }
    
    fn select_remote(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(conflict) = self.conflicts.get(self.current_index) {
            self.resolutions.insert(
                format!("{}:{}", conflict.item_type, conflict.item_id),
                ConflictResolution::Remote,
            );
            self.advance_or_close(cx);
        }
    }
    
    fn advance_or_close(&mut self, cx: &mut ViewContext<Self>) {
        if self.current_index + 1 < self.conflicts.len() {
            self.current_index += 1;
        } else {
            self.all_resolved = true;
            cx.emit(ConflictDialogEvent::Resolved(self.resolutions.clone()));
        }
        cx.notify();
    }
}

impl Render for ConflictResolutionDialog {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let conflict = self.conflicts.get(self.current_index);
        
        div()
            .flex()
            .flex_col()
            .w(px(600.0))
            .h(px(500.0))
            .bg(rgb(0x252526))
            .rounded_lg()
            .p_6()
            .child(
                div()
                    .text_lg()
                    .font_semibold()
                    .child(format!(
                        "Resolve Conflict ({}/{})",
                        self.current_index + 1,
                        self.conflicts.len()
                    ))
            )
            .when_some(conflict, |div, conflict| {
                div.child(self.render_conflict(conflict, cx))
            })
    }
}
```

#### 9.2.10 离线优先数据同步策略

```rust
// src/services/sync/offline_first.rs

/// 离线优先同步策略
/// 
/// 核心原则：
/// 1. 本地写入总是成功，立即保存到本地数据库
/// 2. 在线时自动同步变更到服务器
/// 3. 离线时排队变更，待上线后批量同步
/// 4. 使用操作转换（OT）或 CRDT 处理并发编辑

pub struct OfflineFirstSyncStrategy {
    /// 本地变更队列
    pending_queue: Arc<RwLock<VecDeque<SyncChange>>>,
    
    /// 最大队列大小
    max_queue_size: usize,
    
    /// 同步间隔
    sync_interval: Duration,
    
    /// 是否正在同步
    syncing: Arc<AtomicBool>,
}

impl OfflineFirstSyncStrategy {
    pub fn new() -> Self {
        Self {
            pending_queue: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            max_queue_size: 10000,
            sync_interval: Duration::from_secs(30),
            syncing: Arc::new(AtomicBool::new(false)),
        }
    }
    
    /// 添加本地变更到队列
    pub async fn enqueue_change(&self, change: SyncChange) -> Result<(), SyncError> {
        let mut queue = self.pending_queue.write().await;
        
        // 检查队列大小
        if queue.len() >= self.max_queue_size {
            return Err(SyncError::QueueFull);
        }
        
        // 去重：检查是否已有相同 ID 的变更
        if let Some(pos) = queue.iter().position(|c| {
            c.item_id == change.item_id && c.item_type == change.item_type
        }) {
            // 更新现有变更
            queue.remove(pos);
        }
        
        queue.push_back(change);
        Ok(())
    }
    
    /// 启动后台同步任务
    pub fn start_background_sync(
        &self,
        sync_service: Arc<SyncService>,
    ) -> tokio::task::JoinHandle<()> {
        let queue = self.pending_queue.clone();
        let syncing = self.syncing.clone();
        let interval = self.sync_interval;
        
        tokio::spawn(async move {
            let mut timer = tokio::time::interval(interval);
            
            loop {
                timer.tick().await;
                
                // 检查是否正在同步
                if syncing.load(Ordering::Relaxed) {
                    continue;
                }
                
                // 获取待同步的变更
                let changes: Vec<_> = {
                    let mut q = queue.write().await;
                    let batch_size = q.len().min(100); // 每次最多同步 100 个
                    q.drain(..batch_size).collect()
                };
                
                if changes.is_empty() {
                    continue;
                }
                
                syncing.store(true, Ordering::Relaxed);
                
                // 执行同步
                match sync_service.sync_changes(changes).await {
                    Ok(_) => {
                        tracing::debug!("Background sync completed successfully");
                    }
                    Err(e) => {
                        tracing::error!("Background sync failed: {}", e);
                        // 失败的变更重新加入队列
                        let mut q = queue.write().await;
                        for change in changes {
                            q.push_front(change);
                        }
                    }
                }
                
                syncing.store(false, Ordering::Relaxed);
            }
        })
    }
}

/// CRDT (Conflict-free Replicated Data Types) 实现
/// 用于处理协作编辑中的冲突

pub trait Crdt<T> {
    /// 合并两个副本
    fn merge(&mut self, other: T) -> Result<(), CrdtError>;
    
    /// 生成新的变更操作
    fn generate_operation(&self, local_change: &T) -> Result<Operation, CrdtError>;
    
    /// 应用远程操作
    fn apply_operation(&mut self, operation: &Operation) -> Result<(), CrdtError>;
}

/// LWW-Register (Last-Write-Wins Register)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LwwRegister<T> {
    pub value: T,
    pub timestamp: i64,
    pub node_id: String,
}

impl<T: Clone + PartialEq> Crdt<LwwRegister<T>> for LwwRegister<T> {
    fn merge(&mut self, other: LwwRegister<T>) -> Result<(), CrdtError> {
        if other.timestamp > self.timestamp 
            || (other.timestamp == self.timestamp && other.node_id > self.node_id)
        {
            self.value = other.value;
            self.timestamp = other.timestamp;
            self.node_id = other.node_id;
        }
        Ok(())
    }
    
    fn generate_operation(&self, _local_change: &LwwRegister<T>) -> Result<Operation, CrdtError> {
        Ok(Operation::Update {
            path: vec![],
            value: serde_json::to_value(self)?,
        })
    }
    
    fn apply_operation(&mut self, operation: &Operation) -> Result<(), CrdtError> {
        match operation {
            Operation::Update { value, .. } => {
                let other: LwwRegister<T> = serde_json::from_value(value.clone())?;
                self.merge(other)?;
            }
            _ => {}
        }
        Ok(())
    }
}

/// OR-Set (Observed-Remove Set)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrSet<T> {
    pub elements: HashMap<T, HashSet<String>>, // value -> set of unique tags
    pub tombstones: HashSet<String>,
}

impl<T: Clone + Hash + Eq> Crdt<OrSet<T>> for OrSet<T> {
    fn merge(&mut self, other: OrSet<T>) -> Result<(), CrdtError> {
        // 合并元素
        for (value, tags) in other.elements {
            let entry = self.elements.entry(value).or_insert_with(HashSet::new);
            entry.extend(tags);
        }
        
        // 合并墓碑
        self.tombstones.extend(other.tombstones);
        
        // 清理已删除的元素
        self.elements.retain(|_, tags| {
            !tags.iter().all(|tag| self.tombstones.contains(tag))
        });
        
        Ok(())
    }
    
    fn generate_operation(&self, local_change: &OrSet<T>) -> Result<Operation, CrdtError> {
        // 实现变更生成逻辑
        Ok(Operation::Insert {
            path: vec![],
            value: serde_json::to_value(local_change)?,
        })
    }
    
    fn apply_operation(&mut self, operation: &Operation) -> Result<(), CrdtError> {
        match operation {
            Operation::Insert { value, .. } => {
                let other: OrSet<T> = serde_json::from_value(value.clone())?;
                self.merge(other)?;
            }
            Operation::Delete { path } => {
                // 根据路径删除元素
                if let Some(tag) = path.last() {
                    self.tombstones.insert(tag.clone());
                }
            }
            _ => {}
        }
        Ok(())
    }
}
```

---

## 10. 安全性考虑

1. **敏感数据加密**: 环境变量中的密钥使用 AES 加密存储
2. **脚本沙箱**: JavaScript 执行在受限环境中
3. **请求限制**: 防止意外的大量请求
4. **HTTPS 优先**: 默认验证 SSL 证书
5. **本地数据**: 所有数据默认存储在本地

---

## 11. 性能优化

1. **异步 I/O**: 使用 Tokio 处理所有网络和数据库操作
2. **连接池**: HTTP 客户端和数据库连接池
3. **懒加载**: 大型集合按需加载
4. **缓存**: 响应缓存和环境变量缓存
5. **增量渲染**: GPUI 的 GPU 加速渲染

---

## 12. 开发计划

### Phase 1: 基础框架 (2 周)
- [x] 项目初始化
- [ ] GPUI 基础布局
- [ ] SQLite 存储层
- [ ] 基本 HTTP 请求功能

### Phase 2: 核心功能 (3 周)
- [ ] 请求面板完整实现
- [ ] 响应面板完整实现
- [ ] Collection 树形视图
- [ ] 环境变量管理

### Phase 3: 脚本系统 (2 周)
- [ ] JavaScript 引擎集成
- [ ] Pre-request Script
- [ ] Post-response Script
- [ ] 测试断言

### Phase 4: MCP 集成 (1 周)
- [ ] MCP Server 实现
- [ ] 工具和资源暴露
- [ ] stdio 传输层

### Phase 5: 完善与优化 (2 周)
- [ ] 导入/导出功能
- [ ] 云同步支持
- [ ] 性能优化
- [ ] 文档和测试
```

---

## 13. 深化设计：关键实现细节

### 13.1 GPUI 应用生命周期

```rust
// src/main.rs

use gpui::*;
use std::sync::Arc;

fn main() {
    // 初始化 GPUI 应用
    App::new().run(move |cx: &mut AppContext| {
        // 初始化全局服务
        let storage_service = Arc::new(StorageService::new("postboy.db")
            .await.expect("Failed to initialize storage"));
        
        let http_service = Arc::new(HttpService::new()
            .expect("Failed to initialize HTTP client"));
        
        let script_engine = Arc::new(ScriptEngine::new());
        
        let mcp_server = Arc::new(McpServer::new(
            http_service.clone(),
            storage_service.clone(),
        ));
        
        // 启动 MCP Server（后台线程）
        let mcp_server_clone = mcp_server.clone();
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    if let Err(e) = mcp_server_clone.run_stdio().await {
                        eprintln!("MCP Server error: {:?}", e);
                    }
                });
        });
        
        // 创建应用状态
        let app_state = AppState::new(
            storage_service,
            http_service,
            script_engine,
            mcp_server,
        );
        cx.set_global(app_state);
        
        // 打开主窗口
        cx.open_window(WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: Point { x: px(100.0), y: px(100.0) },
                size: Size { width: px(1400.0), height: px(900.0) },
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Postboy".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            ..Default::default()
        }, |cx| MainWindow::new(cx));
    });
}

// 全局应用状态
pub struct AppState {
    pub storage: Arc<StorageService>,
    pub http: Arc<HttpService>,
    pub script_engine: Arc<ScriptEngine>,
    pub mcp: Arc<McpServer>,
    pub event_bus: Arc<EventBus>,
}

impl AppState {
    pub fn new(
        storage: Arc<StorageService>,
        http: Arc<HttpService>,
        script_engine: Arc<ScriptEngine>,
        mcp: Arc<McpServer>,
    ) -> Self {
        Self {
            storage,
            http,
            script_engine,
            mcp,
            event_bus: Arc::new(EventBus::new()),
        }
    }
}
```

### 13.2 脚本引擎详细实现

```rust
// src/script_engine/mod.rs

use boa_engine::{Context, Source, object::ObjectData, value::Value};
use boa_gc::{Gc, GcCell};
use std::collections::HashMap;

pub struct ScriptEngine {
    // 可选的预编译上下文池
    context_pool: Vec<Context>,
}

impl ScriptEngine {
    pub fn new() -> Self {
        Self {
            context_pool: Vec::new(),
        }
    }
    
    /// 获取或创建上下文
    fn get_context(&mut self) -> Context {
        self.context_pool.pop()
            .unwrap_or_else(|| Self::create_context())
    }
    
    /// 归还上下文到池中
    fn return_context(&mut self, ctx: Context) {
        if self.context_pool.len() < 10 {
            self.context_pool.push(ctx);
        }
    }
    
    /// 创建带内置对象的上下文
    fn create_context() -> Context {
        let ctx = Context::default();
        
        // 注册 console 对象
        register_console(&ctx);
        
        // 注册 crypto 对象（用于签名）
        register_crypto(&ctx);
        
        ctx
    }
    
    /// 执行 Pre-request Script
    pub async fn execute_pre_request(
        &self,
        script: &str,
        context: &mut ScriptContext,
    ) -> Result<ScriptResult, ScriptError> {
        let mut ctx = self.get_context();
        
        // 注入 pm 对象
        inject_pm_object(&mut ctx, context, false)?;
        
        // 执行脚本
        let source = Source::from_bytes(script);
        ctx.eval(source)
            .map_err(|e| ScriptError::Execution(e.to_string()))?;
        
        // 提取修改后的值
        let modified_request = extract_request_modifications(&ctx)?;
        let modified_variables = extract_variable_changes(&ctx)?;
        
        // 归还上下文
        self.return_context(ctx);
        
        Ok(ScriptResult {
            success: true,
            error: None,
            modified_variables,
            modified_request: Some(modified_request),
            test_results: vec![],
        })
    }
    
    /// 执行 Post-response Script
    pub async fn execute_post_response(
        &self,
        script: &str,
        context: &mut ScriptContext,
        response: &HttpResponse,
    ) -> Result<ScriptResult, ScriptError> {
        let mut ctx = self.get_context();
        
        // 注入 pm 对象（包含响应）
        inject_pm_object(&mut ctx, context, true, Some(response))?;
        
        // 执行脚本
        let source = Source::from_bytes(script);
        ctx.eval(source)
            .map_err(|e| ScriptError::Execution(e.to_string()))?;
        
        // 提取测试结果
        let test_results = extract_test_results(&ctx)?;
        let modified_variables = extract_variable_changes(&ctx)?;
        
        // 归还上下文
        self.return_context(ctx);
        
        Ok(ScriptResult {
            success: true,
            error: None,
            modified_variables,
            modified_request: None,
            test_results,
        })
    }
}

/// 注入 pm 对象到 JavaScript 环境
fn inject_pm_object(
    ctx: &mut Context,
    script_ctx: &ScriptContext,
    include_response: bool,
    response: Option<&HttpResponse>,
) -> Result<(), ScriptError> {
    // 创建 pm 对象
    let pm = ObjectData::default();
    let pm_value = Gc::new(GcCell::new(pm));
    
    // pm.environment
    let env_obj = create_environment_object(ctx, script_ctx);
    pm_value.set("environment", env_obj, true, ctx)?;
    
    // pm.globals
    let globals_obj = create_globals_object(ctx, script_ctx);
    pm_value.set("globals", globals_obj, true, ctx)?;
    
    // pm.variables
    let vars_obj = create_variables_object(ctx, script_ctx);
    pm_value.set("variables", vars_obj, true, ctx)?;
    
    // pm.request
    let req_obj = create_request_object(ctx, script_ctx);
    pm_value.set("request", req_obj, true, ctx)?;
    
    // pm.response（仅 post-response）
    if include_response {
        if let Some(resp) = response {
            let resp_obj = create_response_object(ctx, resp)?;
            pm_value.set("response", resp_obj, true, ctx)?;
            
            // 注入断言辅助函数
            inject_test_helpers(ctx, pm_value.clone())?;
        }
    }
    
    // pm.sendRequest
    let send_request_fn = create_send_request_function(ctx);
    pm_value.set("sendRequest", send_request_fn, true, ctx)?;
    
    // 注册到全局
    ctx.register_global_property("pm", pm_value, true)?;
    
    Ok(())
}

/// 创建环境变量对象
fn create_environment_object(
    ctx: &Context,
    script_ctx: &ScriptContext,
) -> Value {
    let env = ObjectData::default();
    let env_value = Gc::new(GcCell::new(env));
    
    // 从 context 中克隆变量
    let variables = script_ctx.environment.clone();
    
    // set 方法
    let set_fn = Function::native(
        ctx,
        "set",
        2,
        |_, args, context| {
            let key = args.get(0).and_then(|v| v.as_string()).unwrap_or("");
            let value = args.get(1).and_then(|v| v.as_string()).unwrap_or("");
            // 存储到 context 的修改记录中
            // context.modified_variables.push((key.to_string(), value.to_string()));
            Ok(Value::undefined())
        },
    );
    
    // get 方法
    let get_fn = Function::native(
        ctx,
        "get",
        1,
        move |_, args, _| {
            let key = args.get(0).and_then(|v| v.as_string()).unwrap_or("");
            if let Some(value) = variables.get(key) {
                Ok(Value::from(value.as_str()))
            } else {
                Ok(Value::undefined())
            }
        },
    );
    
    env_value.set("set", set_fn, true, ctx)?;
    env_value.set("get", get_fn, true, ctx)?;
    
    env_value.into()
}

/// 注入测试辅助函数
fn inject_test_helpers(
    ctx: &mut Context,
    pm_obj: Gc<GcCell<ObjectData>>,
) -> Result<(), ScriptError> {
    // pm.test
    let test_fn = Function::native(
        ctx,
        "test",
        2,
        |_, args, context| {
            let name = args.get(0).and_then(|v| v.as_string()).unwrap_or("");
            let fn_arg = args.get(1);
            
            // 执行测试函数
            if let Some(Function) = fn_arg {
                // 调用测试函数
                // 记录测试结果
            }
            
            Ok(Value::undefined())
        },
    );
    
    pm_obj.set("test", test_fn, true, ctx)?;
    
    // pm.expect (Chai-like 断言)
    let expect_fn = Function::native(
        ctx,
        "expect",
        1,
        |_, args, context| {
            let actual = args.get(0).cloned().unwrap_or(Value::undefined());
            create_expectation(actual, context)
        },
    );
    
    pm_obj.set("expect", expect_fn, true, ctx)?;
    
    Ok(())
}

/// 创建 expectation 链式调用对象
fn create_expectation(actual: Value, ctx: &Context) -> Result<Value, ScriptError> {
    let expect_obj = ObjectData::default();
    let expect_value = Gc::new(GcCell::new(expect_obj));
    
    // .to.equal()
    let to_equal_fn = Function::native(
        ctx,
        "equal",
        1,
        move |_, args, _| {
            let expected = args.get(0).cloned().unwrap_or(Value::undefined());
            let passed = actual.equals(&expected);
            if !passed {
                // 记录失败
            }
            Ok(Value::undefined())
        },
    );
    
    // .to.have.property()
    let have_property_fn = Function::native(
        ctx,
        "property",
        1,
        move |_, args, _| {
            let prop_name = args.get(0).and_then(|v| v.as_string()).unwrap_or("");
            let has_property = actual.as_object()
                .map(|obj| obj.has(prop_name, ctx))
                .unwrap_or(false);
            if !has_property {
                // 记录失败
            }
            Ok(Value::undefined())
        },
    );
    
    // 构建链式结构: to -> have -> (equal, property, etc)
    let to_obj = ObjectData::default();
    let to_value = Gc::new(GcCell::new(to_obj));
    to_value.set("equal", to_equal_fn, true, ctx)?;
    
    let have_obj = ObjectData::default();
    let have_value = Gc::new(GcCell::new(have_obj));
    have_value.set("property", have_property_fn, true, ctx)?;
    
    to_value.set("have", have_value, true, ctx)?;
    expect_value.set("to", to_value, true, ctx)?;
    
    Ok(expect_value.into())
}

/// 注册 console 对象
fn register_console(ctx: &Context) {
    let console = ObjectData::default();
    let console_value = Gc::new(GcCell::new(console));
    
    let log_fn = Function::native(
        ctx,
        "log",
        0, // 可变参数
        |_, args, _| {
            let output: String = args.iter()
                .map(|v| format!("{:?}", v))
                .collect::<Vec<_>>()
                .join(" ");
            println!("[Postboy Script] {}", output);
            Ok(Value::undefined())
        },
    );
    
    console_value.set("log", log_fn, true, ctx)?;
    // 同样添加 error, warn...
    
    ctx.register_global_property("console", console_value, true)?;
}

/// 注册 crypto 对象（用于签名）
fn register_crypto(ctx: &Context) {
    let crypto = ObjectData::default();
    let crypto_value = Gc::new(GcCell::new(crypto));
    
    // 简化的 HMAC 实现
    let create_hmac_fn = Function::native(
        ctx,
        "createHmac",
        2,
        |_, args, _| {
            let algorithm = args.get(0).and_then(|v| v.as_string()).unwrap_or("sha256");
            let key = args.get(1).and_then(|v| v.as_string()).unwrap_or("");
            
            // 返回 Hmac 对象
            let hmac_obj = create_hmac_object(algorithm, key);
            Ok(hmac_obj)
        },
    );
    
    crypto_value.set("createHmac", create_hmac_fn, true, ctx)?;
    ctx.register_global_property("crypto", crypto_value, true)?;
}
```

### 13.3 GPUI 组件状态管理

```rust
// src/ui/request/request_panel.rs

use gpui::*;
use std::sync::Arc;

pub struct RequestPanel {
    /// 当前选中的请求
    active_request: Option<Request>,
    
    /// 当前请求的状态
    request_state: RequestState,
    
    /// 当前选中的 Tab
    active_tab: RequestTab,
    
    /// 是否正在加载
    loading: bool,
    
    /// 订阅的事件接收器
    _event_subscription: Option<tokio::sync::broadcast::Receiver<AppEvent>>,
}

#[derive(Clone, Copy)]
enum RequestTab {
    Params,
    Headers,
    Body,
    Auth,
    Script,
}

pub struct RequestState {
    method: HttpMethod,
    url: String,
    headers: Vec<HeaderEntry>,
    params: Vec<ParamEntry>,
    body: RequestBodyState,
}

#[derive(Clone)]
struct HeaderEntry {
    enabled: bool,
    key: String,
    value: String,
}

#[derive(Clone)]
struct ParamEntry {
    enabled: bool,
    key: String,
    value: String,
}

pub enum RequestBodyState {
    None,
    Json(String),
    FormData(Vec<FormFieldEntry>),
    UrlEncoded(Vec<FormFieldEntry>),
    Raw(String),
    Binary,
}

impl RequestPanel {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        // 订阅事件
        let mut event_rx = cx.global::<AppState>()
            .event_bus
            .subscribe();
        
        // 初始状态
        Self {
            active_request: None,
            request_state: RequestState::default(),
            active_tab: RequestTab::Params,
            loading: false,
            _event_subscription: Some(event_rx),
        }
    }
    
    /// 加载请求到面板
    pub fn load_request(
        &mut self,
        request: Request,
        cx: &mut ViewContext<Self>,
    ) {
        self.active_request = Some(request.clone());
        self.request_state = RequestState::from(request);
        cx.notify();
    }
    
    /// 发送请求
    fn send_request(&mut self, cx: &mut ViewContext<Self>) {
        let state = self.request_state.clone();
        let http_service = cx.global::<AppState>().http.clone();
        let script_engine = cx.global::<AppState>().script_engine.clone();
        
        self.loading = true;
        cx.notify();
        
        cx.spawn(|this, mut cx| async move {
            // 1. 构建 HTTP 请求
            let http_request = build_http_request(&state);
            
            // 2. 执行 Pre-request Script
            // ...
            
            // 3. 发送请求
            let response = http_service.send_request(&http_request).await;
            
            // 4. 执行 Post-response Script
            // ...
            
            // 5. 更新 UI
            this.update(&mut cx, |this, cx| {
                this.loading = false;
                
                match response {
                    Ok(resp) => {
                        // 发送响应到响应面板
                        cx.emit(RequestPanelEvent::RequestComplete(resp));
                    }
                    Err(e) => {
                        cx.emit(RequestPanelEvent::RequestFailed(e.to_string()));
                    }
                }
                
                cx.notify();
            }).ok();
        }).detach();
    }
}

impl Render for RequestPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .child(self.render_request_bar(cx))
            .child(self.render_tab_bar(cx))
            .child(self.render_tab_content(cx))
    }
}

impl RequestPanel {
    fn render_request_bar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .p_2()
            .gap_2()
            .bg(rgb(0x252526))
            .child(self.render_method_selector(cx))
            .child(self.render_url_input(cx))
            .child(self.render_send_button(cx))
    }
    
    fn render_method_selector(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // 方法下拉选择器
        div()
            .px_3()
            .py_2()
            .bg(rgb(0x3c3c3c))
            .rounded_md()
            .child(self.request_state.method.to_string())
    }
    
    fn render_url_input(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .px_3()
            .py_2()
            .bg(rgb(0x3c3c3c))
            .rounded_md()
            .child(self.request_state.url.clone())
    }
    
    fn render_send_button(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_loading = self.loading;
        
        div()
            .px_4()
            .py_2()
            .bg(rgb(0x007acc))
            .rounded_md()
            .cursor_pointer()
            .when_some(is_loading.then(|| true), |div, _| {
                div.opacity(0.5)
            })
            .on_click(cx.listener(|this, _, cx| {
                if !this.loading {
                    this.send_request(cx);
                }
            }))
            .child(if is_loading { "..." } else { "Send" })
    }
    
    fn render_tab_bar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .border_b_1()
            .border_color(rgb(0x404040))
            .child(self.render_tab("Params", RequestTab::Params, cx))
            .child(self.render_tab("Headers", RequestTab::Headers, cx))
            .child(self.render_tab("Body", RequestTab::Body, cx))
            .child(self.render_tab("Auth", RequestTab::Auth, cx))
            .child(self.render_tab("Script", RequestTab::Script, cx))
    }
    
    fn render_tab(&self, label: &str, tab: RequestTab, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_active = self.active_tab == tab;
        
        div()
            .px_4()
            .py_2()
            .cursor_pointer()
            .when(is_active, |div| {
                div.border_b_2()
                    .border_color(rgb(0x007acc))
            })
            .on_click(cx.listener(move |this, _, cx| {
                this.active_tab = tab;
                cx.notify();
            }))
            .child(label.to_string())
    }
    
    fn render_tab_content(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .overflow_y_scroll()
            .match_target(self.active_tab, |tab| match tab {
                RequestTab::Params => self.render_params_tab(cx),
                RequestTab::Headers => self.render_headers_tab(cx),
                RequestTab::Body => self.render_body_tab(cx),
                RequestTab::Auth => self.render_auth_tab(cx),
                RequestTab::Script => self.render_script_tab(cx),
            })
    }
}
```

### 13.4 MCP Server 完整实现

```rust
// src/mcp/server.rs

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::RwLock;

pub struct McpServer {
    http_service: Arc<HttpService>,
    storage: Arc<StorageService>,
    tool_registry: ToolRegistry,
}

impl McpServer {
    pub fn new(
        http_service: Arc<HttpService>,
        storage: Arc<StorageService>,
    ) -> Self {
        let mut tool_registry = ToolRegistry::new();
        
        // 注册所有工具
        tool_registry.register(SendRequestTool::new(http_service.clone()));
        tool_registry.register(ListCollectionsTool::new(storage.clone()));
        tool_registry.register(GetRequestTool::new(storage.clone()));
        tool_registry.register(CreateRequestTool::new(storage.clone()));
        tool_registry.register(SetEnvironmentVariableTool::new(storage.clone()));
        tool_registry.register(RunCollectionTool::new(
            http_service.clone(),
            storage.clone(),
        ));
        
        Self {
            http_service,
            storage,
            tool_registry,
        }
    }
    
    /// 运行 stdio 传输
    pub async fn run_stdio(&self) -> Result<(), McpError> {
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        
        let mut reader = BufReader::new(stdin).lines();
        let mut writer = tokio::io::BufWriter::new(stdout);
        
        // 打开日志
        let mut log_file = if cfg!(debug_assertions) {
            Some(tokio::fs::File::create("mcp_debug.log").await?)
        } else {
            None
        };
        
        loop {
            // 读取一行 JSON
            let line = tokio::select! {
                result = reader.next_line() => {
                    match result {
                        Ok(Some(line)) => line,
                        Ok(None) => break, // EOF
                        Err(e) => return Err(McpError::Io(e)),
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    println!("MCP Server shutting down...");
                    break;
                }
            };
            
            // 调试日志
            if let Some(ref mut f) = log_file {
                f.write_all(format!("<-- {}\n", line).as_bytes()).await?;
                f.flush().await?;
            }
            
            // 解析 JSON-RPC 请求
            let request: JsonRpcRequest = match serde_json::from_str(&line) {
                Ok(req) => req,
                Err(e) => {
                    let error = JsonRpcResponse::error(
                        None,
                        JsonRpcError::new(-32700, "Parse error", None),
                    );
                    writer.write_all(error.to_json().as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                    writer.flush().await?;
                    continue;
                }
            };
            
            // 处理请求
            let response = self.handle_request(request).await;
            
            // 发送响应
            let response_json = response.to_json();
            
            if let Some(ref mut f) = log_file {
                f.write_all(format!("--> {}\n", response_json).as_bytes()).await?;
                f.flush().await?;
            }
            
            writer.write_all(response_json.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            writer.flush().await?;
        }
        
        Ok(())
    }
    
    async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request).await,
            "initialized" => {
                // 客户端已初始化的确认
                JsonRpcResponse::success(request.id, Value::Null)
            }
            "shutdown" => {
                // 优雅关闭
                JsonRpcResponse::success(request.id, Value::Null)
            }
            "tools/list" => self.handle_list_tools(request).await,
            "tools/call" => self.handle_tool_call(request).await,
            "resources/list" => self.handle_list_resources(request).await,
            "resources/read" => self.handle_read_resource(request).await,
            "prompts/list" => self.handle_list_prompts(request).await,
            _ => JsonRpcResponse::error(
                request.id,
                JsonRpcError::new(-32601, "Method not found", None),
            ),
        }
    }
    
    async fn handle_initialize(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        #[derive(Serialize)]
        struct ServerInfo {
            name: String,
            version: String,
        }
        
        #[derive(Serialize)]
        struct Capabilities {
            tools: Value,
            resources: Value,
            prompts: Value,
        }
        
        #[derive(Serialize)]
        struct InitializeResult {
            protocol_version: String,
            capabilities: Capabilities,
            server_info: ServerInfo,
        }
        
        let result = InitializeResult {
            protocol_version: "2024-11-05".to_string(),
            capabilities: Capabilities {
                tools: json!({}),
                resources: json!({
                    "subscribe": true,
                    "listChanged": true,
                }),
                prompts: json!({
                    "listChanged": true,
                }),
            },
            server_info: ServerInfo {
                name: "postboy".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };
        
        JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap())
    }
    
    async fn handle_list_tools(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let tools = self.tool_registry.list_tools();
        
        #[derive(Serialize)]
        struct ListToolsResult {
            tools: Vec<ToolDefinition>,
        }
        
        let result = ListToolsResult { tools };
        
        JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap())
    }
    
    async fn handle_tool_call(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let params = match request.params {
            Some(p) => p,
            None => {
                return JsonRpcResponse::error(
                    request.id,
                    JsonRpcError::new(-32602, "Invalid params", None),
                );
            }
        };
        
        let tool_name = match params.get("name").and_then(|v| v.as_str()) {
            Some(name) => name.to_string(),
            None => {
                return JsonRpcResponse::error(
                    request.id,
                    JsonRpcError::new(-32602, "Missing tool name", None),
                );
            }
        };
        
        let arguments = params.get("arguments").cloned().unwrap_or(Value::Null);
        
        match self.tool_registry.call(&tool_name, arguments).await {
            Ok(result) => {
                #[derive(Serialize)]
                struct ToolCallResult {
                    content: Vec<ContentBlock>,
                    is_error: bool,
                }
                
                #[derive(Serialize)]
                #[serde(tag = "type")]
                enum ContentBlock {
                    #[serde(rename = "text")]
                    Text { text: String },
                }
                
                let call_result = ToolCallResult {
                    content: vec![
                        ContentBlock::Text { text: result },
                    ],
                    is_error: false,
                };
                
                JsonRpcResponse::success(request.id, serde_json::to_value(call_result).unwrap())
            }
            Err(e) => {
                JsonRpcResponse::error(
                    request.id,
                    JsonRpcError::new(-32603, &format!("Tool error: {}", e), None),
                )
            }
        }
    }
    
    async fn handle_list_resources(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let resources = vec![
            json!({
                "uri": "postboy://collections",
                "name": "All Collections",
                "description": "Complete list of all API collections",
                "mimeType": "application/json",
            }),
            json!({
                "uri": "postboy://environments",
                "name": "Environments",
                "description": "List of all environments",
                "mimeType": "application/json",
            }),
        ];
        
        #[derive(Serialize)]
        struct ListResourcesResult {
            resources: Vec<Value>,
        }
        
        let result = ListResourcesResult { resources };
        
        JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap())
    }
    
    async fn handle_read_resource(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let params = match request.params {
            Some(p) => p,
            None => {
                return JsonRpcResponse::error(
                    request.id,
                    JsonRpcError::new(-32602, "Invalid params", None),
                );
            }
        };
        
        let uri = match params.get("uri").and_then(|v| v.as_str()) {
            Some(u) => u,
            None => {
                return JsonRpcResponse::error(
                    request.id,
                    JsonRpcError::new(-32602, "Missing uri", None),
                );
            }
        };
        
        match uri {
            "postboy://collections" => {
                let collections = self.storage.list_collections().await.unwrap();
                let content = serde_json::to_string_pretty(&collections).unwrap();
                
                #[derive(Serialize)]
                struct ResourceContents {
                    contents: Vec<ContentBlock>,
                }
                
                #[derive(Serialize)]
                #[serde(tag = "type")]
                enum ContentBlock {
                    #[serde(rename = "text")]
                    Text { uri: String, text: String },
                }
                
                let result = ResourceContents {
                    contents: vec![
                        ContentBlock::Text {
                            uri: uri.to_string(),
                            text: content,
                        },
                    ],
                };
                
                JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap())
            }
            _ => {
                JsonRpcResponse::error(
                    request.id,
                    JsonRpcError::new(-32602, "Unknown resource", None),
                )
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Value,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }
    
    fn error(id: Value, error: JsonRpcError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
    
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Clone, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl JsonRpcError {
    fn new(code: i32, message: &str, data: Option<Value>) -> Self {
        Self {
            code,
            message: message.to_string(),
            data,
        }
    }
}

// 工具注册表
struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }
    
    fn register(&mut self, tool: Box<dyn Tool>) {
        let def = tool.definition();
        self.tools.insert(def.name.clone(), tool);
    }
    
    fn list_tools(&self) -> Vec<ToolDefinition> {
        self.tools.values()
            .map(|t| t.definition())
            .collect()
    }
    
    async fn call(&self, name: &str, args: Value) -> Result<String, McpError> {
        match self.tools.get(name) {
            Some(tool) => tool.execute(args).await,
            None => Err(McpError::ToolNotFound(name.to_string())),
        }
    }
}

#[async_trait::async_trait]
trait Tool: Send + Sync {
    fn definition(&self) -> ToolDefinition;
    async fn execute(&self, args: Value) -> Result<String, McpError>;
}

#[derive(Debug, Clone, Serialize)]
struct ToolDefinition {
    name: String,
    description: String,
    input_schema: Value,
}

// 具体工具实现示例
struct SendRequestTool {
    http_service: Arc<HttpService>,
}

impl SendRequestTool {
    fn new(http_service: Arc<HttpService>) -> Self {
        Self { http_service }
    }
}

#[async_trait::async_trait]
impl Tool for SendRequestTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "send_request".to_string(),
            description: "Send an HTTP request and return the response".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "method": {
                        "type": "string",
                        "enum": ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"],
                    },
                    "url": { "type": "string" },
                    "headers": {
                        "type": "object",
                        "additionalProperties": { "type": "string" }
                    },
                    "body": { "type": "string" },
                },
                "required": ["method", "url"]
            }),
        }
    }
    
    async fn execute(&self, args: Value) -> Result<String, McpError> {
        let method = args.get("method")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::InvalidParams("Missing method".into()))?;
        
        let url = args.get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::InvalidParams("Missing url".into()))?;
        
        let mut request = HttpRequest {
            method: method.parse().map_err(|_| McpError::InvalidParams("Invalid method".into()))?,
            url: url.to_string(),
            headers: vec![],
            body: None,
            query: vec![],
        };
        
        // 解析 headers
        if let Some(headers) = args.get("headers").and_then(|v| v.as_object()) {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request.headers.push(Header {
                        name: key.clone(),
                        value: value_str.to_string(),
                    });
                }
            }
        }
        
        // 解析 body
        if let Some(body) = args.get("body").and_then(|v| v.as_str()) {
            request.body = Some(body.as_bytes().to_vec());
        }
        
        // 发送请求
        let response = self.http_service.send_request(&request).await?;
        
        // 格式化响应
        let output = format!(
            "Status: {} {}\nTime: {}ms\nSize: {} bytes\n\n{}",
            response.status_code,
            response.status_text,
            response.duration_ms,
            response.size,
            String::from_utf8_lossy(&response.body)
        );
        
        Ok(output)
    }
}
```

### 13.5 侧边栏树形组件

```rust
// src/ui/collection/tree_view.rs

use gpui::*;

pub struct CollectionTreeView {
    collections: Vec<Collection>,
    expanded: HashSet<String>,
    selected: Option<SelectedItem>,
}

#[derive(Clone, Debug)]
enum SelectedItem {
    Collection(String),
    Folder(String, String), // collection_id, folder_id
    Request(String),
}

impl CollectionTreeView {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        // 订阅集合变化
        let storage = cx.global::<AppState>().storage.clone();
        
        Self {
            collections: vec![],
            expanded: HashSet::new(),
            selected: None,
        }
    }
    
    pub fn set_collections(&mut self, collections: Vec<Collection>, cx: &mut ViewContext<Self>) {
        self.collections = collections;
        cx.notify();
    }
    
    fn toggle_expand(&mut self, id: String, cx: &mut ViewContext<Self>) {
        if self.expanded.contains(&id) {
            self.expanded.remove(&id);
        } else {
            self.expanded.insert(id);
        }
        cx.notify();
    }
    
    fn select_item(&mut self, item: SelectedItem, cx: &mut ViewContext<Self>) {
        self.selected = Some(item.clone());
        
        match item {
            SelectedItem::Request(request_id) => {
                // 触发请求加载事件
                cx.emit(CollectionTreeEvent::RequestSelected(request_id));
            }
            _ => {}
        }
        
        cx.notify();
    }
}

impl Render for CollectionTreeView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x252526))
            .overflow_y_scroll()
            .children(self.collections.iter().map(|col| {
                self.render_collection(col, cx)
            }))
    }
}

impl CollectionTreeView {
    fn render_collection(
        &self,
        collection: &Collection,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let collection_id = collection.id.clone();
        let is_expanded = self.expanded.contains(&collection_id);
        let is_selected = matches!(
            self.selected,
            Some(SelectedItem::Collection(ref id)) if id == &collection_id
        );
        
        div()
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_2()
                    .py_1()
                    .cursor_pointer()
                    .when(is_selected, |div| {
                        div.bg(rgb(0x37373d))
                    })
                    .on_click(cx.listener(move |this, _, cx| {
                        this.toggle_expand(collection_id.clone(), cx);
                    }))
                    .child(if is_expanded { "▼" } else { "▶" })
                    .child(
                        div()
                            .ml_1()
                            .child(collection.name.clone())
                    )
            )
            .when(is_expanded, |div| {
                // 渲染子项
                div.children(
                    collection.folders.iter().map(|folder| {
                        self.render_folder(&collection_id, folder, cx)
                    })
                )
                .children(
                    collection.requests.iter().map(|req| {
                        self.render_request(req, 1, cx)
                    })
                )
            })
    }
    
    fn render_folder(
        &self,
        collection_id: &str,
        folder: &Folder,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let folder_id = folder.id.clone();
        let is_expanded = self.expanded.contains(&folder_id);
        
        div()
            .flex_col()
            .ml_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_2()
                    .py_1()
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, cx| {
                        this.toggle_expand(folder_id.clone(), cx);
                    }))
                    .child(if is_expanded { "▼" } else { "▶" })
                    .child(
                        div()
                            .ml_1()
                            .child(folder.name.clone())
                    )
            )
            .when(is_expanded, |div| {
                // 递归渲染子文件夹
                div.children(
                    folder.children.iter().map(|child| {
                        self.render_folder(collection_id, child, cx)
                    })
                )
                .children(
                    folder.requests.iter().map(|req| {
                        self.render_request(req, 2, cx)
                    })
                )
            })
    }
    
    fn render_request(
        &self,
        request: &Request,
        indent: usize,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let request_id = request.id.clone();
        let is_selected = matches!(
            self.selected,
            Some(SelectedItem::Request(ref id)) if id == &request_id
        );
        
        let method_color = match request.method {
            HttpMethod::GET => rgb(0x4ec9b0),
            HttpMethod::POST => rgb(0x569cd6),
            HttpMethod::PUT => rgb(0xdcdcaa),
            HttpMethod::DELETE => rgb(0xf44747),
            HttpMethod::PATCH => rgb(0xce9178),
            HttpMethod::HEAD => rgb(0x808080),
            HttpMethod::OPTIONS => rgb(0x808080),
        };
        
        div()
            .flex()
            .items_center()
            .px_2()
            .py_1()
            .ml(indent as f32)
            .cursor_pointer()
            .when(is_selected, |div| {
                div.bg(rgb(0x37373d))
            })
            .on_click(cx.listener(move |this, _, cx| {
                this.select_item(SelectedItem::Request(request_id.clone()), cx);
            }))
            .child(
                div()
                    .w_8()
                    .text_color(method_color)
                    .child(request.method.to_string())
            )
            .child(request.name.clone())
    }
}
```

### 13.6 代码编辑器组件（语法高亮）

```rust
// src/ui/editor/code_editor.rs

use gpui::*;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;

pub struct CodeEditor {
    /// 编辑器内容
    content: String,
    
    /// 语法设置
    syntax_set: SyntaxSet,
    syntax: String,
    
    /// 主题
    theme: Theme,
    
    /// 光标位置
    cursor: Point,
    
    /// 选择范围
    selection: Option<Range<usize>>,
    
    /// 是否获得焦点
    focused: bool,
}

impl CodeEditor {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        
        Self {
            content: String::new(),
            syntax_set,
            syntax: "javascript".to_string(),
            theme: theme_set.themes["Base16 Eighties Dark"].clone(),
            cursor: Point { x: 0.0, y: 0.0 },
            selection: None,
            focused: false,
        }
    }
    
    pub fn set_content(&mut self, content: String, cx: &mut ViewContext<Self>) {
        self.content = content;
        cx.notify();
    }
    
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
    
    fn insert_char(&mut self, c: char, cx: &mut ViewContext<Self>) {
        let pos = self.cursor.x as usize;
        self.content.insert(pos, c);
        self.cursor.x += 1.0;
        cx.notify();
    }
    
    fn insert_newline(&mut self, cx: &mut ViewContext<Self>) {
        let pos = self.cursor.x as usize;
        self.content.insert(pos, '\n');
        self.cursor.x = 0.0;
        self.cursor.y += 20.0; // 行高
        cx.notify();
    }
    
    fn delete_char(&mut self, cx: &mut ViewContext<Self>) {
        let pos = self.cursor.x as usize;
        if pos > 0 {
            self.content.remove(pos - 1);
            self.cursor.x -= 1.0;
        }
        cx.notify();
    }
}

impl Render for CodeEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let syntax = self.syntax_set
            .find_syntax_by_token(&self.syntax)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        
        let mut highlighter = HighlightLines::new(syntax, &self.theme);
        
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .font_family("Monaco")
            .font_size(px(13.0))
            .on_click(cx.listener(|this, _, cx| {
                this.focused = true;
                cx.notify();
            }))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, cx| {
                if !this.focused {
                    return;
                }
                
                match event.keystroke.as_str() {
                    "backspace" => this.delete_char(cx),
                    "enter" => this.insert_newline(cx),
                    keystroke if keystroke.len() == 1 => {
                        this.insert_char(keystroke.chars().next().unwrap(), cx);
                    }
                    _ => {}
                }
                
                event.stop_propagation();
            }))
            .children(
                LinesWithEndings::from(&self.content)
                    .enumerate()
                    .map(|(i, line)| {
                        let ranges = highlighter.highlight_line(line, &self.syntax_set)
                            .unwrap_or_default();
                        
                        div()
                            .flex()
                            .h(px(20.0))
                            .px_2()
                            .items_center()
                            .children(ranges.into_iter().map(|(style, text)| {
                                div()
                                    .text_color(rgb(style.foreground.r))
                                    .child(text.to_string())
                            }))
                    })
            )
            .when(self.focused, |div| {
                // 渲染光标
                div.child(
                    div()
                        .absolute()
                        .left(px(self.cursor.x))
                        .top(px(self.cursor.y))
                        .w(px(2.0))
                        .h(px(16.0))
                        .bg(rgb(0xffffff))
                )
            })
    }
}
```

---

## 14. 错误处理与日志

### 14.1 错误类型定义

```rust
// src/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostboyError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    
    #[error("Script error: {0}")]
    Script(#[from] ScriptError),
    
    #[error("MCP error: {0}")]
    Mcp(#[from] McpError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Execution error: {0}")]
    Execution(String),
    
    #[error("Timeout")]
    Timeout,
}

#[derive(Error, Debug)]
pub enum McpError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    
    #[error("Invalid params: {0}")]
    InvalidParams(String),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
}

// 结果类型
pub type Result<T> = std::result::Result<T, PostboyError>;
```

### 14.2 日志系统

```rust
// src/logging.rs

use tracing::{info, warn, error, debug};
use tracing_subscriber;

pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_thread_ids(true)
        .init();
}

pub fn log_request(request: &HttpRequest) {
    debug!(
        method = %request.method,
        url = %request.url,
        "Sending request"
    );
}

pub fn log_response(response: &HttpResponse, duration: u64) {
    debug!(
        status = response.status_code,
        duration_ms = duration,
        size = response.size,
        "Received response"
    );
}

pub fn log_script_execution(script: &str, success: bool) {
    if success {
        debug!("Script executed successfully");
    } else {
        warn!("Script execution failed");
    }
}
```

---

## 15. 测试策略

### 15.1 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_http_service() {
        let service = HttpService::new().unwrap();
        let request = HttpRequest {
            method: HttpMethod::GET,
            url: "https://httpbin.org/get".to_string(),
            headers: vec![],
            body: None,
            query: vec![],
        };
        
        let response = service.send_request(&request).await.unwrap();
        assert_eq!(response.status_code, 200);
    }
    
    #[test]
    fn test_variable_parsing() {
        let mut vars = HashMap::new();
        vars.insert("base_url".to_string(), "https://api.example.com".to_string());
        
        let input = "{{base_url}}/users";
        let output = resolve_variables(input, &vars).unwrap();
        assert_eq!(output, "https://api.example.com/users");
    }
    
    #[tokio::test]
    async fn test_script_execution() {
        let engine = ScriptEngine::new();
        let mut context = ScriptContext::default();
        
        let script = r#"
            pm.environment.set("test", "value");
        "#;
        
        let result = engine.execute_pre_request(script, &mut context).await.unwrap();
        assert!(result.modified_variables.contains_key("test"));
    }
}
```

### 15.2 集成测试

```rust
#[tokio::test]
async fn test_full_request_flow() {
    // 初始化服务
    let storage = StorageService::new(":memory:").await.unwrap();
    let http = HttpService::new().unwrap();
    let script_engine = ScriptEngine::new();
    let executor = RequestExecutor::new(http, script_engine, storage);
    
    // 创建测试请求
    let request = Request {
        id: Uuid::new_v4(),
        name: "Test Request".to_string(),
        method: HttpMethod::GET,
        url: "https://httpbin.org/get".to_string(),
        ..Default::default()
    };
    
    // 执行请求
    let environment = Environment::default();
    let result = executor.execute_request(request, &environment).await.unwrap();
    
    assert_eq!(result.response.status_code, 200);
}
```

---

## 16. 参考资料

- [GPUI GitHub](https://github.com/zed-industries/zed)
- [Boa JavaScript Engine](https://github.com/boa-dev/boa)
- [MCP Protocol](https://modelcontextprotocol.io/)
- [Reqwest](https://docs.rs/reqwest/)
- [SQLx](https://docs.rs/sqlx/)
- [Tokio](https://tokio.rs/)
- [Syntect](https://docs.rs/syntect/)

---

这个深化设计方案提供了关键模块的详细实现代码，可以作为开发的蓝图。接下来可以开始搭建项目基础结构并逐步实现各个模块。
        );

        // pm.environment.toObject()
        let to_object_fn = boa_engine::JsNativeFunction::from_copy_closure_with_catches(
            move |_, _, context| {
                let env_ref = env.borrow();
                let obj = boa_engine::object::JsObject::default();
                for (key, value) in &env_ref.variables {
                    obj.set(key.as_str(), value.as_str(), false, context)?;
                }
                Ok(obj.into())
            },
            None,
        );

        obj.set("set", set_fn, false, &mut self.context).unwrap();
        obj.set("get", get_fn, false, &mut self.context).unwrap();
        obj.set("unset", unset_fn, false, &mut self.context).unwrap();
        obj.set("toObject", to_object_fn, false, &mut self.context).unwrap();
    }

    fn register_request_methods(&mut self, obj: Gc<boa_engine::object::JsObject>) {
        let request_data = self.request_data.clone();

        // pm.request.url (getter/setter)
        let req_data_clone = request_data.clone();
        let url_getter = boa_engine::JsNativeFunction::from_copy_closure_with_catches(
            move |_, _, _| {
                Ok(JsValue::String(req_data_clone.borrow().url.clone().into()))
            },
            None,
        );

        let req_data_clone = request_data.clone();
        let url_setter = boa_engine::JsNativeFunction::from_copy_closure_with_catches(
            move |_, args, context| {
                if !args.is_empty() {
                    let url = args[0].to_string(context)?.to_std_string();
                    req_data_clone.borrow_mut().url = url;
                }
                Ok(JsValue::Undefined)
            },
            None,
        );

        obj.set("url", url_getter, false, &mut self.context).unwrap();
    }

    fn register_console_object(&mut self) {
        let console_obj = boa_engine::object::JsObject::default();

        // console.log
        let log_fn = boa_engine::JsNativeFunction::from_copy_closure_with_catches(
            |_, args, context| {
                let output: Vec<String> = args.iter()
                    .map(|arg| arg.to_string(context).unwrap().to_std_string())
                    .collect();
                println!("[Console] {}", output.join(" "));
                Ok(JsValue::Undefined)
            },
            None,
        );

        // console.error
        let error_fn = boa_engine::JsNativeFunction::from_copy_closure_with_catches(
            |_, args, context| {
                let output: Vec<String> = args.iter()
                    .map(|arg| arg.to_string(context).unwrap().to_std_string())
                    .collect();
                eprintln!("[Console Error] {}", output.join(" "));
                Ok(JsValue::Undefined)
            },
            None,
        );

        console_obj.set("log", log_fn, false, &mut self.context).unwrap();
        console_obj.set("error", error_fn, false, &mut self.context).unwrap();

        self.context.register_global_property("console", console_obj).unwrap();
    }

    fn register_require_function(&mut self) {
        // require() 用于加载内置模块
        let require_fn = boa_engine::JsNativeFunction::from_copy_closure_with_catches(
            |this, args, context| {
                if args.is_empty() {
                    return Err(JsNativeError::typ()
                        .with_message("require() needs a module name")
                        .into());
                }
                let module_name = args[0].to_string(context)?.to_std_string();
                
                match module_name.as_str() {
                    "crypto" => Self::create_crypto_module(context),
                    "lodash" => Self::create_lodash_module(context),
                    "moment" => Self::create_moment_module(context),
                    _ => Ok(JsValue::Undefined),
                }
            },
            None,
        );

        self.context.register_global_property("require", require_fn).unwrap();
    }

    fn create_crypto_module(context: &mut Context) -> JsValue {
        let crypto_obj = boa_engine::object::JsObject::default();

        // createHash
        let create_hash = boa_engine::JsNativeFunction::from_copy_closure_with_catches(
            |_, args, context| {
                let algorithm = if !args.is_empty() {
                    args[0].to_string(context)?.to_std_string()
                } else {
                    "sha256".to_string()
                };
                
                let hash_obj = boa_engine::object::JsObject::default();
                // 实现哈希功能...
                Ok(hash_obj.into())
            },
            None,
        );

        crypto_obj.set("createHash", create_hash, false, context).unwrap();
        crypto_obj.into()
    }

    fn create_lodash_module(context: &mut Context) -> JsValue {
        let lodash_obj = boa_engine::object::JsObject::default();

        // _.get
        let get_fn = boa_engine::JsNativeFunction::from_copy_closure_with_catches(
            |_, args, context| {
                if args.len() < 2 {
                    return Ok(JsValue::Undefined);
                }
                // 实现 JSONPath 获取
                let obj = &args[0];
                let path = args[1].to_string(context)?.to_std_string();
                
                // 简单的点符号路径解析
                let segments: Vec<&str> = path.split('.').collect();
                let mut result = obj.clone();
                
                for segment in segments {
                    if let Some(obj) = result.as_object() {
                        let key = boa_engine::property::PropertyKey::String(segment.into());
                        result = obj.get(key, context).unwrap_or(JsValue::Undefined);
                    } else {
                        result = JsValue::Undefined;
                        break;
                    }
                }
                
                Ok(result)
            },
            None,
        );

        lodash_obj.set("get", get_fn, false, context).unwrap();
        lodash_obj.into()
    }

    fn create_moment_module(context: &mut Context) -> JsValue {
        let moment_obj = boa_engine::object::JsObject::default();

        let now_fn = boa_engine::JsNativeFunction::from_copy_closure_with_catches(
            |_, _, _| {
                use std::time::{SystemTime, UNIX_EPOCH};
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64;
                Ok(JsValue::Integer(timestamp))
            },
            None,
        );

        moment_obj.set("now", now_fn, false, context).unwrap();
        moment_obj.into()
    }

    /// 设置环境变量
    pub fn set_environment(&mut self, variables: HashMap<String, String>) {
        self.environment.borrow_mut().variables = variables;
    }

    /// 设置全局变量
    pub fn set_globals(&mut self, variables: HashMap<String, String>) {
        self.globals.borrow_mut().variables = variables;
    }

    /// 设置请求数据
    pub fn set_request(&mut self, request: &crate::models::Request) {
        let mut req_data = self.request_data.borrow_mut();
        req_data.url = request.url.clone();
        req_data.method = request.method.to_string();
        req_data.headers = request.headers.iter()
            .map(|h| (h.key.clone(), h.value.clone()))
            .collect();
        req_data.query = request.params.iter()
            .map(|p| (p.key.clone(), p.value.clone()))
            .collect();
        req_data.body = request.body.as_raw().map(|b| b.clone());
    }

    /// 设置响应数据（用于 post-response 脚本）
    pub fn set_response(&mut self, response: &crate::models::Response) {
        let resp_data = ResponseData {
            code: response.status,
            status: response.status_text.clone(),
            headers: response.headers.iter()
                .map(|h| (h.key.clone(), h.value.clone()))
                .collect(),
            body: response.body.as_text().unwrap_or_default(),
            response_time: response.duration,
            size: response.size,
        };
        self.response_data = Some(Gc::new(GcCell::new(resp_data)));

        // 注册 pm.response
        if let Some(resp_obj) = self.create_response_object() {
            if let Some(pm) = self.context.global_object().get("pm", &mut self.context).ok() {
                if let Some(pm_obj) = pm.as_object() {
                    pm_obj.set("response", resp_obj, false, &mut self.context).ok();
                }
            }
        }
    }

    fn create_response_object(&mut self) -> Option<Gc<boa_engine::object::JsObject>> {
        let resp_data = self.response_data.as_ref()?.clone();
        let test_results = self.test_results.clone();

        let obj = boa_engine::object::JsObject::default();

        // pm.response.code
        let resp_clone = resp_data.clone();
        let code_getter = boa_engine::JsNativeFunction::from_copy_closure_with_catches(
            move |_, _, _| Ok(JsValue::Integer(resp_clone.borrow().code as i32)),
            None,
        );
        obj.set("code", code_getter, false, &mut self.context).ok();

        // pm.response.json()
        let resp_clone = resp_data.clone();
        let json_fn = boa_engine::JsNativeFunction::from_copy_closure_with_catches(
            move |_, _, context| {
                let body = &resp_clone.borrow().body;
                match serde_json::from_str::<serde_json::Value>(body) {
                    Ok(json) => {
                        let js_value = JsValue::from_json(&json, context)?;
                        Ok(js_value)
                    }
                    Err(_) => Ok(JsValue::Undefined),
                }
            },
            None,
        );
        obj.set("json", json_fn, false, &mut self.context).ok();

        // pm.test()
        let test_fn = boa_engine::JsNativeFunction::from_copy_closure_with_catches(
            move |_, args, context| {
                let name = if !args.is_empty() {
                    args[0].to_string(context)?.to_std_string()
                } else {
                    "Unnamed test".to_string()
                };

                let callback = args.get(1).and_then(|a| a.as_object());
                
                // 执行测试函数
                let passed = match callback {
                    Some(cb) => {
                        // 调用测试函数
                        match cb.call(&JsValue::Undefined, &[], context) {
                            Ok(_) => true,
                            Err(e) => false,
                        }
                    }
                    None => true,
                };

                test_results.borrow_mut().push(TestResult {
                    name: name.clone(),
                    passed,
                    error_message: if passed { None } else { Some("Test failed".into()) },
                });

                Ok(JsValue::Undefined)
            },
            None,
        );
        obj.set("test", test_fn, false, &mut self.context).ok();

        Some(Gc::new(obj))
    }

    /// 执行脚本
    pub fn execute(&mut self, script: &str) -> Result<ScriptExecutionResult, ScriptError> {
        // 清空之前的测试结果
        self.test_results.borrow_mut().clear();
        
        // 清空之前的修改记录
        self.environment.borrow_mut().modified.clear();
        self.globals.borrow_mut().modified.clear();

        // 执行脚本
        let source = Source::from_bytes(script);
        match self.context.eval(source) {
            Ok(_) => {
                let test_results = self.test_results.borrow().clone();
                let env_modified = self.environment.borrow().modified.clone();
                let global_modified = self.globals.borrow().modified.clone();
                let request_modified = self.extract_request_changes();

                Ok(ScriptExecutionResult {
                    success: true,
                    test_results,
                    environment_changes: env_modified,
                    global_changes: global_modified,
                    request_changes: request_modified,
                    error: None,
                })
            }
            Err(e) => {
                let error_msg = format!("Script execution error: {}", e);
                let mut test_results = self.test_results.borrow().clone();
                test_results.push(TestResult {
                    name: "Script execution".into(),
                    passed: false,
                    error_message: Some(error_msg.clone()),
                });

                Ok(ScriptExecutionResult {
                    success: false,
                    test_results,
                    environment_changes: Vec::new(),
                    global_changes: Vec::new(),
                    request_changes: None,
                    error: Some(error_msg),
                })
            }
        }
    }

    fn extract_request_changes(&self) -> Option<RequestChanges> {
        let req_data = self.request_data.borrow();
        if req_data.url.is_empty() {
            return None;
        }

        Some(RequestChanges {
            url: if req_data.url.is_empty() { None } else { Some(req_data.url.clone()) },
            headers: if req_data.headers.is_empty() { None } else { Some(req_data.headers.clone()) },
            query: if req_data.query.is_empty() { None } else { Some(req_data.query.clone()) },
        })
    }
}

#[derive(Debug, Clone)]
pub struct ScriptExecutionResult {
    pub success: bool,
    pub test_results: Vec<TestResult>,
    pub environment_changes: Vec<(String, String)>,
    pub global_changes: Vec<(String, String)>,
    pub request_changes: Option<RequestChanges>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RequestChanges {
    pub url: Option<String>,
    pub headers: Option<Vec<(String, String)>>,
    pub query: Option<Vec<(String, String)>>,
}

#[derive(Debug, thiserror::Error)]
pub enum ScriptError {
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}
```

### 14.2 UI 组件系统设计

#### 组件状态管理

```rust
// src/ui/state.rs

use gpui::*;
use std::sync::Arc;

/// 全局应用状态
pub struct AppState {
    /// 当前选中的请求
    pub selected_request: Option<Model<Request>>,
    /// 当前选中的集合
    pub selected_collection: Option<Model<Collection>>,
    /// 当前激活的环境
    pub active_environment: Option<Model<Environment>>,
    /// 最近请求历史
    pub recent_history: Vec<Model<HistoryEntry>>,
    /// 侧边栏展开状态
    pub sidebar_expanded: bool,
    /// 响应面板可见状态
    pub response_visible: bool,
}

/// 请求编辑状态
pub struct RequestEditorState {
    /// HTTP 方法
    pub method: HttpMethod,
    /// 请求 URL
    pub url: String,
    /// 当前选中的标签页
    pub active_tab: RequestTab,
    /// 请求头
    pub headers: Vec<HeaderEntry>,
    /// 查询参数
    pub params: Vec<ParamEntry>,
    /// 请求体
    pub body: RequestBodyState,
    /// 认证配置
    pub auth: AuthConfigState,
    /// Pre-request 脚本
    pub pre_request_script: String,
    /// Post-response 脚本
    pub post_response_script: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RequestTab {
    Params,
    Headers,
    Body,
    Auth,
    PreRequestScript,
    PostResponseScript,
}

/// 响应面板状态
pub struct ResponseState {
    /// HTTP 响应
    pub response: Option<HttpResponse>,
    /// 当前选中的标签页
    pub active_tab: ResponseTab,
    /// 测试结果
    pub test_results: Vec<TestResult>,
    /// 是否正在加载
    pub loading: bool,
}

#[derive(Clone这个设计方案提供了一个完整的 Rust + GPUI 实现的 Postboy 架构。如果你同意这个方案，我可以开始实现代码。你想从哪个部分开始？