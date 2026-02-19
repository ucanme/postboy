# Postboy API 设计文档

## 目录

1. [脚本 Hook API](#1-脚本-hook-api)
2. [MCP 协议实现](#2-mcp-协议实现)
3. [内部服务 API](#3-内部服务-api)
4. [数据持久化 API](#4-数据持久化-api)

---

## 1. 脚本 Hook API

### 1.1 Pre-request Script API

在请求发送前执行的脚本，提供 `pm` 全局对象。

#### 可用对象

```javascript
// pm.environment - 环境变量操作
pm.environment.set("key", "value")        // 设置变量
pm.environment.get("key")                 // 获取变量
pm.environment.unset("key")               // 删除变量
pm.environment.clear()                    // 清空所有变量
pm.environment.toObject()                 // 转换为普通对象

// pm.globals - 全局变量操作
pm.globals.set("key", "value")
pm.globals.get("key")
pm.globals.unset("key")
pm.globals.clear()
pm.globals.toObject()

// pm.request - 请求对象（可修改）
pm.request.url                             // 请求 URL
pm.request.method                          // HTTP 方法
pm.request.headers                         // 请求头数组
pm.request.headers.add({key: "X-Auth", value: "token"})
pm.request.headers.remove("X-Old-Header")
pm.request.body                            // 请求体
pm.request.query                           // 查询参数

// pm.variables - 变量解析（优先级：环境 > 全局）
pm.variables.get("key")                    // 按优先级获取变量

// pm.sendRequest - 辅助发送请求（用于获取 token 等）
pm.sendRequest("https://api.example.com/auth", (err, res) => {
    if (err) {
        console.log(err);
    } else {
        pm.environment.set("token", res.json().token);
    }
});

// console - 调试输出
console.log("message")
console.error("error")
console.warn("warning")
```

#### 示例

```javascript
// 动态设置认证 token
const token = pm.environment.get("api_token");
if (token) {
    pm.request.headers.add({
        key: "Authorization",
        value: `Bearer ${token}`
    });
}

// 先发送认证请求获取新 token
pm.sendRequest({
    url: "https://api.example.com/login",
    method: "POST",
    body: {
        mode: "raw",
        raw: JSON.stringify({
            username: pm.environment.get("username"),
            password: pm.environment.get("password")
        })
    }
}, (err, res) => {
    if (!err && res.code === 200) {
        pm.environment.set("access_token", res.json().access_token);
    }
});

// 生成时间戳
pm.environment.set("timestamp", Date.now());

// 计算签名
const crypto = require("crypto");
const payload = pm.request.url + pm.request.body.raw;
const signature = crypto.createHmac("sha256", pm.environment.get("secret"))
    .update(payload)
    .digest("hex");
pm.request.headers.add({
    key: "X-Signature",
    value: signature
});
```

### 1.2 Post-response Script API

在响应返回后执行的脚本，用于断言测试和数据提取。

#### 可用对象

```javascript
// pm.response - 响应对象（只读）
pm.response.code                           // 状态码
pm.response.status                         // 状态文本
pm.response.headers                        // 响应头
pm.response.responseTime                   // 响应时间（毫秒）
pm.response.responseSize                   // 响应大小（字节）
pm.response.text()                         // 响应体文本
pm.response.json()                         // 解析为 JSON

// pm.test - 测试断言
pm.test("Test name", function() {
    // 断言逻辑
});

// pm.expect - BDD 风格断言（类似 Chai）
pm.expect(pm.response.code).to.equal(200);
pm.expect(pm.response.json()).to.have.property("data");

// pm.environment / pm.globals - 可用于更新变量
pm.environment.set("user_id", pm.response.json().id);
```

#### 内置断言方法

```javascript
// pm.expect(actual).to.equal(expected)
pm.expect(1).to.equal(1);

// pm.expect(actual).to.have.property(name)
pm.expect(obj).to.have.property("id");

// pm.expect(actual).to.include(value)
pm.expect([1, 2, 3]).to.include(2);

// pm.expect(actual).to.be.ok
pm.expect(value).to.be.ok;

// pm.expect(actual).to.be.null / .to.be.undefined
pm.expect(value).to.be.null;

// pm.expect(actual).to.be.above / .to.be.below
pm.expect(10).to.be.above(5);

// pm.expect(actual).to.match(regex)
pm.expect("hello@example.com").to.match(/@/);

// pm.response.to.have.status(code)
pm.response.to.have.status(200);

// pm.response.to.have.header(name)
pm.response.to.have.header("Content-Type");

// pm.response.to.have.body()
pm.response.to.have.body();

// pm.response.to.have.jsonBody()
pm.response.to.have.jsonBody();

// pm.response.to.have.jsonSchema(schema)
pm.response.to.have.jsonSchema({
    type: "object",
    required: ["id", "name"]
});
```

#### 示例

```javascript
// 基本状态码检查
pm.test("Status code is 200", function() {
    pm.response.to.have.status(200);
});

// 检查响应头
pm.test("Content-Type is present", function() {
    pm.response.to.have.header("Content-Type");
    pm.expect(pm.response.headers.get("Content-Type"))
        .to.include("application/json");
});

// JSON 响应验证
pm.test("Response has correct structure", function() {
    const json = pm.response.json();
    pm.expect(json).to.be.an("object");
    pm.expect(json).to.have.property("data");
    pm.expect(json.data).to.be.an("array");
    pm.expect(json.data[0]).to.have.property("id");
    pm.expect(json.data[0]).to.have.property("name");
});

// 性能测试
pm.test("Response time is acceptable", function() {
    pm.expect(pm.response.responseTime).to.be.below(500);
});

// 保存响应数据到环境变量
pm.test("Save user ID", function() {
    const json = pm.response.json();
    pm.environment.set("current_user_id", json.id);
});

// 数组长度验证
pm.test("Data array has items", function() {
    const json = pm.response.json();
    pm.expect(json.data).to.have.length.above(0);
});

// 嵌套属性验证
pm.test("User has email", function() {
    const json = pm.response.json();
    pm.expect(json.user).to.have.property("email");
    pm.expect(json.user.email).to.match(/^[^@]+@[^@]+\.[^@]+$/);
});

// 条件测试
pm.test("Error message on failure", function() {
    if (pm.response.code !== 200) {
        const json = pm.response.json();
        pm.expect(json).to.have.property("error");
        pm.expect(json.error).to.have.property("message");
    }
});

// JSON Schema 验证
pm.test("Schema validation", function() {
    const schema = {
        type: "object",
        properties: {
            id: { type: "number" },
            name: { type: "string" },
            email: { type: "string" }
        },
        required: ["id", "name"]
    };
    pm.response.to.have.jsonSchema(schema);
});

// 遍历数组验证每个项目
pm.test("All items have required fields", function() {
    const json = pm.response.json();
    json.data.forEach(item => {
        pm.expect(item).to.have.property("id");
        pm.expect(item).to.have.property("name");
    });
});
```

### 1.3 脚本执行上下文

```rust
// 脚本上下文结构
pub struct ScriptContext {
    // 环境变量
    pub environment: HashMap<String, String>,
    pub global: HashMap<String, String>,
    
    // 请求对象
    pub request: RequestContext,
    
    // 响应对象（仅 Post-response）
    pub response: Option<ResponseContext>,
    
    // 变量变更记录
    pub modified_variables: Vec<VariableChange>,
}

pub struct RequestContext {
    pub url: String,
    pub method: HttpMethod,
    pub headers: Vec<Header>,
    pub body: Option<String>,
    pub query: Vec<Param>,
}

pub struct ResponseContext {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<Header>,
    pub body: String,
    pub response_time: u64,
    pub size: u64,
}

// 脚本执行结果
pub struct ScriptResult {
    pub success: bool,
    pub error: Option<String>,
    pub modified_variables: HashMap<String, String>,
    pub modified_request: Option<Request>,
    pub test_results: Vec<TestResult>,
}

pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub error: Option<String>,
}
```

---

## 2. MCP 协议实现

### 2.1 MCP Server 端点

Postboy 实现的 MCP Server 支持以下端点：

#### 初始化

```json
// Request
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": {
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "roots": {
                "listChanged": true
            },
            "sampling": {}
        },
        "clientInfo": {
            "name": "test-client",
            "version": "1.0.0"
        }
    }
}

// Response
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "tools": {},
            "resources": {
                "subscribe": true,
                "listChanged": true
            },
            "prompts": {
                "listChanged": true
            }
        },
        "serverInfo": {
            "name": "postboy",
            "version": "1.0.0"
        }
    }
}
```

#### 工具列表

```rust
// src/mcp/tools.rs

pub const MCP_TOOLS: &[ToolDefinition] = &[
    ToolDefinition {
        name: "send_request",
        description: "Send an HTTP request and return the response",
        input_schema: json!({
            "type": "object",
            "properties": {
                "method": {
                    "type": "string",
                    "enum": ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"],
                    "description": "HTTP method"
                },
                "url": {
                    "type": "string",
                    "description": "Request URL"
                },
                "headers": {
                    "type": "object",
                    "description": "Request headers as key-value pairs",
                    "additionalProperties": { "type": "string" }
                },
                "body": {
                    "type": "string",
                    "description": "Request body (for POST, PUT, PATCH)"
                },
                "query": {
                    "type": "object",
                    "description": "Query parameters",
                    "additionalProperties": { "type": "string" }
                }
            },
            "required": ["method", "url"]
        })
    },
    
    ToolDefinition {
        name: "list_collections",
        description: "List all API collections in the workspace",
        input_schema: json!({
            "type": "object",
            "properties": {}
        })
    },
    
    ToolDefinition {
        name: "get_collection",
        description: "Get details of a specific collection",
        input_schema: json!({
            "type": "object",
            "properties": {
                "collection_id": {
                    "type": "string",
                    "description": "Collection ID"
                }
            },
            "required": ["collection_id"]
        })
    },
    
    ToolDefinition {
        name: "create_request",
        description: "Create a new API request",
        input_schema: json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "collection_id": { "type": "string" },
                "folder_id": { "type": "string" },
                "method": { "type": "string" },
                "url": { "type": "string" },
                "headers": { "type": "object" },
                "body": { "type": "string" },
                "pre_request_script": { "type": "string" },
                "post_response_script": { "type": "string" }
            },
            "required": ["name", "method", "url"]
        })
    },
    
    ToolDefinition {
        name: "get_request",
        description: "Get a specific request by ID",
        input_schema: json!({
            "type": "object",
            "properties": {
                "request_id": { "type": "string" }
            },
            "required": ["request_id"]
        })
    },
    
    ToolDefinition {
        name: "update_request",
        description: "Update an existing request",
        input_schema: json!({
            "type": "object",
            "properties": {
                "request_id": { "type": "string" },
                "name": { "type": "string" },
                "url": { "type": "string" },
                "method": { "type": "string" },
                "headers": { "type": "object" },
                "body": { "type": "string" }
            },
            "required": ["request_id"]
        })
    },
    
    ToolDefinition {
        name: "delete_request",
        description: "Delete a request",
        input_schema: json!({
            "type": "object",
            "properties": {
                "request_id": { "type": "string" }
            },
            "required": ["request_id"]
        })
    },
    
    ToolDefinition {
        name: "set_environment_variable",
        description: "Set an environment variable",
        input_schema: json!({
            "type": "object",
            "properties": {
                "key": { "type": "string" },
                "value": { "type": "string" },
                "environment_id": { "type": "string" }
            },
            "required": ["key", "value"]
        })
    },
    
    ToolDefinition {
        name: "get_environment_variables",
        description: "Get all environment variables",
        input_schema: json!({
            "type": "object",
            "properties": {
                "environment_id": { "type": "string" }
            }
        })
    },
    
    ToolDefinition {
        name: "run_collection",
        description: "Run all requests in a collection",
        input_schema: json!({
            "type": "object",
            "properties": {
                "collection_id": { "type": "string" },
                "environment_id": { "type": "string" }
            },
            "required": ["collection_id"]
        })
    },
    
    ToolDefinition {
        name: "export_collection",
        description: "Export a collection to JSON (Postman v2.1 format)",
        input_schema: json!({
            "type": "object",
            "properties": {
                "collection_id": { "type": "string" },
                "format": {
                    "type": "string",
                    "enum": ["postman_v2.1", "openapi_3.0"]
                }
            },
            "required": ["collection_id"]
        })
    },
];
```

#### 资源列表

```rust
// src/mcp/resources.rs

pub const MCP_RESOURCES: &[ResourceDefinition] = &[
    ResourceDefinition {
        uri: "postboy://collections",
        name: "All Collections",
        description: "Complete list of all API collections",
        mime_type: "application/json",
    },
    
    ResourceDefinition {
        uri: "postboy://collections/{collection_id}",
        name: "Collection Details",
        description: "Details of a specific collection including all requests",
        mime_type: "application/json",
    },
    
    ResourceDefinition {
        uri: "postboy://environments",
        name: "Environments",
        description: "List of all environments",
        mime_type: "application/json",
    },
    
    ResourceDefinition {
        uri: "postboy://environments/{environment_id}",
        name: "Environment Variables",
        description: "Variables in a specific environment",
        mime_type: "application/json",
    },
    
    ResourceDefinition {
        uri: "postboy://history",
        name: "Request History",
        description: "Recent request history",
        mime_type: "application/json",
    },
];
```

#### Prompt 模板

```rust
// src/mcp/prompts.rs

pub const MCP_PROMPTS: &[PromptDefinition] = &[
    PromptDefinition {
        name: "analyze_api",
        description: "Analyze an API endpoint and suggest tests",
        arguments: vec![
            PromptArgument {
                name: "url",
                description: "API endpoint URL",
                required: true,
            },
            PromptArgument {
                name: "method",
                description: "HTTP method",
                required: false,
            },
        ],
    },
    
    PromptDefinition {
        name: "generate_tests",
        description: "Generate test scripts for a response",
        arguments: vec![
            PromptArgument {
                name: "response_body",
                description: "Sample response body",
                required: true,
            },
        ],
    },
    
    PromptDefinition {
        name: "create_collection_from_openapi",
        description: "Create a collection from OpenAPI specification",
        arguments: vec![
            PromptArgument {
                name: "openapi_spec",
                description: "OpenAPI specification URL or content",
                required: true,
            },
        ],
    },
];
```

### 2.2 工具调用示例

#### 发送请求

```json
// Request
{
    "jsonrpc": "2.0",
    "id": 2,
    "method": "tools/call",
    "params": {
        "name": "send_request",
        "arguments": {
            "method": "POST",
            "url": "https://api.example.com/users",
            "headers": {
                "Content-Type": "application/json",
                "Authorization": "Bearer {{token}}"
            },
            "body": "{\"name\":\"John\",\"email\":\"john@example.com\"}"
        }
    }
}

// Response
{
    "jsonrpc": "2.0",
    "id": 2,
    "result": {
        "content": [
            {
                "type": "text",
                "text": "Response:\nStatus: 201 Created\nTime: 245ms\nSize: 156 bytes\n\n{\n  \"id\": 123,\n  \"name\": \"John\",\n  \"email\": \"john@example.com\",\n  \"created_at\": \"2024-01-15T10:30:00Z\"\n}"
            }
        ],
        "isError": false
    }
}
```

#### 列出集合

```json
// Request
{
    "jsonrpc": "2.0",
    "id": 3,
    "method": "tools/call",
    "params": {
        "name": "list_collections",
        "arguments": {}
    }
}

// Response
{
    "jsonrpc": "2.0",
    "id": 3,
    "result": {
        "content": [
            {
                "type": "text",
                "text": "Collections:\n\n1. User API (12 requests)\n   - Authentication\n   - User Management\n   - Profile\n\n2. Product API (8 requests)\n   - Catalog\n   - Inventory\n\n3. Order API (15 requests)\n   - Checkout\n   - Payment\n   - Shipping"
            }
        ]
    }
}
```

#### 运行集合测试

```json
// Request
{
    "jsonrpc": "2.0",
    "id": 4,
    "method": "tools/call",
    "params": {
        "name": "run_collection",
        "arguments": {
            "collection_id": "uuid-1234",
            "environment_id": "uuid-env-1"
        }
    }
}

// Response
{
    "jsonrpc": "2.0",
    "id": 4,
    "result": {
        "content": [
            {
                "type": "text",
                "text": "Test Results:\n\nTotal: 12 requests\nPassed: 10\nFailed: 2\n\nFailures:\n1. DELETE /users/{id} - Expected 404, got 500\n2. POST /users - Schema validation failed\n\nDuration: 3.2 seconds"
            }
        ]
    }
}
```

### 2.3 MCP 传输层

#### Stdio 传输（默认）

```rust
// src/mcp/transport/stdio.rs

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::signal::ctrl_c;

pub struct StdioTransport {
    stdin: BufReader<tokio::io::Stdin>,
    stdout: tokio::io::Stdout,
}

impl StdioTransport {
    pub fn new() -> Self {
        Self {
            stdin: BufReader::new(tokio::io::stdin()),
            stdout: tokio::io::stdout(),
        }
    }
    
    pub async fn run(&mut self, server: &McpServer) -> Result<()> {
        let mut line = String::new();
        
        loop {
            // 读取 JSON-RPC 请求（每行一个 JSON）
            line.clear();
            let bytes_read = self.stdin.read_line(&mut line).await?;
            
            if bytes_read == 0 {
                break; // EOF
            }
            
            // 解析并处理请求
            let request: JsonRpcRequest = serde_json::from_str(&line)?;
            let response = server.handle_request(request).await?;
            
            // 发送响应
            let response_json = serde_json::to_string(&response)?;
            self.stdout.write_all(response_json.as_bytes()).await?;
            self.stdout.write_all(b"\n").await?;
            self.stdout.flush().await?;
        }
        
        Ok(())
    }
}
```

#### SSE 传输（可选）

```rust
// src/mcp/transport/sse.rs

use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter;

pub async fn run_sse_server(port: u16, server: Arc<McpServer>) -> Result<()> {
    let route = warp::post()
        .and(warp::path("mcp"))
        .and(warp::sse())
        .map(|sse: warp::sse::Sse| {
            let stream = server.create_event_stream();
            sse.reply(
                warp::sse::event("message")
                    .data(stream)
            )
        });
    
    warp::serve(route)
        .run(([127, 0, 0, 1], port))
        .await;
    
    Ok(())
}
```

---

## 3. 内部服务 API

### 3.1 HTTP 服务

```rust
// src/services/http.rs

use reqwest::{Client, RequestBuilder};
use std::time::Duration;

#[derive(Clone)]
pub struct HttpService {
    client: Client,
}

impl HttpService {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(10)
            .build()?;
        
        Ok(Self { client })
    }
    
    /// 发送 HTTP 请求
    pub async fn send_request(
        &self,
        request: &HttpRequest,
    ) -> Result<HttpResponse, HttpError> {
        let start = std::time::Instant::now();
        
        // 构建 reqwest 请求
        let mut req_builder = self.client
            .request(request.method.into(), &request.url);
        
        // 添加 headers
        for header in &request.headers {
            req_builder = req_builder.header(&header.name, &header.value);
        }
        
        // 添加 body
        if let Some(body) = &request.body {
            req_builder = req_builder.body(body.clone());
        }
        
        // 发送请求
        let response = req_builder
            .send()
            .await
            .map_err(HttpError::RequestFailed)?;
        
        let duration = start.elapsed();
        
        // 解析响应
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.bytes().await?;
        
        Ok(HttpResponse {
            status_code: status.as_u16(),
            status_text: status.canonical_reason().unwrap_or("Unknown").to_string(),
            headers: Self::convert_headers(headers),
            body: body.to_vec(),
            duration_ms: duration.as_millis() as u64,
            size: body.len() as u64,
        })
    }
    
    /// 流式响应（用于大文件下载）
    pub async fn send_request_stream(
        &self,
        request: &HttpRequest,
    ) -> Result<ResponseStream, HttpError> {
        // 实现...
    }
    
    /// WebSocket 连接
    pub async fn websocket_connect(
        &self,
        url: &str,
    ) -> Result<WebSocketConnection, HttpError> {
        // 实现...
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<Header>,
    pub body: Option<Vec<u8>>,
    pub query: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
    pub duration_ms: u64,
    pub size: u64,
}

impl HttpResponse {
    /// 解析为 JSON
    pub fn json(&self) -> Result<serde_json::Value> {
        Ok(serde_json::from_slice(&self.body)?)
    }
    
    /// 获取文本
    pub fn text(&self) -> Result<String> {
        Ok(String::from_utf8(self.body.clone())?)
    }
    
    /// 获取 Headers
    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.iter()
            .find(|h| h.name.eq_ignore_ascii_case(name))
            .map(|h| &h.value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl From<HttpMethod> for reqwest::Method {
    fn from(m: HttpMethod) -> Self {
        match m {
            HttpMethod::GET => reqwest::Method::GET,
            HttpMethod::POST => reqwest::Method::POST,
            HttpMethod::PUT => reqwest::Method::PUT,
            HttpMethod::DELETE => reqwest::Method::DELETE,
            HttpMethod::PATCH => reqwest::Method::PATCH,
            HttpMethod::HEAD => reqwest::Method::HEAD,
            HttpMethod::OPTIONS => reqwest::Method::OPTIONS,
        }
    }
}
```

### 3.2 请求执行器

```rust
// src/services/executor.rs

use std::sync::Arc;

pub struct RequestExecutor {
    http_service: Arc<HttpService>,
    script_engine: Arc<ScriptEngine>,
    storage: Arc<StorageService>,
}

impl RequestExecutor {
    pub async fn execute_request(
        &self,
        request: Request,
        environment: &Environment,
    ) -> Result<ExecutionResult, ExecutionError> {
        // 1. 创建脚本上下文
        let mut context = ScriptContext::new(
            environment.clone(),
            request.clone(),
        );
        
        // 2. 执行 Pre-request Script
        if let Some(script) = &request.pre_request_script {
            let result = self.script_engine
                .execute_pre_request(script, &mut context)
                .await?;
            
            // 应用脚本修改
            if let Some(modified_request) = result.modified_request {
                request = modified_request;
            }
            
            // 更新环境变量
            for (key, value) in result.modified_variables {
                self.storage.set_variable(&environment.id, &key, &value).await?;
                environment.variables.insert(key, value);
            }
        }
        
        // 3. 解析变量
        let resolved_request = self.resolve_variables(&request, environment)?;
        
        // 4. 发送 HTTP 请求
        let http_response = self.http_service
            .send_request(&resolved_request)
            .await?;
        
        // 5. 执行 Post-response Script
        let mut response_context = ScriptContext::with_response(
            environment.clone(),
            request.clone(),
            http_response.clone(),
        );
        
        let mut test_results = Vec::new();
        
        if let Some(script) = &request.post_response_script {
            let result = self.script_engine
                .execute_post_response(script, &mut response_context)
                .await?;
            
            test_results = result.test_results;
            
            // 更新环境变量
            for (key, value) in result.modified_variables {
                self.storage.set_variable(&environment.id, &key, &value).await?;
            }
        }
        
        // 6. 保存到历史
        self.storage.save_history(&request, &http_response).await?;
        
        Ok(ExecutionResult {
            response: http_response,
            test_results,
        })
    }
    
    /// 批量执行集合中的请求
    pub async fn execute_collection(
        &self,
        collection: &Collection,
        environment: &Environment,
        options: ExecutionOptions,
    ) -> Result<CollectionExecutionResult, ExecutionError> {
        let mut results = Vec::new();
        let mut passed = 0;
        let mut failed = 0;
        
        for request in &collection.requests {
            match self.execute_request(request.clone(), environment).await {
                Ok(result) => {
                    let all_passed = result.test_results.iter().all(|t| t.passed);
                    if all_passed {
                        passed += 1;
                    } else {
                        failed += 1;
                    }
                    results.push(result);
                }
                Err(e) => {
                    failed += 1;
                    if options.stop_on_failure {
                        return Err(e);
                    }
                }
            }
        }
        
        Ok(CollectionExecutionResult {
            total: collection.requests.len(),
            passed,
            failed,
            results,
        })
    }
    
    fn resolve_variables(
        &self,
        request: &Request,
        environment: &Environment,
    ) -> Result<HttpRequest, ExecutionError> {
        let re = regex::Regex::new(r"\{\{(\w+)\}\}")?;
        
        let resolve = |text: &str| -> String {
            re.replace_all(text, |caps: &regex::Captures| {
                let key = &caps[1];
                environment.get_variable(key)
                    .unwrap_or(&caps[0].to_string())
                    .clone()
            }).to_string()
        };
        
        Ok(HttpRequest {
            method: request.method,
            url: resolve(&request.url),
            headers: request.headers.iter()
                .map(|h| Header {
                    name: h.name.clone(),
                    value: resolve(&h.value),
                })
                .collect(),
            body: request.body.as_ref().map(|b| {
                match b {
                    RequestBody::Json(json) => {
                        resolve(&json.to_string()).into_bytes()
                    }
                    RequestBody::Raw(text) => resolve(text).into_bytes(),
                    _ => b.to_bytes(),
                }
            }),
            query: request.query.iter()
                .map(|(k, v)| (k.clone(), resolve(v)))
                .collect(),
        })
    }
}

pub struct ExecutionOptions {
    pub stop_on_failure: bool,
    pub delay_between_requests: Option<Duration>,
    pub parallel: bool,
}

pub struct ExecutionResult {
    pub response: HttpResponse,
    pub test_results: Vec<TestResult>,
}

pub struct CollectionExecutionResult {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub results: Vec<ExecutionResult>,
}
```

---

## 4. 数据持久化 API

### 4.1 数据库 Schema

```sql
-- migrations/001_initial.sql

-- Collections
CREATE TABLE collections (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    auth_config TEXT, -- JSON
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Folders
CREATE TABLE folders (
    id TEXT PRIMARY KEY,
    collection_id TEXT NOT NULL,
    parent_id TEXT,
    name TEXT NOT NULL,
    description TEXT,
    FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES folders(id) ON DELETE CASCADE
);

-- Requests
CREATE TABLE requests (
    id TEXT PRIMARY KEY,
    collection_id TEXT,
    folder_id TEXT,
    name TEXT NOT NULL,
    description TEXT,
    method TEXT NOT NULL,
    url TEXT NOT NULL,
    headers TEXT, -- JSON array
    query_params TEXT, -- JSON array
    body_mode TEXT,
    body_content TEXT, -- JSON based on mode
    auth_config TEXT, -- JSON
    pre_request_script TEXT,
    post_response_script TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE,
    FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE CASCADE
);

-- Environments
CREATE TABLE environments (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Environment Variables
CREATE TABLE environment_variables (
    id TEXT PRIMARY KEY,
    environment_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    is_secret INTEGER NOT NULL DEFAULT 0,
    enabled INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE,
    UNIQUE(environment_id, key)
);

-- Request History
CREATE TABLE request_history (
    id TEXT PRIMARY KEY,
    request_id TEXT,
    url TEXT NOT NULL,
    method TEXT NOT NULL,
    status_code INTEGER,
    duration_ms INTEGER,
    size INTEGER,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (request_id) REFERENCES requests(id) ON DELETE SET NULL
);

-- History Response Bodies (separate table for size management)
CREATE TABLE history_responses (
    history_id TEXT PRIMARY KEY,
    body BLOB,
    headers TEXT, -- JSON
    FOREIGN KEY (history_id) REFERENCES request_history(id) ON DELETE CASCADE
);

-- Indexes
CREATE INDEX idx_requests_collection ON requests(collection_id);
CREATE INDEX idx_requests_folder ON requests(folder_id);
CREATE INDEX idx_folders_collection ON folders(collection_id);
CREATE INDEX idx_folders_parent ON folders(parent_id);
CREATE INDEX idx_environment_vars_env ON environment_variables(environment_id);
CREATE INDEX idx_history_timestamp ON request_history(timestamp DESC);
```

### 4.2 存储 API

```rust
// src/services/storage.rs

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use sqlx::migrate::MigrateDatabase;

pub struct StorageService {
    pool: SqlitePool,
}

impl StorageService {
    /// 初始化存储服务
    pub async fn new(db_path: &str) -> Result<Self, StorageError> {
        // 创建数据库（如果不存在）
        if !Sqlite::database_exists(db_path).await? {
            Sqlite::create_database(db_path).await?;
        }
        
        // 连接配置
        let options = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)
            .busy_timeout(Duration::from_secs(30))
            .pragma("journal_mode", "WAL")
            .pragma("synchronous", "NORMAL");
        
        let pool = SqlitePool::connect_with(options).await?;
        
        // 运行迁移
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Self { pool })
    }
    
    // ========== Collections ==========
    
    pub async fn create_collection(
        &self,
        collection: &Collection,
    ) -> Result<(), StorageError> {
        sqlx::query!(
            r#"
            INSERT INTO collections (id, name, description, auth_config, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            collection.id,
            collection.name,
            collection.description,
            collection.auth.as_ref().map(|a| serde_json::to_string(a)),
            collection.created_at,
            collection.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn list_collections(&self) -> Result<Vec<Collection>, StorageError> {
        let rows = sqlx::query_as!(
            CollectionRow,
            "SELECT * FROM collections ORDER BY updated_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut collections = Vec::new();
        for row in rows {
            let folders = self.get_folders_for_collection(&row.id).await?;
            let requests = self.get_requests_for_collection(&row.id).await?;
            
            collections.push(Collection {
                id: row.id,
                name: row.name,
                description: row.description,
                auth: row.auth_config.and_then(|s| serde_json::from_str(&s).ok()),
                folders,
                requests,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(collections)
    }
    
    pub async fn get_collection(&self, id: &str) -> Result<Option<Collection>, StorageError> {
        let row = sqlx::query_as!(
            CollectionRow,
            "SELECT * FROM collections WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let folders = self.get_folders_for_collection(&row.id).await?;
                let requests = self.get_requests_for_collection(&row.id).await?;
                
                Ok(Some(Collection {
                    id: row.id,
                    name: row.name,
                    description: row.description,
                    auth: row.auth_config.and_then(|s| serde_json::from_str(&s).ok()),
                    folders,
                    requests,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }))
            }
            None => Ok(None),
        }
    }
    
    pub async fn update_collection(
        &self,
        collection: &Collection,
    ) -> Result<(), StorageError> {
        sqlx::query!(
            r#"
            UPDATE collections
            SET name = ?, description = ?, auth_config = ?, updated_at = ?
            WHERE id = ?
            "#,
            collection.name,
            collection.description,
            collection.auth.as_ref().map(|a| serde_json::to_string(a)),
            collection.updated_at,
            collection.id,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn delete_collection(&self, id: &str) -> Result<(), StorageError> {
        sqlx::query!("DELETE FROM collections WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    
    // ========== Requests ==========
    
    pub async fn save_request(&self, request: &Request) -> Result<(), StorageError> {
        sqlx::query!(
            r#"
            INSERT INTO requests (
                id, collection_id, folder_id, name, description, method, url,
                headers, query_params, body_mode, body_content, auth_config,
                pre_request_script, post_response_script, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                collection_id = excluded.collection_id,
                folder_id = excluded.folder_id,
                name = excluded.name,
                description = excluded.description,
                method = excluded.method,
                url = excluded.url,
                headers = excluded.headers,
                query_params = excluded.query_params,
                body_mode = excluded.body_mode,
                body_content = excluded.body_content,
                auth_config = excluded.auth_config,
                pre_request_script = excluded.pre_request_script,
                post_response_script = excluded.post_response_script,
                updated_at = excluded.updated_at
            "#,
            request.id,
            request.collection_id,
            request.folder_id,
            request.name,
            request.description,
            request.method.to_string(),
            request.url,
            serde_json::to_string(&request.headers),
            serde_json::to_string(&request.params),
            request.body.mode(),
            serde_json::to_string(&request.body),
            request.auth.as_ref().map(|a| serde_json::to_string(a)),
            request.pre_request_script,
            request.post_response_script,
            request.created_at,
            request.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_request(&self, id: &str) -> Result<Option<Request>, StorageError> {
        let row = sqlx::query_as!(
            RequestRow,
            "SELECT * FROM requests WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        // 转换为 Request...
    }
    
    pub async fn delete_request(&self, id: &str) -> Result<(), StorageError> {
        sqlx::query!("DELETE FROM requests WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    
    // ========== Environments ==========
    
    pub async fn create_environment(
        &self,
        env: &Environment,
    ) -> Result<(), StorageError> {
        // 创建环境
        sqlx::query!(
            "INSERT INTO environments (id, name, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
            env.id,
            env.name,
            env.is_active as i32,
            env.created_at,
            env.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        // 创建变量
        for var in &env.values {
            self.set_variable(&env.id, &var.key, &var.value).await?;
        }
        
        Ok(())
    }
    
    pub async fn list_environments(&self) -> Result<Vec<Environment>, StorageError> {
        let rows = sqlx::query_as!(
            EnvironmentRow,
            "SELECT * FROM environments ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut environments = Vec::new();
        for row in rows {
            let values = self.get_variables_for_environment(&row.id).await?;
            environments.push(Environment {
                id: row.id,
                name: row.name,
                is_active: row.is_active != 0,
                values,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(environments)
    }
    
    pub async fn get_active_environment(&self) -> Result<Option<Environment>, StorageError> {
        let row = sqlx::query_as!(
            EnvironmentRow,
            "SELECT * FROM environments WHERE is_active = 1 LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let values = self.get_variables_for_environment(&row.id).await?;
                Ok(Some(Environment {
                    id: row.id,
                    name: row.name,
                    is_active: true,
                    values,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }))
            }
            None => Ok(None),
        }
    }
    
    pub async fn set_active_environment(&self, id: &str) -> Result<(), StorageError> {
        // 取消所有激活状态
        sqlx::query!("UPDATE environments SET is_active = 0")
            .execute(&self.pool)
            .await?;
        
        // 激活指定环境
        sqlx::query!("UPDATE environments SET is_active = 1 WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn set_variable(
        &self,
        environment_id: &str,
        key: &str,
        value: &str,
    ) -> Result<(), StorageError> {
        sqlx::query!(
            r#"
            INSERT INTO environment_variables (id, environment_id, key, value)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(environment_id, key) DO UPDATE SET value = excluded.value
            "#,
            Uuid::new_v4().to_string(),
            environment_id,
            key,
            value,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn unset_variable(
        &self,
        environment_id: &str,
        key: &str,
    ) -> Result<(), StorageError> {
        sqlx::query!(
            "DELETE FROM environment_variables WHERE environment_id = ? AND key = ?",
            environment_id,
            key,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // ========== History ==========
    
    pub async fn save_history(
        &self,
        request: &Request,
        response: &HttpResponse,
    ) -> Result<(), StorageError> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp_millis();
        
        sqlx::query!(
            r#"
            INSERT INTO request_history (
                id, request_id, url, method, status_code, duration_ms, size, timestamp
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            id,
            request.id,
            request.url,
            request.method.to_string(),
            response.status_code as i32,
            response.duration_ms as i32,
            response.size as i32,
            now,
        )
        .execute(&self.pool)
        .await?;
        
        // 保存响应体（如果有大小限制）
        if response.body.len() < 10_000_000 { // 10MB 限制
            sqlx::query!(
                r#"
                INSERT INTO history_responses (history_id, body, headers)
                VALUES (?, ?, ?)
                "#,
                id,
                response.body,
                serde_json::to_string(&response.headers),
            )
            .execute(&self.pool)
            .await?;
        }
        
        // 清理旧历史（保留最近 1000 条）
        sqlx::query!(
            r#"
            DELETE FROM request_history
            WHERE id NOT IN (
                SELECT id FROM request_history ORDER BY timestamp DESC LIMIT 1000
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_history(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<HistoryEntry>, StorageError> {
        let rows = sqlx::query_as!(
            HistoryRow,
            r#"
            SELECT * FROM request_history
            ORDER BY timestamp DESC
            LIMIT ? OFFSET ?
            "#,
            limit as i32,
            offset as i32,
        )
        .fetch_all(&self.pool)
        .await?;
        
        // 转换为 HistoryEntry...
    }
    
    // ========== Import / Export ==========
    
    /// 导入 Postman Collection v2.1
    pub async fn import_postman_collection(
        &self,
        json: &serde_json::Value,
    ) -> Result<Collection, StorageError> {
        // 解析 Postman 格式
        let postman_col: PostmanCollectionV21 = serde_json::from_value(json.clone())?;
        
        // 转换为内部格式
        let collection = Collection::from_postman(postman_col);
        
        // 保存
        self.create_collection(&collection).await?;
        
        Ok(collection)
    }
    
    /// 导出为 Postman Collection v2.1
    pub async fn export_postman_collection(
        &self,
        id: &str,
    ) -> Result<serde_json::Value, StorageError> {
        let collection = self.get_collection(id).await?
            .ok_or(StorageError::NotFound)?;
        
        Ok(collection.to_postman())
    }
    
    // ========== Helper methods ==========
    
    async fn get_folders_for_collection(
        &self,
        collection_id: &str,
    ) -> Result<Vec<Folder>, StorageError> {
        let rows = sqlx::query_as!(
            FolderRow,
            "SELECT * FROM folders WHERE collection_id = ?",
            collection_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        // 构建树形结构...
    }
    
    async fn get_requests_for_collection(
        &self,
        collection_id: &str,
    ) -> Result<Vec<Request>, StorageError> {
        let rows = sqlx::query_as!(
            RequestRow,
            "SELECT * FROM requests WHERE collection_id = ? AND folder_id IS NULL",
            collection_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        // 转换为 Request...
    }
    
    async fn get_variables_for_environment(
        &self,
        environment_id: &str,
    ) -> Result<Vec<Variable>, StorageError> {
        let rows = sqlx::query_as!(
            VariableRow,
            "SELECT * FROM environment_variables WHERE environment_id = ?",
            environment_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows.into_iter().map(|row| Variable {
            key: row.key,
            value: row.value,
            enabled: row.enabled != 0,
            variable_type: if row.is_secret != 0 {
                VariableType::Secret
            } else {
                VariableType::Normal
            },
        }).collect())
    }
}
```

---

## 5. 事件系统

### 5.1 事件定义

```rust
// src/events.rs

use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum AppEvent {
    // Collection 事件
    CollectionCreated(Collection),
    CollectionUpdated(Collection),
    CollectionDeleted(String),
    
    // Request 事件
    RequestCreated(Request),
    RequestUpdated(Request),
    RequestDeleted(String),
    RequestExecuted(RequestId, HttpResponse),
    
    // Environment 事件
    EnvironmentCreated(Environment),
    EnvironmentUpdated(Environment),
    EnvironmentActivated(String),
    VariableChanged(String, String, String), // env_id, key, value
    
    // UI 事件
    RequestSelected(Request),
    CollectionSelected(Collection),
    
    // Sync 事件
    SyncStarted,
    SyncCompleted(SyncResult),
    SyncFailed(SyncError),
}

pub struct EventBus {
    sender: broadcast::Sender<AppEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self { sender }
    }
    
    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.sender.subscribe()
    }
    
    pub fn publish(&self, event: AppEvent) {
        let _ = self.sender.send(event);
    }
}

// 使用示例
#[async_trait]
pub trait EventListener: Send + Sync {
    async fn on_event(&self, event: &AppEvent);
}

pub struct UiUpdater {
    // UI 组件引用...
}

#[async_trait]
impl EventListener for UiUpdater {
    async fn on_event(&self, event: &AppEvent) {
        match event {
            AppEvent::RequestExecuted(id, response) => {
                // 更新 UI 显示响应
            }
            AppEvent::CollectionCreated(col) => {
                // 添加到侧边栏
            }
            _ => {}
        }
    }
}
```

---

本文档详细描述了 Postboy 的脚本 API、MCP 协议实现、内部服务和存储 API，为后续开发提供了清晰的接口规范。