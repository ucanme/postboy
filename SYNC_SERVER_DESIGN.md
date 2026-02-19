# Postboy 同步服务器设计文档

## 目录

1. [概述](#1-概述)
2. [架构设计](#2-架构设计)
3. [技术栈](#3-技术栈)
4. [API 设计](#4-api-设计)
5. [数据库设计](#5-数据库设计)
6. [WebSocket 服务](#6-websocket-服务)
7. [认证授权](#7-认证授权)
8. [冲突解决](#8-冲突解决)
9. [实时协作](#9-实时协作)
10. [部署架构](#10-部署架构)
11. [监控运维](#11-监控运维)

---

## 1. 概述

### 1.1 项目目标

Postboy 同步服务器是一个为 Postboy 客户端提供云端同步和团队协作功能的后端服务。

### 1.2 核心功能

- **数据同步** - 集合、请求、环境变量的云端存储与同步
- **团队协作** - 多用户实时编辑、权限管理
- **版本控制** - 历史版本管理和回滚
- **冲突解决** - 自动和手动冲突处理
- **实时通知** - WebSocket 推送和在线状态

### 1.3 系统约束

| 约束项 | 指标 |
|--------|------|
| 并发用户 | 10,000+ |
| API 响应时间 | P95 < 200ms |
| WebSocket 延迟 | < 100ms |
| 数据可用性 | 99.9% |
| 数据持久性 | 99.999% |

---

## 2. 架构设计

### 2.1 整体架构

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Client Layer                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐              │
│  │ Desktop  │  │ Desktop  │  │   Web    │  │  Mobile  │              │
│  │ Client 1 │  │ Client 2 │  │  Client  │  │  Client  │              │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘              │
│       │             │             │             │                      │
└───────┼─────────────┼─────────────┼─────────────┼──────────────────────┘
        │             │             │             │
        └─────────────┼─────────────┼─────────────┘
                      │             │
        ┌─────────────▼─────────────▼─────────────┐
        │            Load Balancer (HTTPS/WSS)    │
        │              (Nginx / HAProxy)          │
        └─────────────┬─────────────┬─────────────┘
                      │             │
        ┌─────────────▼─────────────▼─────────────┐
        │              API Gateway                │
        │           (Kong / AWS API)              │
        │  - Rate Limiting                        │
        │  - Authentication                       │
        │  - Request Routing                      │
        └─────────────┬─────────────┬─────────────┘
                      │             │
        ┌─────────────▼─┐   ┌──────▼──────┐
        │  HTTP API    │   │  WebSocket  │
        │   Server     │   │   Server    │
        └──────┬────────┘   └──────┬──────┘
               │                    │
        ┌──────▼────────────────────▼──────┐
        │        Application Layer           │
        │  ┌─────────┐  ┌──────────────┐   │
        │  │  Auth   │  │    Sync      │   │
        │  │ Service │  │   Service    │   │
        │  └─────────┘  └──────────────┘   │
        │  ┌─────────┐  ┌──────────────┐   │
        │  │ Collab  │  │   Version    │   │
        │  │ Service │  │   Service    │   │
        │  └─────────┘  └──────────────┘   │
        └───────────────┬───────────────────┘
                        │
        ┌───────────────▼───────────────────┐
        │          Data Layer                │
        │  ┌──────────┐  ┌──────────────┐   │
        │  │PostgreSQL│  │     Redis    │   │
        │  │ (Primary)│  │   (Cache)    │   │
        │  │          │  │              │   │
        │  │PostgreSQL│  │  Pub/Sub     │   │
        │  │ (Replica)│  │              │   │
        │  └──────────┘  └──────────────┘   │
        │  ┌──────────┐  ┌──────────────┐   │
        │  │   S3     │  │  Elasticsearch│  │
        │  │ (Files)  │  │   (Search)   │   │
        │  └──────────┘  └──────────────┘   │
        └─────────────────────────────────────┘
```

### 2.2 服务拆分

```
postboy-sync-server/
├── auth-service/          # 认证服务
│   ├── user management
│   ├── JWT tokens
│   ├── OAuth2
│   └── session management
│
├── sync-service/          # 同步服务
│   ├── change tracking
│   ├── push/pull
│   ├── conflict detection
│   └── version control
│
├── collab-service/        # 协作服务
│   ├── real-time editing
│   ├── presence
│   ├── cursors
│   └── permissions
│
├── gateway/               # API 网关
│   ├── routing
│   ├── rate limiting
│   ├── authentication
│   └── metrics
│
└── common/                # 共享代码
    ├── models
    ├── errors
    └── utils
```

---

## 3. 技术栈

### 3.1 后端框架

| 组件 | 技术选择 | 说明 |
|------|----------|------|
| 语言 | Rust 1.75+ | 高性能、内存安全 |
| Web 框架 | Actix-web 4.x | 高性能异步框架 |
| WebSocket | Actix-ws + Tokio | 异步 WebSocket |
| ORM | SeaORM | 异步 ORM |
| 数据库 | PostgreSQL 15+ | 关系型数据库 |
| 缓存 | Redis 7+ | 内存数据库 + Pub/Sub |
| 消息队列 | Redis Stream / RabbitMQ | 异步任务处理 |

### 3.2 基础设施

| 组件 | 技术选择 | 说明 |
|------|----------|------|
| 容器化 | Docker | 应用容器化 |
| 编排 | Kubernetes | 容器编排 |
| 反向代理 | Nginx | 负载均衡 |
| 监控 | Prometheus + Grafana | 指标监控 |
| 日志 | ELK Stack | 日志聚合分析 |
| 追踪 | Jaeger | 分布式追踪 |
| CDN | Cloudflare | 静态资源分发 |

### 3.3 第三方服务

| 服务 | 用途 |
|------|------|
| AWS S3 | 文件存储（导入/导出） |
| SendGrid | 邮件通知 |
| Sentry | 错误追踪 |
| Stripe | 支付（付费功能） |

---

## 4. API 设计

### 4.1 RESTful API

#### 认证相关

```http
# 用户注册
POST /api/v1/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePass123!",
  "name": "John Doe"
}

Response 201:
{
  "user_id": "uuid",
  "email": "user@example.com",
  "name": "John Doe",
  "created_at": 1704067200000
}

# 用户登录
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePass123!"
}

Response 200:
{
  "user": {
    "user_id": "uuid",
    "email": "user@example.com",
    "name": "John Doe",
    "avatar_url": "https://..."
  },
  "tokens": {
    "access": "eyJhbGc...",
    "refresh": "eyJhbGc...",
    "expires_in": 3600
  }
}

# 刷新 Token
POST /api/v1/auth/refresh
Content-Type: application/json

{
  "refresh_token": "eyJhbGc..."
}

# 登出
POST /api/v1/auth/logout
Authorization: Bearer <access_token>

# 密码重置
POST /api/v1/auth/forgot-password
{
  "email": "user@example.com"
}

POST /api/v1/auth/reset-password
{
  "token": "reset_token_from_email",
  "new_password": "NewSecurePass123!"
}
```

#### 同步相关

```http
# 拉取变更
GET /api/v1/sync/pull?device_id={device_id}&since={timestamp}
Authorization: Bearer <access_token>

Response 200:
{
  "timestamp": 1704067200000,
  "server_time": 1704067215000,
  "changes": [
    {
      "change_id": "uuid",
      "item_type": "collection",
      "item_id": "uuid",
      "operation": "update",
      "version": 5,
      "data": { ... },
      "created_by": "user_id",
      "created_at": 1704067200000
    }
  ],
  "has_more": false
}

# 推送变更
POST /api/v1/sync/push
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "device_id": "device_uuid",
  "changes": [
    {
      "change_id": "uuid",
      "item_type": "request",
      "item_id": "uuid",
      "operation": "create",
      "version": 1,
      "data": { ... },
      "timestamp": 1704067200000
    }
  ]
}

Response 200:
{
  "sync_id": "sync_uuid",
  "accepted": ["change_uuid_1", "change_uuid_2"],
  "rejected": [
    {
      "index": 2,
      "change_id": "change_uuid_3",
      "reason": "conflict",
      "conflict_with": {
        "version": 5,
        "data": { ... }
      }
    }
  ],
  "server_time": 1704067215000
}

# 确认同步完成
POST /api/v1/sync/acknowledge
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "sync_id": "sync_uuid"
}

# 获取同步状态
GET /api/v1/sync/status
Authorization: Bearer <access_token>

Response 200:
{
  "last_sync": 1704067200000,
  "pending_changes": 0,
  "conflicts": [],
  "devices": [
    {
      "device_id": "uuid",
      "name": "MacBook Pro",
      "last_seen": 1704067200000,
      "is_online": true
    }
  ]
}
```

#### 集合管理

```http
# 创建集合
POST /api/v1/collections
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "name": "My API Collection",
  "description": "API endpoints for testing",
  "icon": "folder"
}

# 获取集合列表
GET /api/v1/collections
Authorization: Bearer <access_token>

Query Params:
  - page: number (default: 1)
  - limit: number (default: 20, max: 100)
  - sort: "name" | "updated" | "created" (default: "updated")
  - order: "asc" | "desc" (default: "desc")
  - search: string (optional)

Response 200:
{
  "collections": [
    {
      "collection_id": "uuid",
      "name": "My API Collection",
      "description": "API endpoints for testing",
      "owner": {
        "user_id": "uuid",
        "name": "John Doe"
      },
      "is_shared": true,
      "members_count": 3,
      "requests_count": 15,
      "version": 5,
      "created_at": 1704067200000,
      "updated_at": 1704067215000
    }
  ],
  "total": 25,
  "page": 1,
  "pages": 2
}

# 获取集合详情
GET /api/v1/collections/{collection_id}
Authorization: Bearer <access_token>

# 更新集合
PUT /api/v1/collections/{collection_id}
Authorization: Bearer <access_token>

# 删除集合
DELETE /api/v1/collections/{collection_id}
Authorization: Bearer <access_token>

# 复制集合
POST /api/v1/collections/{collection_id}/duplicate
Authorization: Bearer <access_token>
```

#### 协作相关

```http
# 分享集合
POST /api/v1/collections/{collection_id}/share
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "emails": ["user1@example.com", "user2@example.com"],
  "role": "editor",
  "message": "Please review this API collection"
}

Response 200:
{
  "invited": [
    {
      "email": "user1@example.com",
      "invitation_id": "uuid"
    }
  ],
  "already_members": ["user2@example.com"],
  "failed": []
}

# 接受分享邀请
POST /api/v1/collections/{collection_id}/accept-share
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "invitation_token": "token_from_email"
}

# 获取集合成员
GET /api/v1/collections/{collection_id}/members
Authorization: Bearer <access_token>

Response 200:
{
  "members": [
    {
      "user": {
        "user_id": "uuid",
        "name": "John Doe",
        "email": "john@example.com",
        "avatar_url": "https://..."
      },
      "role": "owner",
      "permissions": {
        "can_edit": true,
        "can_delete": true,
        "can_share": true,
        "can_export": true
      },
      "joined_at": 1704067200000
    }
  ]
}

# 更新成员权限
PUT /api/v1/collections/{collection_id}/members/{user_id}
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "role": "viewer"
}

# 移除成员
DELETE /api/v1/collections/{collection_id}/members/{user_id}
Authorization: Bearer <access_token>

# 离开共享集合
DELETE /api/v1/collections/{collection_id}/leave
Authorization: Bearer <access_token>
```

#### 版本历史

```http
# 获取版本历史
GET /api/v1/collections/{collection_id}/history
Authorization: Bearer <access_token>

Query Params:
  - page: number
  - limit: number

Response 200:
{
  "versions": [
    {
      "version_id": "uuid",
      "version": 5,
      "description": "Updated user endpoint",
      "created_by": {
        "user_id": "uuid",
        "name": "John Doe"
      },
      "created_at": 1704067200000,
      "changes_summary": {
        "requests_added": 2,
        "requests_modified": 1,
        "requests_deleted": 0
      }
    }
  ],
  "total": 15
}

# 获取特定版本
GET /api/v1/collections/{collection_id}/history/{version_id}
Authorization: Bearer <access_token>

# 恢复到特定版本
POST /api/v1/collections/{collection_id}/restore
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "version_id": "uuid"
}

# 创建版本快照
POST /api/v1/collections/{collection_id}/snapshot
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "description": "Before major refactor"
}
```

#### 冲突解决

```http
# 获取冲突列表
GET /api/v1/sync/conflicts
Authorization: Bearer <access_token>

Response 200:
{
  "conflicts": [
    {
      "conflict_id": "uuid",
      "item_type": "request",
      "item_id": "uuid",
      "item_name": "GET /users",
      "collection_id": "uuid",
      "local_version": 3,
      "remote_version": 4,
      "local_value": { ... },
      "remote_value": { ... },
      "created_at": 1704067200000
    }
  ]
}

# 解决冲突
POST /api/v1/sync/conflicts/{conflict_id}/resolve
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "resolution": "local",  // "local" | "remote" | "merge"
  "merged_value": { ... }  // required if resolution is "merge"
}

# 批量解决冲突
POST /api/v1/sync/conflicts/resolve-batch
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "resolutions": [
    {
      "conflict_id": "uuid",
      "resolution": "local"
    },
    {
      "conflict_id": "uuid",
      "resolution": "remote"
    }
  ]
}
```

#### 环境变量

```http
# 获取环境变量列表
GET /api/v1/environments
Authorization: Bearer <access_token>

# 创建环境
POST /api/v1/environments
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "name": "Production",
  "variables": [
    {
      "key": "API_URL",
      "value": "https://api.example.com",
      "is_secret": false
    },
    {
      "key": "API_KEY",
      "value": "sk_live_...",
      "is_secret": true
    }
  ]
}

# 更新环境
PUT /api/v1/environments/{environment_id}
Authorization: Bearer <access_token>

# 删除环境
DELETE /api/v1/environments/{environment_id}
Authorization: Bearer <access_token>
```

#### 导入导出

```http
# 导出集合
GET /api/v1/collections/{collection_id}/export
Authorization: Bearer <access_token>

Query Params:
  - format: "postman_v2.1" | "openapi_3.0" | "insomnia"
  - include_secrets: boolean (default: false)

Response 200:
Content-Type: application/json
Content-Disposition: attachment; filename="collection.json"

{ ... }

# 导入集合
POST /api/v1/collections/import
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "format": "postman_v2.1",
  "data": { ... },
  "name": "Imported Collection"  // optional, overrides name in data
}

# 导入 URL
POST /api/v1/collections/import-from-url
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "url": "https://api.example.com/openapi.json",
  "format": "openapi_3.0"
}
```

### 4.2 WebSocket API

#### 连接

```javascript
// WebSocket 连接 URL
wss://api.postboy.app/ws/v1/sync?token=<access_token>&device_id=<device_id>

// 连接成功
{
  "type": "connected",
  "server_time": 1704067215000,
  "session_id": "session_uuid"
}

// 连接错误
{
  "type": "error",
  "code": "AUTH_FAILED",
  "message": "Invalid or expired token"
}
```

#### 客户端发送的消息

```javascript
// 订阅集合更新
{
  "type": "subscribe",
  "collection_id": "collection_uuid"
}

// 取消订阅
{
  "type": "unsubscribe",
  "collection_id": "collection_uuid"
}

// 广播操作（协作编辑）
{
  "type": "operation",
  "collection_id": "collection_uuid",
  "operation": {
    "type": "update",
    "item_type": "request",
    "item_id": "request_uuid",
    "path": ["headers", 0],
    "value": { "key": "Authorization", "value": "Bearer xxx" }
  }
}

// 更新光标位置
{
  "type": "cursor",
  "collection_id": "collection_uuid",
  "cursor": {
    "item_type": "request",
    "item_id": "request_uuid",
    "field": "body",
    "position": { "line": 5, "column": 10 }
  }
}

// 更新在线状态
{
  "type": "presence",
  "status": "online" | "away" | "offline"
}

// 心跳
{
  "type": "ping",
  "timestamp": 1704067215000
}
```

#### 服务器发送的消息

```javascript
// 用户加入会话
{
  "type": "user_joined",
  "session_id": "session_uuid",
  "user": {
    "user_id": "uuid",
    "name": "John Doe",
    "avatar_url": "https://..."
  },
  "color": "#FF5733",
  "timestamp": 1704067215000
}

// 用户离开会话
{
  "type": "user_left",
  "session_id": "session_uuid",
  "user_id": "uuid",
  "timestamp": 1704067215000
}

// 收到远程操作
{
  "type": "operation",
  "session_id": "session_uuid",
  "user_id": "uuid",
  "operation": {
    "type": "update",
    "item_type": "request",
    "item_id": "request_uuid",
    "path": ["headers", 0],
    "value": { "key": "Authorization", "value": "Bearer xxx" }
  },
  "timestamp": 1704067215000
}

// 光标位置更新
{
  "type": "cursor",
  "session_id": "session_uuid",
  "user": {
    "user_id": "uuid",
    "name": "John Doe",
    "color": "#FF5733"
  },
  "cursor": {
    "item_type": "request",
    "item_id": "request_uuid",
    "field": "body",
    "position": { "line": 5, "column": 10 }
  },
  "timestamp": 1704067215000
}

// 变更通知
{
  "type": "change",
  "collection_id": "collection_uuid",
  "change": {
    "change_id": "uuid",
    "item_type": "request",
    "item_id": "uuid",
    "operation": "create",
    "version": 1,
    "data": { ... },
    "created_by": {
      "user_id": "uuid",
      "name": "John Doe"
    }
  },
  "timestamp": 1704067215000
}

// 冲突检测
{
  "type": "conflict",
  "conflicts": [
    {
      "conflict_id": "uuid",
      "item_type": "request",
      "item_id": "uuid",
      "item_name": "GET /users",
      "local_version": 3,
      "remote_version": 4
    }
  ]
}

// 心跳响应
{
  "type": "pong",
  "timestamp": 1704067215000
}

// 在线用户列表
{
  "type": "presence",
  "collection_id": "collection_uuid",
  "users": [
    {
      "user_id": "uuid",
      "name": "John Doe",
      "color": "#FF5733",
      "status": "online",
      "cursor": { ... }
    }
  ]
}
```

---

## 5. 数据库设计

### 5.1 核心表结构

```sql
-- 用户表
CREATE TABLE users (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    avatar_url TEXT,
    bio TEXT,
    is_verified BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    last_login_at TIMESTAMPTZ
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created ON users(created_at DESC);

-- 设备表
CREATE TABLE devices (
    device_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    device_type VARCHAR(50), -- 'desktop', 'mobile', 'web'
    os_info VARCHAR(255),
    last_seen TIMESTAMPTZ DEFAULT NOW(),
    is_online BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, device_id)
);

CREATE INDEX idx_devices_user ON devices(user_id);
CREATE INDEX idx_devices_last_seen ON devices(last_seen DESC);

-- 集合表
CREATE TABLE collections (
    collection_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    icon VARCHAR(50),
    color VARCHAR(7), -- hex color
    is_public BOOLEAN DEFAULT FALSE,
    is_template BOOLEAN DEFAULT FALSE,
    version INTEGER DEFAULT 1,
    data JSONB NOT NULL, -- 完整的集合数据
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_collections_owner ON collections(owner_id);
CREATE INDEX idx_collections_updated ON collections(updated_at DESC);
CREATE INDEX idx_collections_public ON collections(is_public) WHERE is_public = TRUE;

-- 集合成员表
CREATE TABLE collection_members (
    member_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collection_id UUID NOT NULL REFERENCES collections(collection_id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL CHECK (role IN ('owner', 'admin', 'editor', 'viewer')),
    can_edit BOOLEAN DEFAULT FALSE,
    can_delete BOOLEAN DEFAULT FALSE,
    can_share BOOLEAN DEFAULT FALSE,
    can_export BOOLEAN DEFAULT FALSE,
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(collection_id, user_id)
);

CREATE INDEX idx_collection_members_collection ON collection_members(collection_id);
CREATE INDEX idx_collection_members_user ON collection_members(user_id);

-- 变更记录表
CREATE TABLE changes (
    change_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES devices(device_id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    collection_id UUID REFERENCES collections(collection_id) ON DELETE CASCADE,
    item_type VARCHAR(50) NOT NULL CHECK (item_type IN ('collection', 'request', 'folder', 'environment')),
    item_id UUID NOT NULL,
    operation VARCHAR(10) NOT NULL CHECK (operation IN ('create', 'update', 'delete')),
    version INTEGER NOT NULL,
    data JSONB NOT NULL,
    synced BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_changes_device ON changes(device_id, created_at DESC);
CREATE INDEX idx_changes_collection ON changes(collection_id, created_at DESC);
CREATE INDEX idx_changes_item ON changes(item_type, item_id);
CREATE INDEX idx_changes_synced ON changes(synced) WHERE synced = FALSE;

-- 冲突记录表
CREATE TABLE conflicts (
    conflict_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    collection_id UUID REFERENCES collections(collection_id) ON DELETE CASCADE,
    item_type VARCHAR(50) NOT NULL,
    item_id UUID NOT NULL,
    item_name VARCHAR(255),
    local_version INTEGER NOT NULL,
    remote_version INTEGER NOT NULL,
    local_value JSONB NOT NULL,
    remote_value JSONB NOT NULL,
    resolved BOOLEAN DEFAULT FALSE,
    resolution VARCHAR(20), -- 'local', 'remote', 'merge'
    resolved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_conflicts_user ON conflicts(user_id, resolved, created_at DESC);
CREATE INDEX idx_conflicts_collection ON conflicts(collection_id, resolved, created_at DESC);

-- 版本历史表
CREATE TABLE collection_versions (
    version_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collection_id UUID NOT NULL REFERENCES collections(collection_id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    description TEXT,
    data JSONB NOT NULL,
    created_by UUID NOT NULL REFERENCES users(user_id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(collection_id, version)
);

CREATE INDEX idx_versions_collection ON collection_versions(collection_id, version DESC);

-- 环境变量表
CREATE TABLE environments (
    environment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    is_default BOOLEAN DEFAULT FALSE,
    variables JSONB NOT NULL, -- array of {key, value, is_secret, enabled}
    version INTEGER DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_environments_user ON environments(user_id, updated_at DESC);

-- 分享邀请表
CREATE TABLE share_invitations (
    invitation_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collection_id UUID NOT NULL REFERENCES collections(collection_id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL CHECK (role IN ('admin', 'editor', 'viewer')),
    invited_by UUID NOT NULL REFERENCES users(user_id),
    message TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    accepted BOOLEAN DEFAULT FALSE,
    accepted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_invitations_collection ON share_invitations(collection_id, accepted);
CREATE INDEX idx_invitations_email ON share_invitations(email, expires_at);

-- 会话表（WebSocket）
CREATE TABLE sessions (
    session_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    device_id UUID REFERENCES devices(device_id) ON DELETE SET NULL,
    collection_id UUID REFERENCES collections(collection_id) ON DELETE CASCADE,
    status VARCHAR(20) DEFAULT 'online' CHECK (status IN ('online', 'away', 'offline')),
    cursor_data JSONB,
    connected_at TIMESTAMPTZ DEFAULT NOW(),
    last_ping TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_sessions_user ON sessions(user_id, last_ping DESC);
CREATE INDEX idx_sessions_collection ON sessions(collection_id, last_ping DESC);

-- 操作日志表（用于审计）
CREATE TABLE audit_logs (
    log_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(user_id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50) NOT NULL,
    resource_id UUID NOT NULL,
    details JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_user ON audit_logs(user_id, created_at DESC);
CREATE INDEX idx_audit_logs_resource ON audit_logs(resource_type, resource_id);
CREATE INDEX idx_audit_logs_created ON audit_logs(created_at DESC);

-- 配额表
CREATE TABLE user_quotas (
    user_id UUID PRIMARY KEY REFERENCES users(user_id) ON DELETE CASCADE,
    max_collections INTEGER DEFAULT 10,
    max_requests_per_collection INTEGER DEFAULT 100,
    max_storage_mb INTEGER DEFAULT 100,
    max_collaborators INTEGER DEFAULT 5,
    collections_count INTEGER DEFAULT 0,
    storage_used_mb INTEGER DEFAULT 0,
    plan_tier VARCHAR(20) DEFAULT 'free', -- 'free', 'pro', 'team', 'enterprise'
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 5.2 数据迁移策略

```sql
-- 迁移版本管理
CREATE TABLE schema_migrations (
    version BIGINT PRIMARY KEY,
    applied_at TIMESTAMPTZ DEFAULT NOW()
);
```

---

## 6. WebSocket 服务

### 6.1 连接管理

```rust
// src/websocket/server.rs

use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// WebSocket 会话
pub struct WsSession {
    /// 会话 ID
    session_id: String,
    
    /// 用户 ID
    user_id: String,
    
    /// 设备 ID
    device_id: String,
    
    /// 房间管理器
    rooms: Arc<RoomManager>,
    
    /// 当前订阅的房间
    subscribed_rooms: Vec<String>,
    
    /// 心跳间隔
    heartbeat: Instant,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                self.handle_text_message(text, ctx);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

impl WsSession {
    /// 处理文本消息
    fn handle_text_message(&mut self, text: String, ctx: &mut ws::WebsocketContext<Self>) {
        match serde_json::from_str::<ClientMessage>(&text) {
            Ok(msg) => match msg.r#type {
                MessageType::Subscribe => {
                    self.subscribe(msg.collection_id.unwrap(), ctx);
                }
                MessageType::Unsubscribe => {
                    self.unsubscribe(msg.collection_id.unwrap(), ctx);
                }
                MessageType::Operation => {
                    self.handle_operation(msg.operation.unwrap(), ctx);
                }
                MessageType::Cursor => {
                    self.handle_cursor(msg.cursor.unwrap(), ctx);
                }
                MessageType::Presence => {
                    self.update_presence(msg.status.unwrap(), ctx);
                }
                MessageType::Ping => {
                    self.respond_pong(ctx);
                }
            },
            Err(e) => {
                // 发送错误消息
                let error = ServerMessage {
                    r#type: "error".to_string(),
                    code: "INVALID_MESSAGE".to_string(),
                    message: e.to_string(),
                };
                ctx.text(serde_json::to_string(&error).unwrap());
            }
        }
    }
    
    /// 订阅房间
    fn subscribe(&mut self, collection_id: String, ctx: &mut ws::WebsocketContext<Self>) {
        // 加入房间
        self.rooms.join(collection_id.clone(), self.session_id.clone()).await;
        
        // 记录订阅
        self.subscribed_rooms.push(collection_id.clone());
        
        // 发送当前房间在线用户
        let users = self.rooms.get_users(&collection_id).await;
        let presence = ServerMessage::presence(collection_id, users);
        ctx.text(serde_json::to_string(&presence).unwrap());
        
        // 广播新用户加入
        let joined = ServerMessage::user_joined(self.session_id.clone(), self.user_info()).await;
        self.rooms.broadcast(&collection_id, joined, Some(&self.session_id)).await;
    }
    
    /// 取消订阅
    fn unsubscribe(&mut self, collection_id: String, ctx: &mut ws::WebsocketContext<Self>) {
        self.rooms.leave(&collection_id, &self.session_id).await;
        self.subscribed_rooms.retain(|id| id != &collection_id);
        
        // 广播用户离开
        let left = ServerMessage::user_left(self.session_id.clone(), self.user_id.clone());
        self.rooms.broadcast(&collection_id, left, None).await;
    }
    
    /// 处理操作
    fn handle_operation(&mut self, operation: Operation, ctx: &mut ws::WebsocketContext<Self>) {
        // 保存操作到数据库
        // ...
        
        // 广播给房间内其他用户
        let msg = ServerMessage::operation(
            self.session_id.clone(),
            self.user_id.clone(),
            operation,
        );
        
        if let Some(collection_id) = &operation.collection_id {
            self.rooms.broadcast(collection_id, msg, Some(&self.session_id)).await;
        }
    }
    
    /// 处理光标更新
    fn handle_cursor(&mut self, cursor: Cursor, ctx: &mut ws::WebsocketContext<Self>) {
        // 更新数据库中的光标位置
        // ...
        
        // 广播给房间内其他用户
        let msg = ServerMessage::cursor(
            self.session_id.clone(),
            self.user_info(),
            cursor,
        );
        
        if let Some(collection_id) = &cursor.collection_id {
            self.rooms.broadcast(collection_id, msg, Some(&self.session_id)).await;
        }
    }
    
    /// 更新在线状态
    fn update_presence(&mut self, status: String, ctx: &mut ws::WebsocketContext<Self>) {
        // 更新数据库
        // ...
        
        // 广播状态变更
        for collection_id in &self.subscribed_rooms {
            let msg = ServerMessage::presence(
                collection_id.clone(),
                self.rooms.get_users(collection_id).await,
            );
            ctx.text(serde_json::to_string(&msg).unwrap());
        }
    }
}

/// 房间管理器
pub struct RoomManager {
    /// 房间 -> 会话列表
    rooms: Arc<RwLock<HashMap<String, Room>>>,
}

#[derive(Clone)]
pub struct Room {
    pub collection_id: String,
    pub sessions: HashMap<String, SessionInfo>,
}

#[derive(Clone)]
pub struct SessionInfo {
    pub session_id: String,
    pub user_id: String,
    pub user_name: String,
    pub color: String,
    pub cursor: Option<Cursor>,
    pub status: String,
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// 加入房间
    pub async fn join(&self, collection_id: String, session_id: String) {
        let mut rooms = self.rooms.write().await;
        let room = rooms.entry(collection_id.clone()).or_insert_with(|| Room {
            collection_id,
            sessions: HashMap::new(),
        });
        // 添加会话
    }
    
    /// 离开房间
    pub async fn leave(&self, collection_id: &str, session_id: &str) {
        let mut rooms = self.rooms.write().await;
        if let Some(room) = rooms.get_mut(collection_id) {
            room.sessions.remove(session_id);
            // 如果房间为空，删除房间
            if room.sessions.is_empty() {
                rooms.remove(collection_id);
            }
        }
    }
    
    /// 广播消息到房间
    pub async fn broadcast(&self, collection_id: &str, message: ServerMessage, exclude: Option<&str>) {
        let rooms = self.rooms.read().await;
        if let Some(room) = rooms.get(collection_id) {
            let text = serde_json::to_string(&message).unwrap();
            for (session_id, _) in room.sessions.iter() {
                if let Some(exclude) = exclude {
                    if session_id == exclude {
                        continue;
                    }
                }
                // 发送消息到会话
                // ...
            }
        }
    }
    
    /// 获取房间用户列表
    pub async fn get_users(&self, collection_id: &str) -> Vec<UserInfo> {
        let rooms = self.rooms.read().await;
        if let Some(room) = rooms.get(collection_id) {
            room.sessions.values()
                .map(|s| UserInfo {
                    user_id: s.user_id.clone(),
                    name: s.user_name.clone(),
                    color: s.color.clone(),
                    status: s.status.clone(),
                    cursor: s.cursor.clone(),
                })
                .collect()
        } else {
            vec![]
        }
    }
}

/// 消息类型
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "subscribe")]
    Subscribe { collection_id: Option<String> },
    
    #[serde(rename = "unsubscribe")]
    Unsubscribe { collection_id: Option<String> },
    
    #[serde(rename = "operation")]
    Operation { operation: Option<Operation> },
    
    #[serde(rename = "cursor")]
    Cursor { cursor: Option<Cursor> },
    
    #[serde(rename = "presence")]
    Presence { status: Option<String> },
    
    #[serde(rename = "ping")]
    Ping,
}

#[derive(Debug, Serialize, Clone)]
pub struct ServerMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Operation {
    pub collection_id: String,
    #[serde(rename = "type")]
    pub op_type: String,
    pub item_type: String,
    pub item_id: String,
    pub path: Vec<String>,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cursor {
    pub collection_id: String,
    pub item_type: String,
    pub item_id: String,
    pub field: String,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub user_id: String,
    pub name: String,
    pub color: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
}
```

### 6.2 心跳与重连

```rust
// 心跳间隔
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

/// 心跳任务
fn start_heartbeat(ctx: &mut ws::WebsocketContext<WsSession>) {
    ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
        if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
            ctx.stop();
            return;
        }
        ctx.ping(b"");
    });
}

/// 客户端重连策略
// 客户端应实现指数退避重连
// 1s, 2s, 4s, 8s, 16s, 32s, 60s (最大)
```

---

## 7. 认证授权

### 7.1 JWT 实现

```rust
// src/auth/jwt.rs

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};

const JWT_SECRET: &[u8] = b"your-secret-key";
const ACCESS_TOKEN_EXPIRES: i64 = 3600; // 1 hour
const REFRESH_TOKEN_EXPIRES: i64 = 2592000; // 30 days

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // user_id
    pub name: String,
    pub email: String,
    pub exp: i64,           // 过期时间
    pub iat: i64,           // 签发时间
    pub jti: String,        // JWT ID
}

impl Claims {
    pub fn new(user: &User) -> Self {
        let now = Utc::now();
        let exp = now + Duration::seconds(ACCESS_TOKEN_EXPIRES);
        
        Self {
            sub: user.user_id.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        }
    }
}

pub fn encode_access_token(user: &User) -> Result<String, AuthError> {
    let claims = Claims::new(user);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    ).map_err(AuthError::JwtEncoding)
}

pub fn decode_access_token(token: &str) -> Result<Claims, AuthError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(AuthError::JwtDecoding)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String,        // user_id
    pub token_id: String,   // refresh token ID
    pub exp: i64,
    pub iat: i64,
}

pub fn encode_refresh_token(user_id: &str) -> Result<(String, String), AuthError> {
    let token_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let exp = now + Duration::seconds(REFRESH_TOKEN_EXPIRES);
    
    let claims = RefreshTokenClaims {
        sub: user_id.to_string(),
        token_id: token_id.clone(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    ).map_err(AuthError::JwtEncoding)?;
    
    // 存储 refresh token 到数据库
    // ...
    
    Ok((token, token_id))
}
```

### 7.2 OAuth2 集成

```rust
// src/auth/oauth.rs

use oauth2::{
    AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, Scope, TokenResponse, TokenUrl, AuthUrl,
    basic::BasicClient, reqwest::async_http_client,
};

pub struct OAuthConfig {
    pub github: Option<OAuthProvider>,
    pub google: Option<OAuthProvider>,
    pub microsoft: Option<OAuthProvider>,
}

pub struct OAuthProvider {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

impl OAuthProvider {
    pub fn to_client(&self, auth_url: &str, token_url: &str) -> BasicClient {
        BasicClient::new(
            ClientId::new(self.client_id.clone()),
            Some(ClientSecret::new(self.client_secret.clone())),
            AuthUrl::new(auth_url.to_string()).unwrap(),
            Some(TokenUrl::new(token_url.to_string()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(self.redirect_url.clone()).unwrap())
    }
}

/// GitHub OAuth
pub async fn github_login(code: String, config: &OAuthConfig) -> Result<User, AuthError> {
    let provider = config.github.as_ref().ok_or(AuthError::OAuthNotConfigured)?;
    
    let client = provider.to_client(
        "https://github.com/login/oauth/authorize",
        "https://github.com/login/oauth/access_token",
    );
    
    // 交换 code 获取 token
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await?;
    
    // 获取用户信息
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token_result.access_token().secret()))
        .header("User-Agent", "Postboy")
        .send()
        .await?;
    
    let github_user: GithubUser = response.json().await?;
    
    // 查找或创建用户
    let user = find_or_create_user_from_github(github_user).await?;
    
    Ok(user)
}

#[derive(Debug, Deserialize)]
struct GithubUser {
    id: i64,
    login: String,
    name: Option<String>,
    email: Option<String>,
    avatar_url: String,
}
```

---

## 8. 冲突解决

### 8.1 冲突检测

```rust
// src/sync/conflict.rs

#[derive(Debug, Clone)]
pub struct ConflictDetector {
    db: Arc<Db>,
}

impl ConflictDetector {
    pub async fn detect_conflicts(
        &self,
        user_id: &str,
        changes: Vec<Change>,
    ) -> Result<Vec<Conflict>, SyncError> {
        let mut conflicts = Vec::new();
        
        for change in &changes {
            // 获取服务器端最新版本
            let server_item = self.db
                .get_item(&change.item_type, &change.item_id)
                .await?;
            
            if let Some(server_item) = server_item {
                // 检查版本是否冲突
                if server_item.version > change.version {
                    // 获取客户端版本
                    let client_item = self.db
                        .get_item_version(&change.item_type, &change.item_id, change.version)
                        .await?;
                    
                    if let Some(client_item) = client_item {
                        conflicts.push(Conflict {
                            conflict_id: Uuid::new_v4().to_string(),
                            user_id: user_id.to_string(),
                            item_type: change.item_type.clone(),
                            item_id: change.item_id.clone(),
                            item_name: server_item.name.clone(),
                            local_version: change.version,
                            remote_version: server_item.version,
                            local_value: client_item.data,
                            remote_value: server_item.data,
                        });
                    }
                }
            }
        }
        
        Ok(conflicts)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Conflict {
    pub conflict_id: String,
    pub user_id: String,
    pub item_type: String,
    pub item_id: String,
    pub item_name: String,
    pub local_version: i64,
    pub remote_version: i64,
    pub local_value: serde_json::Value,
    pub remote_value: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum ConflictResolution {
    Local,
    Remote,
    Merge(serde_json::Value),
}

impl Conflict {
    pub async fn resolve(
        &self,
        resolution: &ConflictResolution,
        db: &Db,
    ) -> Result<(), SyncError> {
        match resolution {
            ConflictResolution::Local => {
                // 推送本地版本到服务器
                db.update_item(
                    &self.item_type,
                    &self.item_id,
                    &self.local_value,
                    self.remote_version,
                ).await?;
            }
            ConflictResolution::Remote => {
                // 接受远程版本，更新本地
                // 客户端会在下次 pull 时获取
            }
            ConflictResolution::Merge(merged_value) => {
                // 使用合并后的值
                db.update_item(
                    &self.item_type,
                    &self.item_id,
                    merged_value,
                    self.remote_version + 1,
                ).await?;
            }
        }
        
        // 标记冲突已解决
        db.mark_conflict_resolved(&self.conflict_id).await?;
        
        Ok(())
    }
}
```

### 8.2 自动合并策略

```rust
// src/sync/merge.rs

pub trait MergeStrategy {
    fn can_merge(&self, local: &serde_json::Value, remote: &serde_json::Value) -> bool;
    fn merge(&self, local: &serde_json::Value, remote: &serde_json::Value) 
        -> Result<serde_json::Value, MergeError>;
}

/// 递归合并 JSON 对象
pub struct DeepMergeStrategy;

impl MergeStrategy for DeepMergeStrategy {
    fn can_merge(&self, local: &serde_json::Value, remote: &serde_json::Value) -> bool {
        // 检查是否可以安全合并
        match (local, remote) {
            (Value::Object(_), Value::Object(_)) => true,
            (Value::Array(a), Value::Array(b)) => {
                // 数组只在索引不冲突时合并
                a.len() == b.len()
            }
            _ => false,
        }
    }
    
    fn merge(&self, local: &Value, remote: &Value) -> Result<Value, MergeError> {
        match (local, remote) {
            (Value::Object(a), Value::Object(b)) => {
                let mut result = a.clone();
                
                for (key, b_value) in b {
                    if let Some(a_value) = a.get(key) {
                        // 递归合并
                        result.insert(key.clone(), self.merge(a_value, b_value)?);
                    } else {
                        // 新增字段
                        result.insert(key.clone(), b_value.clone());
                    }
                }
                
                Ok(Value::Object(result))
            }
            (Value::Array(a), Value::Array(b)) => {
                // 数组按索引合并
                let mut result = Vec::new();
                
                for (i, (a_item, b_item)) in a.iter().zip(b.iter()).enumerate() {
                    if a_item == b_item {
                        result.push(a_item.clone());
                    } else if self.can_merge(a_item, b_item) {
                        result.push(self.merge(a_item, b_item)?);
                    } else {
                        // 冲突，保留两个版本
                        result.push(json!({
                            "_conflict": true,
                            "_local": a_item,
                            "_remote": b_item,
                        }));
                    }
                }
                
                // 处理长度不一致
                if b.len() > a.len() {
                    for item in &b[a.len()..] {
                        result.push(item.clone());
                    }
                }
                
                Ok(Value::Array(result))
            }
            _ => Err(MergeError::CannotMerge),
        }
    }
}

/// OT (Operational Transformation) 合并策略
/// 用于实时协作场景
pub struct OtMergeStrategy;

impl OtMergeStrategy {
    /// 转换操作
    pub fn transform(op1: &Operation, op2: &Operation) -> Result<(Operation, Operation), OtError> {
        match (&op1.op_type, &op2.op_type) {
            ("insert", "insert") => Self::transform_insert_insert(op1, op2),
            ("insert", "delete") => Self::transform_insert_delete(op1, op2),
            ("delete", "insert") => Self::transform_delete_insert(op1, op2),
            ("delete", "delete") => Self::transform_delete_delete(op1, op2),
            _ => Err(OtError::IncompatibleTypes),
        }
    }
    
    fn transform_insert_insert(op1: &Operation, op2: &Operation) -> Result<(Operation, Operation), OtError> {
        // 两个插入操作
        // 基于位置调整索引
        let pos1 = op1.position;
        let pos2 = op2.position;
        
        let op1_transformed = if pos2 <= pos1 {
            Operation {
                position: pos1 + op2.length,
                ..op1.clone()
            }
        } else {
            op1.clone()
        };
        
        let op2_transformed = if pos1 <= pos2 {
            Operation {
                position: pos2 + op1.length,
                ..op2.clone()
            }
        } else {
            op2.clone()
        };
        
        Ok((op1_transformed, op2_transformed))
    }
    
    // ... 其他转换方法
}
```

---

## 9. 实时协作

### 9.1 操作转换

```rust
// src/collab/ot.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CollabOperation {
    #[serde(rename = "insert")]
    Insert {
        path: Vec<String>,
        position: usize,
        value: serde_json::Value,
    },
    
    #[serde(rename = "delete")]
    Delete {
        path: Vec<String>,
        position: usize,
        length: usize,
    },
    
    #[serde(rename = "replace")]
    Replace {
        path: Vec<String>,
        old_value: serde_json::Value,
        new_value: serde_json::Value,
    },
    
    #[serde(rename = "move")]
    Move {
        from_path: Vec<String>,
        to_path: Vec<String>,
    },
}

/// OT 引擎
pub struct OtEngine {
    document: Arc<RwLock<CollabDocument>>,
}

impl OtEngine {
    pub async fn apply_remote_operation(
        &self,
        op: CollabOperation,
        remote_revision: i64,
    ) -> Result<CollabDocument, OtError> {
        let mut doc = self.document.write().await;
        
        // 获取本地待发送的操作
        let pending_ops = doc.get_pending_operations();
        
        // 转换操作
        let transformed_ops = Self::transform_operation(op, pending_ops)?;
        
        // 应用转换后的操作
        for op in transformed_ops {
            doc.apply_operation(op)?;
        }
        
        // 更新版本
        doc.set_revision(remote_revision);
        
        Ok(doc.clone())
    }
    
    fn transform_operation(
        op: CollabOperation,
        against: Vec<PendingOperation>,
    ) -> Result<Vec<CollabOperation>, OtError> {
        let mut result = vec![op];
        
        for pending in against {
            result = Self::transform_pair(&result[0], &pending.op)?;
        }
        
        Ok(result)
    }
    
    fn transform_pair(
        op1: &CollabOperation,
        op2: &CollabOperation,
    ) -> Result<Vec<CollabOperation>, OtError> {
        match (op1, op2) {
            (CollabOperation::Insert { path: p1, position: pos1, value: v1 },
             CollabOperation::Insert { path: p2, position: pos2, value: _ }) => {
                if p1 == p2 {
                    // 同一路径的插入
                    let new_pos1 = if pos2 <= pos1 { pos1 + 1 } else { *pos1 };
                    return Ok(vec![
                        CollabOperation::Insert {
                            path: p1.clone(),
                            position: new_pos1,
                            value: v1.clone(),
                        }
                    ]);
                }
            }
            // 其他组合...
            _ => {}
        }
        
        Ok(vec![op1.clone()])
    }
}
```

### 9.2 CRDT 实现

```rust
// src/collab/crdt.rs

use serde::{Deserialize, Serialize};

/// RGA (Replicated Growable Array)
/// 用于列表类型的数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rga<T> {
    atoms: Vec<RgaAtom<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RgaAtom<T> {
    id: (String, u64),      // (node_id, counter)
    value: Option<T>,
    tombstone: bool,
}

impl<T: Clone + PartialEq> Rga<T> {
    pub fn new() -> Self {
        Self { atoms: vec![] }
    }
    
    pub fn insert(&mut self, node_id: &str, after_id: Option<(String, u64)>, value: T) {
        let id = (node_id.to_string(), self.next_counter());
        
        let atom = RgaAtom {
            id,
            value: Some(value),
            tombstone: false,
        };
        
        if let Some(after) = after_id {
            // 找到插入位置
            let pos = self.atoms.iter()
                .position(|a| a.id == after)
                .unwrap_or(self.atoms.len());
            
            // 找到下一个可见位置
            let insert_pos = self.atoms[pos..]
                .iter()
                .position(|a| !a.tombstone)
                .map(|p| pos + p + 1)
                .unwrap_or(self.atoms.len());
            
            self.atoms.insert(insert_pos, atom);
        } else {
            self.atoms.insert(0, atom);
        }
    }
    
    pub fn delete(&mut self, node_id: &str, counter: u64) {
        if let Some(atom) = self.atoms.iter_mut()
            .find(|a| a.id == (node_id.to_string(), counter))
        {
            atom.tombstone = true;
            atom.value = None;
        }
    }
    
    pub fn merge(&mut self, other: Rga<T>) {
        // 合并两个 RGA
        for atom in other.atoms {
            if !self.atoms.iter().any(|a| a.id == atom.id) {
                // 新原子，插入
                let pos = self.find_position_for(&atom);
                self.atoms.insert(pos, atom);
            }
        }
    }
    
    fn find_position_for(&self, atom: &RgaAtom<T>) -> usize {
        // 基于 ID 和因果关系找到正确位置
        // 实现略...
        self.atoms.len()
    }
    
    pub fn to_vec(&self) -> Vec<T> {
        self.atoms.iter()
            .filter_map(|a| a.value.clone())
            .collect()
    }
    
    fn next_counter(&self) -> u64 {
        self.atoms.len() as u64
    }
}

/// LWW-Element-Set
/// 用于集合类型的数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LwwSet<T> {
    adds: Vec<LwwElement<T>>,
    removes: Vec<LwwElement<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LwwElement<T> {
    value: T,
    timestamp: i64,
    node_id: String,
}

impl<T: Clone + PartialEq + Eq + std::hash::Hash> LwwSet<T> {
    pub fn new() -> Self {
        Self {
            adds: vec![],
            removes: vec![],
        }
    }
    
    pub fn add(&mut self, value: T, timestamp: i64, node_id: &str) {
        let element = LwwElement {
            value,
            timestamp,
            node_id: node_id.to_string(),
        };
        
        self.adds.push(element);
    }
    
    pub fn remove(&mut self, value: T, timestamp: i64, node_id: &str) {
        let element = LwwElement {
            value,
            timestamp,
            node_id: node_id.to_string(),
        };
        
        self.removes.push(element);
    }
    
    pub fn contains(&self, value: &T) -> bool {
        // 获取最新的 add 和 remove
        let latest_add = self.adds.iter()
            .filter(|e| &e.value == value)
            .max_by_key(|e| e.timestamp);
        
        let latest_remove = self.removes.iter()
            .filter(|e| &e.value == value)
            .max_by_key(|e| e.timestamp);
        
        match (latest_add, latest_remove) {
            (Some(add), Some(remove)) => add.timestamp > remove.timestamp,
            (Some(_), None) => true,
            _ => false,
        }
    }
    
    pub fn to_set(&self) -> std::collections::HashSet<T> {
        self.adds.iter()
            .filter(|add| {
                if let Some(remove) = self.removes.iter()
                    .find(|r| r.value == add.value)
                {
                    add.timestamp > remove.timestamp
                } else {
                    true
                }
            })
            .map(|e| e.value.clone())
            .collect()
    }
    
    pub fn merge(&mut self, other: LwwSet<T>) {
        self.adds.extend(other.adds);
        self.removes.extend(other.removes);
        
        // 清理旧元素
        self.cleanup();
    }
    
    fn cleanup(&mut self) {
        // 移除被覆盖的元素
        self.adds.sort_by_key(|e| e.timestamp);
        self.adds.dedup_by_key(|e| e.value.clone());
        
        self.removes.sort_by_key(|e| e.timestamp);
        self.removes.dedup_by_key(|e| e.value.clone());
    }
}
```

---

## 10. 部署架构

### 10.1 Kubernetes 配置

```yaml
# k8s/configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: postboy-sync-config
data:
  DATABASE_URL: "postgresql://user:pass@postgres-primary:5432/postboy"
  REDIS_URL: "redis://redis:6379"
  JWT_SECRET: "your-secret-key"
  AWS_REGION: "us-east-1"
  S3_BUCKET: "postboy-sync-storage"
---
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: postboy-sync-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: postboy-sync-api
  template:
    metadata:
      labels:
        app: postboy-sync-api
    spec:
      containers:
      - name: api
        image: postboy/sync-api:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            configMapKeyRef:
              name: postboy-sync-config
              key: DATABASE_URL
        - name: REDIS_URL
          valueFrom:
            configMapKeyRef:
              name: postboy-sync-config
              key: REDIS_URL
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: postboy-sync-api
spec:
  selector:
    app: postboy-sync-api
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP
---
# k8s/websocket-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: postboy-sync-ws
spec:
  replicas: 2
  selector:
    matchLabels:
      app: postboy-sync-ws
  template:
    metadata:
      labels:
        app: postboy-sync-ws
    spec:
      containers:
      - name: ws
        image: postboy/sync-ws:latest
        ports:
        - containerPort: 8081
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "256Mi"
            cpu: "200m"
---
apiVersion: v1
kind: Service
metadata:
  name: postboy-sync-ws
spec:
  selector:
    app: postboy-sync-ws
  ports:
  - port: 80
    targetPort: 8081
  type: ClusterIP
---
# k8s/ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: postboy-sync-ingress
  annotations:
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
    nginx.ingress.kubernetes.io/websocket-services: "postboy-sync-ws"
spec:
  tls:
  - hosts:
    - api.postboy.app
    secretName: postboy-sync-tls
  rules:
  - host: api.postboy.app
    http:
      paths:
      - path: /api
        pathType: Prefix
        backend:
          service:
            name: postboy-sync-api
            port:
              number: 80
      - path: /ws
        pathType: Prefix
        backend:
          service:
            name: postboy-sync-ws
            port:
              number: 80
```

### 10.2 Docker Compose（开发环境）

```yaml
# docker-compose.yml
version: '3.8'

services:
  postgres-primary:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: postboy_sync
      POSTGRES_USER: postboy
      POSTGRES_PASSWORD: dev_password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./migrations:/docker-entrypoint-initdb.d

  postgres-replica:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: postboy_sync
      POSTGRES_USER: postboy
      POSTGRES_PASSWORD: dev_password
      PGSTANDBY_MODE: "on"
    ports:
      - "5433:5432"
    depends_on:
      - postgres-primary

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

  api:
    build:
      context: .
      dockerfile: Dockerfile.api
    environment:
      DATABASE_URL: "postgresql://postboy:dev_password@postgres-primary:5432/postboy_sync"
      REDIS_URL: "redis://redis:6379"
      JWT_SECRET: "dev_secret"
      RUST_LOG: "debug"
    ports:
      - "8080:8080"
    depends_on:
      - postgres-primary
      - redis

  websocket:
    build:
      context: .
      dockerfile: Dockerfile.ws
    environment:
      REDIS_URL: "redis://redis:6379"
      JWT_SECRET: "dev_secret"
      RUST_LOG: "debug"
    ports:
      - "8081:8081"
    depends_on:
      - redis

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      GF_SECURITY_ADMIN_PASSWORD: "admin"

volumes:
  postgres_data:
  redis_data:
```

---

## 11. 监控运维

### 11.1 Prometheus 指标

```rust
// src/metrics.rs

use prometheus::{
    Counter, Histogram, IntGauge, Registry, TextEncoder, Encoder,
};
use lazy_static::lazy_static;

lazy_static! {
    // HTTP 请求计数
    pub static ref HTTP_REQUESTS_TOTAL: Counter = Counter::new(
        "http_requests_total",
        "Total number of HTTP requests"
    ).unwrap();
    
    // HTTP 请求延迟
    pub static ref HTTP_REQUEST_DURATION: Histogram = Histogram::with_opts(
        prometheus::HistogramOpts {
            namespace: "http",
            subsystem: "request",
            name: "duration_seconds",
            help: "HTTP request latency",
            buckets: vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0],
        }
    ).unwrap();
    
    // WebSocket 连接数
    pub static ref WEBSOCKET_CONNECTIONS: IntGauge = IntGauge::new(
        "websocket_connections",
        "Current number of WebSocket connections"
    ).unwrap();
    
    // 同步操作计数
    pub static ref SYNC_OPERATIONS_TOTAL: Counter = Counter::new(
        "sync_operations_total",
        "Total number of sync operations"
    ).unwrap();
    
    // 数据库连接池
    pub static ref DB_POOL_SIZE: IntGauge = IntGauge::new(
        "db_pool_size",
        "Database connection pool size"
    ).unwrap();
    
    // 冲突计数
    pub static ref CONFLICTS_DETECTED_TOTAL: Counter = Counter::new(
        "conflicts_detected_total",
        "Total number of conflicts detected"
    ).unwrap();
}

/// 中间件：记录 HTTP 请求
pub async fn metrics_middleware(
    req: ServiceRequest,
    next: Next<impl Message>,
) -> Result<ServiceResponse, Error> {
    let start = Instant::now();
    let method = req.method().to_string();
    let path = req.path().to_string();
    
    let response = next.call(req).await?;
    
    let duration = start.elapsed();
    
    HTTP_REQUESTS_TOTAL.inc();
    HTTP_REQUEST_DURATION.observe(duration.as_secs_f64());
    
    Ok(response)
}
```

### 11.2 健康检查

```rust
// src/health.rs

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    timestamp: i64,
    checks: Checks,
}

#[derive(Serialize)]
struct Checks {
    database: CheckStatus,
    redis: CheckStatus,
    storage: CheckStatus,
}

#[derive(Serialize)]
struct CheckStatus {
    healthy: bool,
    latency_ms: Option<u64>,
    error: Option<String>,
}

pub async fn health_check(
    db: web::Data<Arc<Db>>,
    redis: web::Data<Arc<Redis>>,
) -> impl Responder {
    let db_status = check_database(db.as_ref()).await;
    let redis_status = check_redis(redis.as_ref()).await;
    let storage_status = check_storage().await;
    
    let all_healthy = db_status.healthy && redis_status.healthy && storage_status.healthy;
    
    let response = HealthResponse {
        status: if all_healthy { "healthy".to_string() } else { "unhealthy".to_string() },
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now().timestamp_millis(),
        checks: Checks {
            database: db_status,
            redis: redis_status,
            storage: storage_status,
        },
    };
    
    let status = if all_healthy { 
        http::StatusCode::OK 
    } else { 
        http::StatusCode::SERVICE_UNAVAILABLE 
    };
    
    HttpResponse::build(status).json(response)
}

async fn check_database(db: &Db) -> CheckStatus {
    let start = Instant::now();
    match db.ping().await {
        Ok(_) => CheckStatus {
            healthy: true,
            latency_ms: Some(start.elapsed().as_millis() as u64),
            error: None,
        },
        Err(e) => CheckStatus {
            healthy: false,
            latency_ms: None,
            error: Some(e.to_string()),
        },
    }
}
```

### 11.3 告警规则

```yaml
# prometheus/alerts.yaml
groups:
- name: postboy-sync
  rules:
  - alert: HighErrorRate
    expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.05
    for: 5m
    labels:
      severity: critical
    annotations:
      summary: "High error rate detected"
      description: "Error rate is {{ $value }} errors/sec"
  
  - alert: HighLatency
    expr: histogram_quantile(0.95, http_request_duration_seconds_bucket) > 1
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High latency detected"
      description: "P95 latency is {{ $value }} seconds"
  
  - alert: DatabaseConnectionsHigh
    expr: db_pool_size > 80
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "Database pool nearly exhausted"
  
  - alert: RedisDown
    expr: up{job="redis"} == 0
    for: 1m
    labels:
      severity: critical
    annotations:
      summary: "Redis is down"
  
  - alert: TooManyConflicts
    expr: rate(conflicts_detected_total[5m]) > 10
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High conflict rate detected"
```

---

## 附录

### A. API 响应代码

| 代码 | 说明 |
|------|------|
| 200 | 成功 |
| 201 | 创建成功 |
| 204 | 无内容 |
| 400 | 请求参数错误 |
| 401 | 未认证 |
| 403 | 无权限 |
| 404 | 资源不存在 |
| 409 | 冲突 |
| 422 | 验证失败 |
| 429 | 请求过多（限流） |
| 500 | 服务器错误 |
| 503 | 服务不可用 |

### B. 错误响应格式

```json
{
  "error": {
    "code": "CONFLICT_DETECTED",
    "message": "A conflict was detected for this item",
    "details": {
      "item_type": "request",
      "item_id": "uuid",
      "conflicts": [...]
    }
  }
}
```

### C. 速率限制

| 级别 | 请求限制 |
|------|----------|
| 免费用户 | 100 requests/minute |
| Pro 用户 | 1000 requests/minute |
| Team 用户 | 10000 requests/minute |
| Enterprise | 自定义 |

---

这份同步服务器设计文档提供了完整的服务端架构方案。接下来你可以：
1. **开始实现服务器代码** - 从数据模型和 API 端点开始
2. **创建数据库迁移脚本** - 基于提供的 Schema
3. **设置 CI/CD 流程** - 自动化测试和部署
4. **其他** - 请告诉我