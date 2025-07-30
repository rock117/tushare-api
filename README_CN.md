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
tushare-api = "1.1.0"

# 可选：启用 tracing 支持
# tushare-api = { version = "1.1.0", features = ["tracing"] }
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

### 4. 使用过程宏自动转换结构体

该库提供了强大的过程宏，可以自动将 Tushare API 响应转换为强类型的 Rust 结构体，无需手动解析。

#### 使用过程宏

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList};
use tushare_derive::FromTushareData;

// 使用自动转换定义您的结构体
#[derive(Debug, Clone, FromTushareData)]
pub struct Stock {
    pub ts_code: String,
    pub symbol: String,
    pub name: String,
    pub area: Option<String>,
    pub industry: Option<String>,
    pub market: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    // 使用 call_api_as 进行直接转换到 TushareEntityList<Stock>
    let stocks: TushareEntityList<Stock> = client.call_api_as(request!(Api::StockBasic, {
        "list_status" => "L",
        "exchange" => "SSE"
    }, [
        "ts_code", "symbol", "name", "area", "industry", "market"
    ])).await?;
    
    // 直接访问数据
    println!("找到 {} 只股票:", stocks.len());
    for stock in stocks.iter().take(5) {
        println!("  {}: {} ({})", stock.ts_code, stock.name, stock.market);
    }
    
    // 访问分页信息
    println!("当前页面: {} 条记录", stocks.len());
    println!("总记录数: {}", stocks.count());
    println!("是否还有更多页面: {}", stocks.has_more());
    
    Ok(())
}
```

#### 字段映射和可选字段

```rust
use tushare_derive::FromTushareData;

// 带字段映射和可选字段的高级结构体
#[derive(Debug, Clone, FromTushareData)]
pub struct StockInfo {
    pub ts_code: String,
    
    // 将 API 字段 "symbol" 映射到结构体字段 "stock_symbol"
    #[tushare(field = "symbol")]
    pub stock_symbol: String,
    
    pub name: String,
    
    // 可选字段会自动处理
    pub area: Option<String>,
    pub industry: Option<String>,
    
    // 跳过 API 响应中不存在的字段
    #[tushare(skip)]
    pub calculated_value: f64,
}

// 实现 Default 以便使用
impl Default for StockInfo {
    fn default() -> Self {
        Self {
            ts_code: String::new(),
            stock_symbol: String::new(),
            name: String::new(),
            area: None,
            industry: None,
            calculated_value: 0.0,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    let stock_info: TushareEntityList<StockInfo> = client.call_api_as(request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "symbol", "name", "area", "industry"
    ])).await?;
    
    for info in stock_info.iter().take(3) {
        println!("股票: {} ({}) - 行业: {:?}", 
                 info.name, info.stock_symbol, info.industry);
    }
    
    Ok(())
}
```

#### 生成的结构体说明

当您使用新的泛型分页容器时，您会得到一个清晰、类型安全的接口：

```rust
// 您的原始结构体
#[derive(Debug, Clone, FromTushareData)]
pub struct Stock {
    pub ts_code: String,
    pub name: String,
    pub area: Option<String>,
}

// 使用泛型 TushareEntityList<T> 容器：
// TushareEntityList<Stock> {
//     pub items: Vec<Stock>,        // 您的数据项
//     pub has_more: bool,           // 分页：是否还有更多页面？
//     pub count: i64,               // 分页：总记录数
// }
```

**当您调用：**
```rust
let stocks: TushareEntityList<Stock> = client.call_api_as(request).await?;
// 或者
let stocks = client.call_api_as::<Stock>(request).await?;
```

**您会得到一个 `TushareEntityList<Stock>` 结构体，包含：**
- **`items`** - `Vec<Stock>` 包含实际转换后的数据
- **`has_more`** - `bool` 表示是否还有更多页面可获取
- **`count`** - `i64` 显示可用的总记录数

**以及这些自动生成的方法：**
- `stocks.len()` - 当前页面的项目数量
- `stocks.is_empty()` - 当前页面是否为空
- `stocks.items()` - 获取项目切片
- `stocks.has_more()` - 检查是否还有更多页面
- `stocks.count()` - 获取总记录数
- `stocks.iter()` - 遍历项目（通过 Deref）
- `for stock in &stocks { ... }` - 直接迭代支持

#### 分页支持

`TushareEntityList<T>` 容器提供内置分页支持，具有清晰直观的接口：

- `items: Vec<T>` - 实际的数据项
- `has_more: bool` - 是否还有更多页面可用
- `count: i64` - 总记录数

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList};
use tushare_derive::FromTushareData;

#[derive(Debug, Clone, FromTushareData)]
pub struct Stock {
    pub ts_code: String,
    pub name: String,
    pub area: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    // 获取分页结果
    let stocks: TushareEntityList<Stock> = client.call_api_as(request!(Api::StockBasic, {
        "list_status" => "L",
        "limit" => "100",
        "offset" => "0"
    }, [
        "ts_code", "name", "area"
    ])).await?;
    
    // 访问分页信息
    println!("当前页面: {} 只股票", stocks.len());
    println!("总可用数量: {} 只股票", stocks.count());
    println!("是否还有更多页面: {}", stocks.has_more());
    
    // 遍历当前页面的项目
    for stock in &stocks {
        println!("{}: {} ({})", 
                 stock.ts_code, 
                 stock.name, 
                 stock.area.as_deref().unwrap_or("未知"));
    }
    
    // 直接访问项目
    let first_stock = &stocks.items()[0];
    println!("第一只股票: {}", first_stock.name);
    
    Ok(())
}
```

#### 支持的字段类型

过程宏支持以下 Rust 类型：

- `String` - 必需的字符串字段
- `Option<String>` - 可选的字符串字段
- `f64` - 必需的浮点数
- `Option<f64>` - 可选的浮点数
- `i64` - 必需的整数
- `Option<i64>` - 可选的整数
- `bool` - 必需的布尔值
- `Option<bool>` - 可选的布尔值

#### 手动转换（替代方法）

如果您不想使用过程宏，仍然可以使用手动方法：

```rust
use tushare_api::{TushareClient, Api, request, utils::response_to_vec, traits::FromTushareData};
use tushare_api::error::TushareError;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Stock {
    pub ts_code: String,
    pub name: String,
    pub area: Option<String>,
}

// 手动实现 FromTushareData
impl FromTushareData for Stock {
    fn from_row(fields: &[String], values: &[Value]) -> Result<Self, TushareError> {
        let ts_code_idx = fields.iter().position(|f| f == "ts_code")
            .ok_or_else(|| TushareError::ParseError("缺少 ts_code 字段".to_string()))?;
        let name_idx = fields.iter().position(|f| f == "name")
            .ok_or_else(|| TushareError::ParseError("缺少 name 字段".to_string()))?;
        let area_idx = fields.iter().position(|f| f == "area");
            
        Ok(Stock {
            ts_code: values[ts_code_idx].as_str()
                .ok_or_else(|| TushareError::ParseError("无效的 ts_code".to_string()))?
                .to_string(),
            name: values[name_idx].as_str()
                .ok_or_else(|| TushareError::ParseError("无效的 name".to_string()))?
                .to_string(),
            area: area_idx.and_then(|idx| values[idx].as_str().map(|s| s.to_string())),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    // 获取原始响应
    let response = client.call_api(request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "name", "area"
    ])).await?;
    
    // 转换为 Vec<Stock>
    let stocks = response_to_vec::<Stock>(response)?;
    
    println!("找到 {} 只股票", stocks.len());
    for stock in stocks.iter().take(3) {
        println!("  {}: {} - 地区: {:?}", stock.ts_code, stock.name, stock.area);
    }
    
    Ok(())
}
```

### 5. 如何设置日志

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
tushare-api = { version = "1.1.0", features = ["tracing"] }
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

### 6. 主要数据结构

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
    pub async fn call_api_as<T>(&self, request: TushareRequest) -> TushareResult<T>
    where
        T: TryFrom<TushareResponse, Error = TushareError>;
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
