# Postboy 功能路线图

参考 Postman 的功能，规划 Postboy 的完整功能实现路线。

## ✅ 已完成功能

### 核心请求功能
- [x] HTTP方法选择（GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS）
- [x] URL输入框
- [x] 发送HTTP请求
- [x] 查看响应结果
- [x] 响应状态显示（状态码、耗时）
- [x] 响应Body显示

### UI/UX
- [x] 侧边栏UI（集合、环境）
- [x] 请求/响应面板布局
- [x] 主题系统（暗色主题）
- [x] HTTP方法循环选择器
- [x] 统一的颜色系统

## 🚧 进行中（优先级排序）

### 高优先级（P0）

#### 1. Headers完全可编辑
**目标**: 让用户可以添加、编辑、删除HTTP请求头

**功能点**:
- [ ] 动态添加新Header行
- [ ] 编辑Header的Key和Value
- [ ] 删除Header行
- [ ] Header值的自动补全（常见Content-Type等）
- [ ] 保存Header配置到请求中

**实现方式**:
```rust
// 使用现有的HeaderEntry结构
// 实现InputState的onChange事件
// 将headers数据传递给HTTP请求
```

#### 2. Body数据集成
**目标**: 在HTTP请求中包含Body数据

**功能点**:
- [ ] 支持JSON格式Body
- [ ] 支持Text格式Body
- [ ] 根据Content-Type自动设置Body格式
- [ ] Body编辑器的语法验证
- [ ] 美化JSON格式化显示

**技术要点**:
```rust
// 从body_input读取内容
// 根据Content-Type序列化Body
// 传递给HttpService
```

#### 3. 错误处理增强
**目标**: 更友好的错误提示和处理

**功能点**:
- [ ] 网络错误提示（连接失败、超时等）
- [ ] 请求格式错误提示（无效URL、非法JSON等）
- [ ] 响应错误显示（4xx, 5xx状态码）
- [ ] 错误消息的颜色和样式区分
- [ ] 错误日志记录

#### 4. 加载动画效果
**目标**: 提升用户体验，显示请求进行中状态

**功能点**:
- [ ] Send按钮的Loading状态
- [ ] 请求发送中的Spinner动画
- [ ] 响应等待的进度提示
- [ ] 取消请求功能

### 中优先级（P1）

#### 5. 请求历史记录
**目标**: 保存和快速访问历史请求

**功能点**:
- [ ] 自动保存每次请求
- [ ] 历史记录列表显示
- [ ] 点击历史记录快速填充请求
- [ ] 清除历史记录
- [ ] 搜索历史记录
- [ ] 收藏重要请求

**数据结构**:
```rust
struct RequestHistory {
    id: Uuid,
    url: String,
    method: HttpMethod,
    headers: Vec<Header>,
    body: Option<String>,
    timestamp: DateTime<Utc>,
    name: Option<String>,
}
```

#### 6. URL参数编辑（Query Parameters）
**目标**: 方便编辑URL查询参数

**功能点**:
- [ ] 参数列表视图（Key-Value对）
- [ ] 添加/删除/编辑参数
- [ ] 参数自动编码
- [ ] 从URL解析现有参数
- [ ] 参数持久化

#### 7. 响应查看器增强
**目标**: 更好的响应内容展示

**功能点**:
- [ ] JSON语法高亮
- [ ] XML/HTML格式化显示
- [ ] 响应大小显示
- [ ] 响应时间统计
- [ ] 响应Headers查看
- [ ] 响应Body搜索
- [ ] 响应导出（复制、下载）
- [ ] 响应详情面板（Headers, Status, Time）

#### 8. 认证支持（Authentication）
**目标**: 支持常见认证方式

**功能点**:
- [ ] Basic Auth
- [ ] Bearer Token
- [ ] API Key
- [ ] OAuth 2.0
- [ ] 自定义Header认证
- [ ] 认证信息持久化（加密存储）

**实现方式**:
```rust
enum AuthType {
    None,
    Basic { username: String, password: String },
    Bearer { token: String },
    ApiKey { key: String, value: String, header: String },
}
```

### 低优先级（P2）

#### 9. 集合管理（Collections）
**目标**: 组织和管理多个请求

**功能点**:
- [ ] 创建集合
- [ ] 在集合中添加请求
- [ ] 集合文件夹组织
- [ ] 集合导入/导出（JSON格式）
- [ ] 集合搜索
- [ ] 集合描述和文档

**数据结构**:
```rust
struct Collection {
    id: Uuid,
    name: String,
    description: Option<String>,
    requests: Vec<Request>,
    folders: Vec<Folder>,
    created_at: DateTime<Utc>,
}

struct Folder {
    id: Uuid,
    name: String,
    requests: Vec<Request>,
}
```

#### 10. 环境变量管理
**目标**: 支持多环境配置（开发、测试、生产）

**功能点**:
- [ ] 创建环境
- [ ] 定义变量（Key-Value）
- [ ] 在请求中使用变量 {{variable_name}}
- [ ] 环境切换
- [ ] 环境变量自动补全
- [ ] 环境导入/导出

#### 11. 请求脚本与测试
**目标**: 自动化测试和请求前脚本

**功能点**:
- [ ] Pre-request脚本（请求前执行）
- [ ] Test脚本（响应后验证）
- [ ] 断言库（pm.test, pm.expect等）
- [ ] 脚本编辑器
- [ ] 脚本错误提示
- [ ] 测试结果展示

#### 12. Cookies管理
**目标**: 管理请求和响应的Cookies

**功能点**:
- [ ] Cookie Jar
- [ ] 自动存储响应Cookie
- [ ] 手动添加Cookie
- [ ] Cookie编辑器
- [ ] Cookie导入/导出

#### 13. 代理设置
**目标**: 支持通过代理发送请求

**功能点**:
- [ ] HTTP代理配置
- [ ] HTTPS代理配置
- [ ] SOCKS代理
- [ ] 代理认证
- [ ] 系统代理自动检测

#### 14. 证书配置
**目标**: 支持自定义SSL证书

**功能点**:
- [ ] CA证书配置
- [ ] 客户端证书配置
- [ ] 私钥配置
- [ ] SSL验证开关

#### 15. 批量请求/集合运行
**目标**: 顺序或并行执行多个请求

**功能点**:
- [ ] Collection Runner
- [ ] 顺序执行
- [ ] 并行执行
- [ ] 执行结果汇总
- [ ] 失败重试
- [ ] 执行延迟控制

#### 16. 代码生成
**目标**: 为请求生成多种语言的代码

**功能点**:
- [ ] JavaScript (Fetch, Axios, jQuery)
- [ ] Python (requests, http.client)
- [ ] cURL
- [ ] Java (OkHttp, HttpURLConnection)
- [ ] Go (net/http)
- [ ] PHP (cURL)
- [ ] 代码一键复制

## 🎯 功能优先级矩阵

| 功能 | 重要性 | 紧急性 | 优先级 |
|------|--------|--------|--------|
| Headers完全可编辑 | 高 | 高 | P0 |
| Body数据集成 | 高 | 高 | P0 |
| 错误处理增强 | 高 | 高 | P0 |
| 加载动画效果 | 中 | 高 | P0 |
| 请求历史记录 | 高 | 中 | P1 |
| URL参数编辑 | 中 | 中 | P1 |
| 响应查看器增强 | 中 | 中 | P1 |
| 认证支持 | 中 | 中 | P1 |
| 集合管理 | 中 | 低 | P2 |
| 环境变量管理 | 中 | 低 | P2 |
| 请求脚本与测试 | 低 | 低 | P2 |
| Cookies管理 | 低 | 低 | P2 |
| 代理设置 | 低 | 低 | P2 |
| 证书配置 | 低 | 低 | P2 |
| 批量请求 | 低 | 低 | P2 |
| 代码生成 | 低 | 低 | P2 |

## 📊 实现进度

### Phase 1: 核心功能完善（当前阶段）
- [x] HTTP方法选择
- [x] URL输入
- [x] 发送请求
- [ ] Headers完全可编辑
- [ ] Body数据集成
- [ ] 错误处理增强
- [ ] 加载动画

### Phase 2: 用户体验优化
- [ ] 请求历史记录
- [ ] URL参数编辑
- [ ] 响应查看器增强
- [ ] 认证支持
- [ ] 快捷键支持

### Phase 3: 高级功能
- [ ] 集合管理
- [ ] 环境变量管理
- [ ] 请求脚本与测试
- [ ] Cookies管理

### Phase 4: 企业级功能
- [ ] 团队协作（同步、分享）
- [ ] Mock服务器
- [ ] API文档生成
- [ ] 监控和告警
- [ ] CI/CD集成

## 🛠 技术债务和优化

### 性能优化
- [ ] 响应数据流式处理（大文件）
- [ ] 虚拟滚动（大量历史记录）
- [ ] 请求缓存机制
- [ ] UI渲染优化

### 代码质量
- [ ] 单元测试覆盖
- [ ] 集成测试
- [ ] 错误日志记录
- [ ] 代码文档完善

### 安全性
- [ ] 敏感数据加密存储
- [ ] 输入验证和清理
- [ ] SQL注入防护
- [ ] XSS防护

## 📝 下一步行动

### 立即开始（本周）
1. 实现Headers完全可编辑
2. 在HTTP请求中包含Body数据
3. 增强错误处理和显示

### 近期计划（本月）
4. 添加加载动画效果
5. 实现请求历史记录
6. 添加URL参数编辑

### 中期计划（下季度）
7. 响应查看器增强
8. 认证支持
9. 集合管理基础功能

## 🎨 UI/UX 改进建议

### 交互优化
- [ ] 键盘快捷键（Ctrl+Enter发送请求）
- [ ] 拖拽排序（集合项、Headers）
- [ ] 右键菜单（上下文操作）
- [ ] 撤销/重做功能
- [ ] 深色/浅色主题切换

### 可访问性
- [ ] 高对比度模式
- [ ] 字体大小调整
- [ ] 键盘导航完整支持
- [ ] 屏幕阅读器支持

---

**最后更新**: 2026-02-22
**当前版本**: v0.1.0
**目标版本**: v1.0.0
