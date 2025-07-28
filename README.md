# Tushare API - Rust 库

> ⚠️ **开发状态**: 本项目还在开发中，API 可能会发生变化。请谨慎在生产环境中使用。

这是一个使用 Rust 编写的通用 Tushare API 客户端库，提供对 Tushare 各种数据接口的访问功能。

## 功能特性

- 支持所有 Tushare API 接口调用
- 类型安全的 API 枚举和错误处理
- 灵活的请求参数和字段配置
- 支持环境变量配置 Token
- 自定义超时设置
- 简洁的宏语法支持

## 前置要求

1. **Rust 环境**: 确保已安装 Rust (推荐使用 rustup)
2. **Tushare API Token**: 需要在 [Tushare官网](https://tushare.pro/) 注册并获取 API Token

## 安装

在你的 `Cargo.toml` 中添加以下依赖：

```toml
[dependencies]
tushare-api = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 使用方法

### 环境变量设置

首先，你需要设置 Tushare API Token 环境变量：

```bash
# Windows
set TUSHARE_TOKEN=your_token_here

# Linux/Mac
export TUSHARE_TOKEN=your_token_here
```

### 基本使用

#### 方式一：直接传入 Token

```rust
use tushare_api::{TushareClient, TushareRequest, Api, TushareResult, params, fields};

#[tokio::main]
async fn main() -> TushareResult<()> {
    // 创建客户端
    let client = TushareClient::new("your_tushare_token_here");
    
    // 创建请求 - 使用便捷宏
    let request = TushareRequest {
        api_name: Api::StockBasic,
        params: params!("list_status" => "L"),
        fields: fields!["ts_code", "name"],
    };
    
    // 调用 API
    let response = client.call_api(request).await?;
    println!("获取到 {} 条记录", response.data.items.len());
    
    Ok(())
}
```

#### 方式二：使用环境变量（推荐）

首先设置环境变量：
```bash
# Windows
set TUSHARE_TOKEN=your_tushare_token_here

# Linux/macOS
export TUSHARE_TOKEN=your_tushare_token_here
```

然后在代码中使用：
```rust
use tushare_api::{TushareClient, TushareRequest, Api, TushareResult, params, fields};

#[tokio::main]
async fn main() -> TushareResult<()> {
    // 从环境变量创建客户端
    let client = TushareClient::from_env()?;
    
    // 创建请求 - 使用便捷宏
    let request = TushareRequest {
        api_name: Api::StockBasic,
        params: params!("list_status" => "L"),
        fields: fields!["ts_code", "name"],
    };
    
    // 调用 API
    let response = client.call_api(request).await?;
    println!("获取到 {} 条记录", response.data.items.len());
    
    Ok(())
}
```

### 客户端创建方法

本库提供多种创建客户端的方式：

#### 1. 使用默认超时设置

```rust
// 直接传入 token
let client = TushareClient::new("your_token_here");

// 从环境变量获取 token
let client = TushareClient::from_env()?;
```

#### 2. 自定义超时设置

```rust
use std::time::Duration;

// 直接传入 token 和超时设置
let client = TushareClient::with_timeout(
    "your_token_here",
    Duration::from_secs(5),  // 连接超时 5 秒
    Duration::from_secs(60)  // 请求超时 60 秒
);

// 从环境变量获取 token 和自定义超时设置
let client = TushareClient::from_env_with_timeout(
    Duration::from_secs(5),  // 连接超时 5 秒
    Duration::from_secs(60)  // 请求超时 60 秒
)?;
```

> **推荐使用环境变量方式**：这样可以避免在代码中硬编码 API Token，提高安全性。

### 自定义超时设置

```rust
use tushare_api::TushareClient;
use std::time::Duration;

let client = TushareClient::with_timeout(
    "your_token_here",
    Duration::from_secs(5),  // 连接超时 5 秒
    Duration::from_secs(60)  // 请求超时 60 秒
);
```

## 运行示例

项目包含一个完整的使用示例，您可以这样运行：

```bash
# 设置环境变量
set TUSHARE_TOKEN=your_actual_token_here  # Windows
# 或
export TUSHARE_TOKEN=your_actual_token_here  # Linux/Mac

# 运行示例
cargo run --example basic_usage
```

## 便捷宏使用

库提供了便捷的宏来简化请求构建：

```rust
use tushare_api::{params, fields, request, Api};

// 使用 params! 宏创建参数
let params = params!("list_status" => "L", "exchange" => "SSE");

// 使用 fields! 宏创建字段列表
let fields = fields!["ts_code", "name", "industry"];

// 使用 request! 宏一次性创建完整请求
let req = request!(Api::StockBasic, {
    "list_status" => "L",
    "exchange" => "SSE"
}, [
    "ts_code", "name", "industry"
]);
```

## API 文档

### TushareClient

主要的客户端结构体，用于与 Tushare API 交互。

#### 方法

- `new(token: &str) -> Self`: 创建新的客户端实例（使用默认超时设置：连接超时 10 秒，请求超时 30 秒）
- `from_env() -> TushareResult<Self>`: 从环境变量 `TUSHARE_TOKEN` 创建客户端实例
- `with_timeout(token: &str, connect_timeout: Duration, timeout: Duration) -> Self`: 创建带自定义超时设置的客户端实例
- `from_env_with_timeout(connect_timeout: Duration, timeout: Duration) -> TushareResult<Self>`: 从环境变量创建带自定义超时的客户端
- `call_api(request: TushareRequest) -> TushareResult<TushareResponse>`: 调用 Tushare API

### TushareRequest

API 请求结构体，包含以下字段：

- `api_name`: API 名称（使用 `Api` 枚举）
- `params`: 请求参数（HashMap<String, String>）
- `fields`: 返回字段列表（Vec<String>）

### TushareResponse

API 响应结构体，包含以下字段：

- `request_id`: 请求 ID
- `code`: 响应状态码
- `msg`: 响应消息
- `data`: 响应数据（TushareData）

## 依赖项

- `reqwest`: HTTP 客户端库，用于发送 API 请求
- `tokio`: 异步运行时
- `serde`: 序列化/反序列化库
- `serde_json`: JSON 处理

## 支持的 API

本库支持所有 Tushare API 接口，通过 `Api` 枚举定义：

- `Api::StockBasic`: 股票基础信息
- `Api::Custom(String)`: 自定义 API 接口名称
- 更多 API 接口持续添加中...

## 错误处理

程序包含完善的错误处理机制：

- 检查环境变量是否设置
- 处理网络请求错误
- 处理 API 响应错误
- 提供用户友好的错误信息

## 注意事项

1. 请确保您的 Tushare API Token 有效且有足够的调用次数
2. 免费用户可能有 API 调用频率限制
3. 建议使用环境变量方式配置 Token 以提高安全性
4. 可根据需要调整超时设置以适应不同的网络环境

## 许可证

MIT License
