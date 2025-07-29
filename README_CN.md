# Tushare API - Rust 库

[![Crates.io](https://img.shields.io/crates/v/tushare-api.svg)](https://crates.io/crates/tushare-api)
[![Documentation](https://docs.rs/tushare-api/badge.svg)](https://docs.rs/tushare-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个用于访问 Tushare 金融数据 API的Rust 客户端库。该库提供类型安全的异步访问所有 Tushare 数据接口。

## ✨ 特性

- 🚀 **异步/等待支持**：为高性能异步操作而构建
- 🔒 **类型安全**：强类型 API 枚举和全面的错误处理
- 🔧 **开发者友好**：便捷的宏和构建器模式
- 🌍 **生产就绪**：全面的错误处理和安全特性

## 📋 要求

- **Tushare API Token**：在 [Tushare Pro](https://tushare.pro/) 注册以获取API token

## 📦 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
tushare-api = "1.0.0"

# 可选：启用 tracing 支持
# tushare-api = { version = "1.0.0", features = ["tracing"] }
```

## 🚀 快速开始

```rust
use tushare_api::{TushareClient, Api, request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置环境变量
    std::env::set_var("TUSHARE_TOKEN", "your_token_here");
    
    // 创建客户端
    let client = TushareClient::from_env()?;
    
    // 调用 API
    let response = client.call_api(request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "name", "industry"
    ])).await?;
    
    println!("获取到 {} 条记录", response.data.items.len());
    Ok(())
}
```

## 📖 API 使用指南

### 1. 如何导入 Tushare API

```rust
// 基础导入
use tushare_api::{TushareClient, TushareRequest, TushareResponse, Api, TushareResult};

// 便捷宏
use tushare_api::{params, fields, request};

// 日志配置
use tushare_api::{LogLevel, LogConfig, Logger};

// HTTP 客户端配置和连接池设置
use tushare_api::{TushareClientBuilder, HttpClientConfig};

// 自定义超时
use std::time::Duration;
```

### 2. 如何创建 Tushare 客户端

库提供多种创建客户端的方式：

#### 方法 1：使用环境变量（推荐）

```rust
// 首先设置环境变量
std::env::set_var("TUSHARE_TOKEN", "your_token_here");

// 使用默认超时设置
let client = TushareClient::from_env()?;

// 使用自定义超时设置
let client = TushareClient::from_env_with_timeout(
    Duration::from_secs(10),  // 连接超时 10 秒
    Duration::from_secs(60)   // 请求超时 60 秒
)?;
```

#### 方法 2：直接使用 Token

```rust
// 使用默认超时设置
let client = TushareClient::new("your_token_here");

// 使用自定义超时设置
let client = TushareClient::with_timeout(
    "your_token_here",
    Duration::from_secs(5),   // 连接超时 5 秒
    Duration::from_secs(60)   // 请求超时 60 秒
);
```

#### 方法 3：使用构建器模式

```rust
// 基础构建器，包含超时和日志
let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_connect_timeout(Duration::from_secs(10))
    .with_timeout(Duration::from_secs(60))
    .with_log_level(LogLevel::Debug)
    .log_requests(true)
    .log_responses(false)
    .log_sensitive_data(false)
    .log_performance(true)
    .build()?;

// 高级构建器，包含连接池设置
let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_connect_timeout(Duration::from_secs(5))
    .with_timeout(Duration::from_secs(60))
    .with_pool_max_idle_per_host(20)  // 每个主机最多 20 个空闲连接
    .with_pool_idle_timeout(Duration::from_secs(60))  // 连接保持 60 秒
    .with_log_level(LogLevel::Info)
    .build()?;

// 使用 HttpClientConfig 进行高级 HTTP 设置
let http_config = HttpClientConfig::new()
    .with_connect_timeout(Duration::from_secs(3))
    .with_timeout(Duration::from_secs(30))
    .with_pool_max_idle_per_host(15)
    .with_pool_idle_timeout(Duration::from_secs(45));

let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_http_config(http_config)
    .with_log_level(LogLevel::Debug)
    .build()?;
```

### 3. 如何发送请求

#### 方法 1：使用便捷宏（推荐）

```rust
use tushare_api::{request, Api};

// 带参数和字段的单个 API 调用
let response = client.call_api(request!(Api::StockBasic, {
    "list_status" => "L",
    "exchange" => "SSE"
}, [
    "ts_code", "name", "industry", "area"
])).await?;

// 无参数的 API 调用
let response = client.call_api(request!(Api::TradeCal, {}, [
    "exchange", "cal_date", "is_open"
])).await?;

// 无字段的 API 调用（获取所有字段）
let response = client.call_api(request!(Api::StockBasic, {
    "list_status" => "L"
}, [])).await?;
```

#### 方法 2：使用单独的宏

```rust
use tushare_api::{TushareRequest, Api, params, fields};

let request = TushareRequest {
    api_name: Api::StockBasic,
    params: params!("list_status" => "L", "exchange" => "SSE"),
    fields: fields!("ts_code", "name", "industry"),
};

let response = client.call_api(request).await?;
```

#### 方法 3：手动构建

```rust
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("list_status".to_string(), "L".to_string());

let request = TushareRequest {
    api_name: Api::StockBasic,
    params,
    fields: vec!["ts_code".to_string(), "name".to_string()],
};

let response = client.call_api(request).await?;
```

### 4. 如何设置日志

#### 使用 `env_logger`

```rust
// 设置日志级别并初始化日志器
std::env::set_var("RUST_LOG", "tushare_api=debug");
env_logger::init();

// 创建带日志配置的客户端
let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_log_level(LogLevel::Debug)
    .log_requests(true)        // 记录请求详情
    .log_responses(false)      // 不记录响应内容（可能很大）
    .log_sensitive_data(false) // 不记录敏感数据如 token
    .log_performance(true)     // 记录性能指标
    .build()?;
```

#### 使用 `tracing`（可选特性）

首先，在您的 `Cargo.toml` 中启用 tracing 特性：

```toml
[dependencies]
tushare-api = { version = "1.0.0", features = ["tracing"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

然后在您的代码中：

```rust
use tracing_subscriber;

// 初始化 tracing 订阅器
std::env::set_var("RUST_LOG", "tushare_api=trace");
tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

// 客户端配置保持不变
let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_log_level(LogLevel::Trace)
    .build()?;
```

#### 使用 `tracing-log` 桥接

```rust
use tracing_subscriber;
use tracing_log::LogTracer;

// 初始化 log-to-tracing 桥接
LogTracer::init()?;

// 设置 tracing 订阅器
std::env::set_var("RUST_LOG", "tushare_api=debug");
tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();
```

#### 日志级别和输出

- **`LogLevel::Off`**：无日志
- **`LogLevel::Error`**：仅错误
- **`LogLevel::Warn`**：错误和警告
- **`LogLevel::Info`**：基本 API 调用信息（默认）
- **`LogLevel::Debug`**：详细的请求/响应信息
- **`LogLevel::Trace`**：所有信息，包括原始响应内容

示例日志输出：
```
INFO  [abc123] Starting Tushare API call: stock_basic, params count: 2, fields count: 3
DEBUG [abc123] API request details - API: stock_basic, params: {...}, fields: [...]
DEBUG [abc123] Sending HTTP request to Tushare API
DEBUG [abc123] Received HTTP response, status code: 200
INFO  [abc123] API call successful, duration: 245ms, data rows returned: 100
```

### 5. 主要数据结构

#### TushareClient

与 Tushare API 交互的主要客户端。

```rust
pub struct TushareClient {
    // 内部字段是私有的
}

impl TushareClient {
    // 创建方法
    pub fn new(token: &str) -> Self;
    pub fn from_env() -> TushareResult<Self>;
    pub fn with_timeout(token: &str, connect_timeout: Duration, timeout: Duration) -> Self;
    pub fn from_env_with_timeout(connect_timeout: Duration, timeout: Duration) -> TushareResult<Self>;
    pub fn builder() -> TushareClientBuilder;
    
    // API 调用方法
    pub async fn call_api(&self, request: TushareRequest) -> TushareResult<TushareResponse>;
}
```

#### TushareRequest

表示带参数和字段规范的 API 请求。

```rust
#[derive(Debug, Clone)]
pub struct TushareRequest {
    pub api_name: Api,                    // 要调用的 API
    pub params: HashMap<String, String>,  // 请求参数
    pub fields: Vec<String>,              // 要返回的字段
}
```

#### TushareResponse

表示来自 Tushare API 的响应。

```rust
#[derive(Debug)]
pub struct TushareResponse {
    pub request_id: String,  // 唯一请求标识符
    pub code: i32,          // 响应代码（0 = 成功）
    pub msg: String,        // 响应消息
    pub data: TushareData,  // 实际数据
}
```

#### TushareData

包含 API 返回的实际数据。

```rust
#[derive(Debug)]
pub struct TushareData {
    pub fields: Vec<String>,     // 字段名
    pub items: Vec<Vec<String>>, // 数据行
}
```

#### Api 枚举

所有支持的 API 的强类型枚举。

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Api {
    StockBasic,     // 基础股票信息
    Daily,          // 日线数据
    TradeCal,       // 交易日历
    FundBasic,      // 基金基础信息
    IndexBasic,     // 指数基础信息
    // ... 更多 API
    Custom(String), // 按名称的任何其他 API
}
```

## 🔧 高级用法

### 处理响应数据

```rust
let response = client.call_api(request!(Api::StockBasic, {
    "list_status" => "L"
}, [
    "ts_code", "name", "industry"
])).await?;

// 遍历数据行
for item in response.data.items {
    let ts_code = &item[0];  // 第一个字段
    let name = &item[1];     // 第二个字段
    let industry = &item[2]; // 第三个字段
    
    println!("股票: {} - {} ({})", ts_code, name, industry);
}

// 或者使用字段索引
let field_indices: std::collections::HashMap<_, _> = response.data.fields
    .iter()
    .enumerate()
    .map(|(i, field)| (field.as_str(), i))
    .collect();

for item in response.data.items {
    if let Some(&ts_code_idx) = field_indices.get("ts_code") {
        println!("股票代码: {}", item[ts_code_idx]);
    }
}
```

## 🧪 运行示例

```bash
# 设置您的 token
export TUSHARE_TOKEN="your_token_here"

# 运行基础示例
cargo run --example basic_usage

# 运行日志示例
cargo run --example logging_example

# 运行 tracing 示例（需要 tracing 特性）
cargo run --example tracing_example --features tracing
```

## 📄 许可证

本项目采用 MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件。

## 📞 支持

- 📖 [文档](https://docs.rs/tushare-api)
- 🐛 [问题跟踪](https://github.com/rock117/tushare-api/issues)
- 💬 [讨论](https://github.com/rock117/tushare-api/discussions)
