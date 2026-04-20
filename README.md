# RChat

> 基于 Rust + Vue3 的实时通讯系统，支持私聊、群聊、WebSocket 实时消息。

------

## 🛠️ 技术栈

### 后端

- Rust + Axum
- PostgreSQL + sqlx

### 前端

- Vue3 + TypeScript

### 数据库

- PostgreSQL（开发环境：18.3）

------

## ✨ 功能特性

- 用户注册 / 登录（JWT 鉴权）
- 好友系统（申请 / 接受 / 拒绝）
- 私聊 / 群聊
- 会话列表（最近消息、时间排序）
- WebSocket 实时消息（`/ws`）
- 用户头像上传与展示

------

## 📂 项目结构

```text
./src
├── config.rs        # 读取 .env 配置
├── db.rs            # 数据库连接
├── handlers/        # HTTP 接口层（axum handlers）
├── middleware/      # JWT 鉴权中间件
├── models/          # 数据结构 & DTO
├── services/        # 业务逻辑层
├── main.rs
```

------

## 🚀 快速开始

### 环境要求

- Rust >= 1.88
- Node.js >= 18
- PostgreSQL >= 14

------

### 后端启动

```bash
git clone https://github.com/yoshino-nya/rchat-backend.git
cd rchat-backend

createdb rchat
psql -d rchat -f migrations/schema.sql

cargo run
```

#### `.env` 示例

```env
DB_URL=postgres://dev:123456@localhost/rchat
BASE_URL=http://localhost:4000
PORT=4000
JWT_SECRET=your-secret-key
```

------

### 前端启动

```bash
git clone https://github.com/yoshino-nya/rchat-frontend.git
cd rchat-frontend

npm install
npm run dev
```

------

## 🔌 API / WebSocket

- HTTP API: `http://localhost:4000/api`
- WebSocket: `ws://localhost:4000/ws`

------

## 📄 License

MIT

------

## 👤 Author

- Yoshino-nya
