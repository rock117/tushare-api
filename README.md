# Tushare API - Rust 库

[![Crates.io](https://img.shields.io/crates/v/tushare-api.svg)](https://crates.io/crates/tushare-api)
[![Documentation](https://docs.rs/tushare-api/badge.svg)](https://docs.rs/tushare-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个用于访问 Tushare 金融数据 API的Rust 客户端库。该库提供类型安全的异步访问所有 Tushare 数据接口。

## ✨ 特性

- 🚀 **异步**：为高性能异步操作而构建
- 🔒 **类型安全**：强类型 API 枚举和全面的错误处理
- 🔧 **开发者友好**：便捷的宏和构建器模式
- 📊 **第三方类型支持**：内置支持 `rust_decimal`、`chrono`、`uuid` 和 `bigdecimal`
- 🌍 **生产就绪**：全面的错误处理和安全特性

## 📋 前置条件

- **Tushare API Token**：在 [Tushare Pro](https://tushare.pro/) 注册以获取API token

## 📦 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
tushare-api = "1.2.5"

# 可选：启用第三方类型支持
# tushare-api = { version = "1.2.5", features = ["rust_decimal", "chrono"] }

# 或启用所有第三方类型
# tushare-api = { version = "1.2.5", features = ["all_types"] }

# 可选：启用 tracing 支持
# tushare-api = { version = "1.2.5", features = ["tracing"] }
```

## 🚀 快速开始

```rust
use tushare_api::TushareClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置环境变量
    std::env::set_var("TUSHARE_TOKEN", "your_token_here");
    
    // 创建客户端
    let client = TushareClient::from_env()?;
    
    // 调用 API
    let response = client
        .call_api(
            r#"
            {
                "api_name": "stock_basic",
                "params": { "list_status": "L" },
                "fields": ["ts_code", "name", "industry"]
            }
            "#,
        )
        .await?;
    
    if let Some(data) = response.data {
        println!("获取到 {} 条记录", data.items.len());
    }
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

#### 方法 4：直接使用字符串参数

```rust
// 直接传 JSON 字符串（适合快速调试/复制粘贴 API 请求）
let response = client
    .call_api(
        r#"
        {
            "api_name": "stock_basic",
            "params": { "list_status": "L", "exchange": "SSE" },
            "fields": ["ts_code", "name", "industry", "area"]
        }
        "#,
    )
    .await?;
```

#### 方法 5：任何实现了 `TryInto<TushareRequest>` 的类型都可以作为参数

`call_api` 的签名是泛型的：

```rust
pub async fn call_api<T>(&self, request: T) -> TushareResult<TushareResponse>
where
    T: TryInto<TushareRequest>,
    <T as TryInto<TushareRequest>>::Error: Into<TushareError>,
```

所以你可以直接传入：

```rust
// 1) 直接传 TushareRequest
let req = TushareRequest {
    api_name: Api::StockBasic,
    params: params!("list_status" => "L"),
    fields: fields!("ts_code", "name"),
};
let response = client.call_api(req).await?;

// 2) 直接传 &str 
let response = client.call_api(r#"{
    "api_name": "stock_basic",
    "params": { "list_status": "L" },
    "fields": ["ts_code", "name"]
}"#).await?;

// 3) 直接传 String
let json = r#"{
    "api_name": "stock_basic",
    "params": { "list_status": "L" },
    "fields": ["ts_code", "name"]
}"#.to_string();
let response = client.call_api(json).await?;
```

### 4. 使用过程宏自动转换结构体

该库提供了强大的过程宏，可以自动将 Tushare API 响应转换为强类型的 Rust 结构体，无需手动解析。

#### 使用过程宏

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList};
use tushare_api::DeriveFromTushareData;

// 使用自动转换定义您的结构体
#[derive(Debug, Clone, DeriveFromTushareData)]
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
use tushare_api::DeriveFromTushareData;

// 带字段映射和可选字段的高级结构体
#[derive(Debug, Clone, DeriveFromTushareData)]
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
#[derive(Debug, Clone, DeriveFromTushareData)]
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
use tushare_api::DeriveFromTushareData;

#[derive(Debug, Clone, DeriveFromTushareData)]
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

#### 自定义日期格式支持

库支持使用 `#[tushare(date_format = "...")]` 属性进行自定义日期格式解析。这在处理返回非标准日期格式的 API 时特别有用。

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList, DeriveFromTushareData};

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct CustomDateFormats {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    // 标准日期格式（自动检测：YYYYMMDD、YYYY-MM-DD 等）
    #[tushare(field = "trade_date")]
    pub trade_date: chrono::NaiveDate,
    
    // 欧洲日期格式：DD/MM/YYYY
    #[tushare(field = "european_date", date_format = "%d/%m/%Y")]
    pub european_date: chrono::NaiveDate,
    
    // 美国日期格式：MM-DD-YYYY
    #[tushare(field = "us_date", date_format = "%m-%d-%Y")]
    pub us_date: chrono::NaiveDate,
    
    // 德国日期格式：DD.MM.YYYY
    #[tushare(field = "german_date", date_format = "%d.%m.%Y")]
    pub german_date: Option<chrono::NaiveDate>,
    
    // 自定义日期时间格式：YYYY/MM/DD HH:MM
    #[tushare(field = "custom_datetime", date_format = "%Y/%m/%d %H:%M")]
    pub custom_datetime: chrono::NaiveDateTime,
    
    // 中文日期格式：YYYY年MM月DD日
    #[tushare(field = "chinese_date", date_format = "%Y年%m月%d日")]
    pub chinese_date: Option<chrono::NaiveDate>,
    
    // UTC 日期时间格式：YYYY-MM-DD HH:MM:SS +ZZZZ
    #[tushare(field = "utc_datetime", date_format = "%Y-%m-%d %H:%M:%S %z")]
    pub utc_datetime: chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    // API 调用示例（注意：实际 API 可能不返回这些确切格式）
    let data: TushareEntityList<CustomDateFormats> = client.call_api_as(request!(
        Api::StockBasic, {
            "list_status" => "L",
            "limit" => "10"
        }, [
            "ts_code", "trade_date", "european_date", "us_date", 
            "german_date", "custom_datetime", "chinese_date", "utc_datetime"
        ]
    )).await?;
    
    for record in data.iter() {
        println!("股票: {} - 交易日期: {}", record.stock_code, record.trade_date);
        println!("  欧洲格式: {}", record.european_date);
        println!("  美国格式: {}", record.us_date);
        println!("  德国格式: {:?}", record.german_date);
        println!("  自定义日期时间: {:?}", record.custom_datetime);
        println!("  中文格式: {:?}", record.chinese_date);
        println!("  UTC 时间: {}", record.utc_datetime);
        println!("---");
    }
    
    Ok(())
}
```

##### 常用日期格式模式

| 格式字符串 | 示例输入 | 说明 |
|-----------|---------|------|
| `"%Y-%m-%d"` | `"2024-03-15"` | ISO 日期格式 |
| `"%d/%m/%Y"` | `"15/03/2024"` | 欧洲格式 |
| `"%m-%d-%Y"` | `"03-15-2024"` | 美国格式 |
| `"%d.%m.%Y"` | `"15.03.2024"` | 德国格式 |
| `"%Y年%m月%d日"` | `"2024年03月15日"` | 中文格式 |
| `"%Y%m%d"` | `"20240315"` | 紧凑格式 |
| `"%Y-%m-%d %H:%M:%S"` | `"2024-03-15 14:30:00"` | 日期时间格式 |
| `"%Y/%m/%d %H:%M"` | `"2024/03/15 14:30"` | 自定义日期时间 |
| `"%Y-%m-%d %H:%M:%S %z"` | `"2024-03-15 14:30:00 +0800"` | 带时区格式 |

##### 自定义日期格式的优势

- **精确控制**：为每个字段指定确切的格式
- **无需包装类型**：直接使用 chrono 类型
- **类型安全**：编译时格式验证
- **灵活性**：支持可选字段
- **清晰语法**：声明式且直观
- **错误处理**：详细的错误信息便于调试

#### 第三方类型支持

库通过可选的特性标志为流行的第三方类型提供内置支持。这对于需要高精度算术或日期/时间处理的金融应用程序特别有用。

##### 启用第三方类型

在您的 `Cargo.toml` 中添加所需的特性：

```toml
[dependencies]
# 启用特定类型
tushare-api = { version = "1.2.5", features = ["rust_decimal", "chrono"] }

# 或启用所有第三方类型
tushare-api = { version = "1.2.5", features = ["all_types"] }
```

##### 高精度小数示例

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList, DeriveFromTushareData};

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FinancialData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    #[tushare(field = "trade_date")]
    pub date: String,
    
    // 用于金融计算的高精度小数
    #[tushare(field = "close")]
    pub close_price: rust_decimal::Decimal,
    
    #[tushare(field = "vol")]
    pub volume: Option<rust_decimal::Decimal>,
    
    #[tushare(field = "amount")]
    pub amount: Option<rust_decimal::Decimal>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    let data: TushareEntityList<FinancialData> = client.call_api_as(request!(
        Api::Daily, {
            "ts_code" => "000001.SZ",
            "trade_date" => "20240315"
        }, [
            "ts_code", "trade_date", "close", "vol", "amount"
        ]
    )).await?;
    
    for record in data.iter() {
        println!("股票: {} - 价格: {} 日期: {}", 
                 record.stock_code, 
                 record.close_price, 
                 record.date);
    }
    
    Ok(())
}
```

##### 日期/时间类型示例

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList, DeriveFromTushareData};

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct DateTimeData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    // 从 YYYYMMDD 格式自动解析
    #[tushare(field = "trade_date")]
    pub trade_date: chrono::NaiveDate,
    
    // 可选的日期时间字段
    #[tushare(field = "update_time")]
    pub update_time: Option<chrono::NaiveDateTime>,
    
    // 高精度价格
    #[tushare(field = "close")]
    pub close_price: rust_decimal::Decimal,
}
```

##### 支持的第三方类型

| 类型 | 特性标志 | 说明 | 示例值 |
|------|---------|------|--------|
| `rust_decimal::Decimal` | `rust_decimal` | 高精度小数 | `"123.456"`, `123.456` |
| `bigdecimal::BigDecimal` | `bigdecimal` | 任意精度 | `"999999999999999999999.123"` |
| `chrono::NaiveDate` | `chrono` | 无时区日期 | `"20240315"`, `"2024-03-15"` |
| `chrono::NaiveDateTime` | `chrono` | 无时区日期时间 | `"2024-03-15 14:30:00"` |
| `chrono::DateTime<Utc>` | `chrono` | UTC 日期时间 | RFC3339 格式 |
| `uuid::Uuid` | `uuid` | UUID 类型 | `"550e8400-e29b-41d4-a716-446655440000"` |

详细文档和示例请参阅 [第三方类型指南](docs/THIRD_PARTY_TYPES.md)。

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
tushare-api = { version = "1.2.5", features = ["tracing"] }
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
