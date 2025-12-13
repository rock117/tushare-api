# Tushare API - Rust Library

[![Crates.io](https://img.shields.io/crates/v/tushare-api.svg)](https://crates.io/crates/tushare-api)
[![Documentation](https://docs.rs/tushare-api/badge.svg)](https://docs.rs/tushare-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive Rust client library for accessing Tushare financial data APIs. This library provides type-safe, async access to all Tushare data interfaces.

## ✨ Features

- 🚀 **Async/Await Support**: Built for high-performance async operations
- 🔒 **Type Safety**: Strongly typed API enums and comprehensive error handling
- 🔧 **Developer Friendly**: Convenient macros and builder patterns
- 📊 **Third-Party Type Support**: Built-in support for `rust_decimal`, `chrono`, `uuid`, and `bigdecimal`
- 🌍 **Production Ready**: Comprehensive error handling and security features

## 📋 Requirements

- **Tushare API Token**: Register at [Tushare Pro](https://tushare.pro/) to get your API token

## 📦 Installation

Add this to your `Cargo.toml`:
ca
```toml
[dependencies]
tushare-api = "1.1.3"

# Optional: Enable third-party type support
# tushare-api = { version = "1.1.3", features = ["rust_decimal", "chrono"] }

# Or enable all third-party types
# tushare-api = { version = "1.1.3", features = ["all_types"] }

# Optional: Enable tracing support
# tushare-api = { version = "1.1.3", features = ["tracing"] }
```

## 🚀 Quick Start

```rust
use tushare_api::{TushareClient, Api, request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set your token as environment variable
    std::env::set_var("TUSHARE_TOKEN", "your_token_here");
    
    // Create client from environment variable
    let client = TushareClient::from_env()?;
    
    // Make API call using convenient macro
    let response = client.call_api(request!(Api::StockBasic, {
        "list_status" => "L",
        "exchange" => "SSE"
    }, [
        "ts_code", "name", "industry"
    ])).await?;
    
    println!("Retrieved {} records", response.data.items.len());
    Ok(())
}
```

## 📖 API Usage Guide

### 1. How to Import Tushare API

```rust
// Basic imports
use tushare_api::{TushareClient, TushareRequest, TushareResponse, Api, TushareResult};

// For convenient macros
use tushare_api::{params, fields, request};

// For logging configuration
use tushare_api::{LogLevel, LogConfig, Logger};

// For HTTP client configuration and connection pool settings
use tushare_api::{TushareClientBuilder, HttpClientConfig};

// For custom timeouts
use std::time::Duration;
```

### 2. How to Create Tushare Client

The library provides multiple ways to create a client:

#### Method 1: Using Environment Variable (Recommended)

```rust
// Set environment variable first
std::env::set_var("TUSHARE_TOKEN", "your_token_here");

// Create client with default settings
let client = TushareClient::from_env()?;

// Create client with custom timeouts
let client = TushareClient::from_env_with_timeout(
    Duration::from_secs(10),  // connect timeout
    Duration::from_secs(60)   // request timeout
)?;
```

#### Method 2: Direct Token

```rust
// Create client with default settings
let client = TushareClient::new("your_token_here");

// Create client with custom timeouts
let client = TushareClient::with_timeout(
    "your_token_here",
    Duration::from_secs(10),  // connect timeout
    Duration::from_secs(60)   // request timeout
);
```

#### Method 3: Using Builder Pattern

```rust
// Basic builder with timeouts and logging
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

// Advanced builder with connection pool settings
let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_connect_timeout(Duration::from_secs(5))
    .with_timeout(Duration::from_secs(60))
    .with_pool_max_idle_per_host(20)  // Max 20 idle connections per host
    .with_pool_idle_timeout(Duration::from_secs(60))  // Keep connections for 60s
    .with_log_level(LogLevel::Info)
    .build()?;

// Using HttpClientConfig for advanced HTTP settings
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

### 3. How to Send Requests

#### Method 1: Using Convenient Macros (Recommended)

```rust
use tushare_api::{request, Api};

// Single API call with parameters and fields
let response = client.call_api(request!(Api::StockBasic, {
    "list_status" => "L",
    "exchange" => "SSE"
}, [
    "ts_code", "name", "industry", "area"
])).await?;

// API call without parameters
let response = client.call_api(request!(Api::TradeCal, {}, [
    "exchange", "cal_date", "is_open"
])).await?;

// API call without fields (get all fields)
let response = client.call_api(request!(Api::StockBasic, {
    "list_status" => "L"
}, [])).await?;
```

#### Method 2: Using Individual Macros

```rust
use tushare_api::{params, fields, TushareRequest, Api};

// Create parameters
let params = params!(
    "list_status" => "L",
    "exchange" => "SSE"
);

// Create fields
let fields = fields![
    "ts_code", "name", "industry"
];

// Create request
let request = TushareRequest {
    api_name: Api::StockBasic,
    params,
    fields,
};

// Send request
let response = client.call_api(request).await?;
```

#### Method 3: Manual Construction

```rust
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("list_status".to_string(), "L".to_string());
params.insert("exchange".to_string(), "SSE".to_string());

let fields = vec![
    "ts_code".to_string(),
    "name".to_string(),
    "industry".to_string(),
];

let request = TushareRequest {
    api_name: Api::StockBasic,
    params,
    fields,
};

let response = client.call_api(request).await?;
```

### 4. Automatic Struct Conversion with Procedural Macros

The library provides powerful procedural macros to automatically convert Tushare API responses into strongly-typed Rust structs, eliminating the need for manual parsing.

#### Using Procedural Macros

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList};
use tushare_api::DeriveFromTushareData;

// Define your struct with automatic conversion
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
    
    // Using call_api_as for direct conversion to TushareEntityList<Stock>
    let stocks: TushareEntityList<Stock> = client.call_api_as(request!(Api::StockBasic, {
        "list_status" => "L",
        "exchange" => "SSE"
    }, [
        "ts_code", "symbol", "name", "area", "industry", "market"
    ])).await?;
    
    // Access the data directly
    println!("Found {} stocks:", stocks.len());
    for stock in stocks.iter().take(5) {
        println!("  {}: {} ({})", stock.ts_code, stock.name, stock.market);
    }
    
    // Access pagination information
    println!("Current page: {} items", stocks.len());
    println!("Total records: {}", stocks.count());
    println!("Has more pages: {}", stocks.has_more());
    
    Ok(())
}
```

#### Field Mapping and Optional Fields

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList};
use tushare_api::DeriveFromTushareData;

// Advanced struct with field mapping and optional fields
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct StockInfo {
    pub ts_code: String,
    
    // Map API field "symbol" to struct field "stock_symbol"
    #[tushare(field = "symbol")]
    pub stock_symbol: String,
    
    pub name: String,
    
    // Optional fields are automatically handled
    pub area: Option<String>,
    pub industry: Option<String>,
    
    // Skip fields that don't exist in API response
    #[tushare(skip)]
    pub calculated_value: f64,
}

// Implement Default for convenience
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
        println!("Stock: {} ({}) - Industry: {:?}", 
                 info.name, info.stock_symbol, info.industry);
    }
    
    Ok(())
}
```

#### Generated Struct Structure

When you use the new generic pagination container, you get a clean, type-safe interface:

```rust
// Your original struct
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct Stock {
    pub ts_code: String,
    pub name: String,
    pub area: Option<String>,
}

// Use the generic TushareEntityList<T> container:
// TushareEntityList<Stock> {
//     pub items: Vec<Stock>,        // Your data items
//     pub has_more: bool,           // Pagination: more pages available?
//     pub count: i64,               // Pagination: total record count
// }
```

**When you call:**
```rust
let stocks: TushareEntityList<Stock> = client.call_api_as(request).await?;
// OR
let stocks = client.call_api_as::<Stock>(request).await?;
```

**You get a `TushareEntityList<Stock>` struct with:**
- **`items`** - `Vec<Stock>` containing the actual converted data
- **`has_more`** - `bool` indicating if there are more pages to fetch
- **`count`** - `i64` showing the total number of records available

**Plus these automatically generated methods:**
- `stocks.len()` - Number of items in current page
- `stocks.is_empty()` - Whether current page is empty
- `stocks.items()` - Get items as slice
- `stocks.has_more()` - Check if more pages available
- `stocks.count()` - Get total record count
- `stocks.iter()` - Iterate over items (via Deref)
- `for stock in &stocks { ... }` - Direct iteration support

#### Pagination Support

The generic `TushareEntityList<T>` container provides built-in pagination support with a clean, intuitive interface:

- `items: Vec<T>` - The actual data items
- `has_more: bool` - Whether more pages are available
- `count: i64` - Total number of records

```rust
use tushare_api::{TushareClient, Api, request, DeriveFromTushareData};

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct Stock {
    pub ts_code: String,
    pub name: String,
    pub area: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    // Get paginated results
    let stocks: TushareEntityList<Stock> = client.call_api_as(request!(Api::StockBasic, {
        "list_status" => "L",
        "limit" => "100",
        "offset" => "0"
    }, [
        "ts_code", "name", "area"
    ])).await?;
    
    // Access pagination information
    println!("Current page: {} stocks", stocks.len());
    println!("Total available: {} stocks", stocks.count());
    println!("Has more pages: {}", stocks.has_more());
    
    // Iterate through current page items
    for stock in &stocks {
        println!("{}: {} ({})", 
                 stock.ts_code, 
                 stock.name, 
                 stock.area.as_deref().unwrap_or("Unknown"));
    }
    
    // Access items directly
    let first_stock = &stocks.items()[0];
    println!("First stock: {}", first_stock.name);
    
    Ok(())
}
```

#### Supported Field Types

The procedural macros support the following Rust types:

**Basic Types:**
- `String` - Required string field
- `Option<String>` - Optional string field
- `f64`, `f32` - Required floating-point numbers
- `Option<f64>`, `Option<f32>` - Optional floating-point numbers
- `i64`, `i32`, `i16`, `i8`, `isize` - Required signed integers
- `Option<i64>`, `Option<i32>`, etc. - Optional signed integers
- `u64`, `u32`, `u16`, `u8`, `usize` - Required unsigned integers
- `Option<u64>`, `Option<u32>`, etc. - Optional unsigned integers
- `bool` - Required boolean
- `Option<bool>` - Optional boolean
- `char` - Required character
- `Option<char>` - Optional character

**Third-Party Types (with feature flags):**
- `rust_decimal::Decimal` - High-precision decimal (feature: `rust_decimal`)
- `bigdecimal::BigDecimal` - Arbitrary precision decimal (feature: `bigdecimal`)
- `chrono::NaiveDate` - Date without timezone (feature: `chrono`)
- `chrono::NaiveDateTime` - DateTime without timezone (feature: `chrono`)
- `chrono::DateTime<Utc>` - UTC DateTime (feature: `chrono`)
- `uuid::Uuid` - UUID type (feature: `uuid`)
- All above types with `Option<T>` for optional fields

### 5. Third-Party Type Support

The library provides built-in support for popular third-party types through optional feature flags. This is especially useful for financial applications that require high-precision arithmetic or date/time handling.

#### Enabling Third-Party Types

Add the desired features to your `Cargo.toml`:

```toml
[dependencies]
# Enable specific types
tushare-api = { version = "1.1.3", features = ["rust_decimal", "chrono"] }

# Or enable all third-party types
tushare-api = { version = "1.1.3", features = ["all_types"] }
```

#### Example with High-Precision Decimals

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList, DeriveFromTushareData};

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FinancialData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    #[tushare(field = "trade_date")]
    pub date: String,
    
    // High-precision decimal for financial calculations
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
        println!("Stock: {} - Price: {} on {}", 
                 record.stock_code, 
                 record.close_price, 
                 record.date);
    }
    
    Ok(())
}
```

#### Example with Date/Time Types

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList, DeriveFromTushareData};

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct DateTimeData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    // Automatic parsing from YYYYMMDD format
    #[tushare(field = "trade_date")]
    pub trade_date: chrono::NaiveDate,
    
    // Optional datetime field
    #[tushare(field = "update_time")]
    pub update_time: Option<chrono::NaiveDateTime>,
    
    // High-precision price
    #[tushare(field = "close")]
    pub close_price: rust_decimal::Decimal,
}
```

#### Custom Date Format Support

The library supports custom date format parsing using the `#[tushare(date_format = "...")]` attribute. This is especially useful when dealing with APIs that return dates in non-standard formats.

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList, DeriveFromTushareData};

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct CustomDateFormats {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    // Standard date format (auto-detected: YYYYMMDD, YYYY-MM-DD, etc.)
    #[tushare(field = "trade_date")]
    pub trade_date: chrono::NaiveDate,
    
    // European date format: DD/MM/YYYY
    #[tushare(field = "european_date", date_format = "%d/%m/%Y")]
    pub european_date: chrono::NaiveDate,
    
    // US date format: MM-DD-YYYY
    #[tushare(field = "us_date", date_format = "%m-%d-%Y")]
    pub us_date: chrono::NaiveDate,
    
    // German date format: DD.MM.YYYY
    #[tushare(field = "german_date", date_format = "%d.%m.%Y")]
    pub german_date: Option<chrono::NaiveDate>,
    
    // Custom datetime format: YYYY/MM/DD HH:MM
    #[tushare(field = "custom_datetime", date_format = "%Y/%m/%d %H:%M")]
    pub custom_datetime: chrono::NaiveDateTime,
    
    // Chinese date format: YYYY年MM月DD日
    #[tushare(field = "chinese_date", date_format = "%Y年%m月%d日")]
    pub chinese_date: Option<chrono::NaiveDate>,
    
    // UTC datetime with timezone: YYYY-MM-DD HH:MM:SS +ZZZZ
    #[tushare(field = "utc_datetime", date_format = "%Y-%m-%d %H:%M:%S %z")]
    pub utc_datetime: chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    // Example API call (note: actual API may not return these exact formats)
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
        println!("Stock: {} - Trade Date: {}", record.stock_code, record.trade_date);
        println!("  European: {}", record.european_date);
        println!("  US: {}", record.us_date);
        println!("  German: {:?}", record.german_date);
        println!("  Custom DateTime: {:?}", record.custom_datetime);
        println!("  Chinese: {:?}", record.chinese_date);
        println!("  UTC: {}", record.utc_datetime);
        println!("---");
    }
    
    Ok(())
}
```

##### Common Date Format Patterns

| Format String | Example Input | Description |
|---------------|---------------|-------------|
| `"%Y-%m-%d"` | `"2024-03-15"` | ISO date format |
| `"%d/%m/%Y"` | `"15/03/2024"` | European format |
| `"%m-%d-%Y"` | `"03-15-2024"` | US format |
| `"%d.%m.%Y"` | `"15.03.2024"` | German format |
| `"%Y年%m月%d日"` | `"2024年03月15日"` | Chinese format |
| `"%Y%m%d"` | `"20240315"` | Compact format |
| `"%Y-%m-%d %H:%M:%S"` | `"2024-03-15 14:30:00"` | DateTime format |
| `"%Y/%m/%d %H:%M"` | `"2024/03/15 14:30"` | Custom DateTime |
| `"%Y-%m-%d %H:%M:%S %z"` | `"2024-03-15 14:30:00 +0800"` | With timezone |

##### Benefits of Custom Date Formats

- **Precise Control**: Specify exact format per field
- **No Wrapper Types**: Use chrono types directly
- **Type Safety**: Compile-time format validation
- **Flexible**: Works with optional fields
- **Clear Syntax**: Declarative and intuitive
- **Error Handling**: Detailed error messages for debugging

#### Supported Third-Party Types

| Type | Feature Flag | Description | Example Values |
|------|-------------|-------------|----------------|
| `rust_decimal::Decimal` | `rust_decimal` | High-precision decimal | `"123.456"`, `123.456` |
| `bigdecimal::BigDecimal` | `bigdecimal` | Arbitrary precision | `"999999999999999999999.123"` |
| `chrono::NaiveDate` | `chrono` | Date without timezone | `"20240315"`, `"2024-03-15"` |
| `chrono::NaiveDateTime` | `chrono` | DateTime without timezone | `"2024-03-15 14:30:00"` |
| `chrono::DateTime<Utc>` | `chrono` | UTC DateTime | RFC3339 format |
| `uuid::Uuid` | `uuid` | UUID type | `"550e8400-e29b-41d4-a716-446655440000"` |

For detailed documentation and examples, see [Third-Party Types Guide](docs/THIRD_PARTY_TYPES.md).

#### Manual Conversion (Alternative Approach)

If you prefer not to use procedural macros, you can still use the manual approach:

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

// Manual implementation of FromTushareData
impl FromTushareData for Stock {
    fn from_row(fields: &[String], values: &[Value]) -> Result<Self, TushareError> {
        let ts_code_idx = fields.iter().position(|f| f == "ts_code")
            .ok_or_else(|| TushareError::ParseError("Missing ts_code field".to_string()))?;
        let name_idx = fields.iter().position(|f| f == "name")
            .ok_or_else(|| TushareError::ParseError("Missing name field".to_string()))?;
        let area_idx = fields.iter().position(|f| f == "area");
            
        Ok(Stock {
            ts_code: values[ts_code_idx].as_str()
                .ok_or_else(|| TushareError::ParseError("Invalid ts_code".to_string()))?
                .to_string(),
            name: values[name_idx].as_str()
                .ok_or_else(|| TushareError::ParseError("Invalid name".to_string()))?
                .to_string(),
            area: area_idx.and_then(|idx| values[idx].as_str().map(|s| s.to_string())),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    // Get raw response
    let response = client.call_api(request!(Api::StockBasic, {
        "list_status" => "L"
    }, [
        "ts_code", "name", "area"
    ])).await?;
    
    // Convert to Vec<Stock>
    let stocks = response_to_vec::<Stock>(response)?;
    
    println!("Found {} stocks", stocks.len());
    for stock in stocks.iter().take(3) {
        println!("  {}: {} - Area: {:?}", stock.ts_code, stock.name, stock.area);
    }
    
    Ok(())
}
```

### 5. How to Configure Logging

The library supports both `log` and `tracing` ecosystems with flexible configuration.

#### Using with `env_logger` (Default)

```rust
// Set log level and initialize logger
std::env::set_var("RUST_LOG", "tushare_api=debug");
env_logger::init();

// Create client with logging configuration
let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_log_level(LogLevel::Debug)
    .log_requests(true)        // Log request details
    .log_responses(false)      // Don't log response content (can be large)
    .log_sensitive_data(false) // Don't log sensitive data like tokens
    .log_performance(true)     // Log performance metrics
    .build()?;
```

#### Using with `tracing` (Optional Feature)

First, enable the tracing feature in your `Cargo.toml`:

```toml
[dependencies]
tushare-api = { version = "1.1.3", features = ["tracing"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

Then in your code:

```rust
use tracing_subscriber;

// Initialize tracing subscriber
std::env::set_var("RUST_LOG", "tushare_api=trace");
tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();

// Client configuration remains the same
let client = TushareClient::builder()
    .with_token("your_token_here")
    .with_log_level(LogLevel::Trace)
    .log_requests(true)
    .log_responses(true)
    .log_performance(true)
    .build()?;
```

#### Using `tracing-log` Bridge

If you want to use `tracing` but the library is compiled without the tracing feature:

```toml
[dependencies]
tushare-api = "1.1.3"  # Without tracing feature
tracing = "0.1"
tracing-subscriber = "0.3"
tracing-log = "0.2"
```

```rust
use tracing_subscriber;
use tracing_log::LogTracer;

// Initialize log-to-tracing bridge
LogTracer::init()?;

// Set up tracing subscriber
std::env::set_var("RUST_LOG", "tushare_api=debug");
tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();
```

#### Log Levels and Output

- **`LogLevel::Off`**: No logging
- **`LogLevel::Error`**: Only errors
- **`LogLevel::Warn`**: Errors and warnings
- **`LogLevel::Info`**: Basic API call information (default)
- **`LogLevel::Debug`**: Detailed request/response information
- **`LogLevel::Trace`**: All information including raw response content

Example log output:
```
INFO  [abc123] Starting Tushare API call: stock_basic, params count: 2, fields count: 3
DEBUG [abc123] API request details - API: stock_basic, params: {...}, fields: [...]
DEBUG [abc123] Sending HTTP request to Tushare API
DEBUG [abc123] Received HTTP response, status code: 200
INFO  [abc123] API call successful, duration: 245ms, data rows returned: 100
```

### 6. Main Data Structures

#### TushareClient

The main client for interacting with Tushare APIs.

```rust
pub struct TushareClient {
    // Internal fields are private
}

impl TushareClient {
    // Creation methods
    pub fn new(token: &str) -> Self;
    pub fn from_env() -> TushareResult<Self>;
    pub fn with_timeout(token: &str, connect_timeout: Duration, timeout: Duration) -> Self;
    pub fn from_env_with_timeout(connect_timeout: Duration, timeout: Duration) -> TushareResult<Self>;
    pub fn builder() -> TushareClientBuilder;
    
    // API call methods
    pub async fn call_api(&self, request: TushareRequest) -> TushareResult<TushareResponse>;
    pub async fn call_api_as<T>(&self, request: TushareRequest) -> TushareResult<T>
    where
        T: TryFrom<TushareResponse, Error = TushareError>;
}
```

#### TushareRequest

Represents an API request with parameters and field specifications.

```rust
#[derive(Debug, Clone)]
pub struct TushareRequest {
    pub api_name: Api,                    // Which API to call
    pub params: HashMap<String, String>,  // Request parameters
    pub fields: Vec<String>,              // Fields to return
}
```

#### TushareResponse

Represents the response from Tushare API.

```rust
#[derive(Debug)]
pub struct TushareResponse {
    pub request_id: String,  // Unique request identifier
    pub code: i32,          // Response code (0 = success)
    pub msg: String,        // Response message
    pub data: TushareData,  // Actual data
}
```

#### TushareData

Contains the actual data returned by the API.

```rust
#[derive(Debug)]
pub struct TushareData {
    pub fields: Vec<String>,     // Field names
    pub items: Vec<Vec<String>>, // Data rows
}
```

#### Api Enum

Strongly typed enumeration of all supported APIs.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Api {
    // Stock APIs
    StockBasic,      // Basic stock information
    Daily,           // Daily stock prices
    DailyBasic,      // Daily basic stock data
    Weekly,          // Weekly stock data
    Monthly,         // Monthly stock data
    StockCompany,    // Company information
    
    // Fund APIs
    FundBasic,       // Basic fund information
    FundDaily,       // Daily fund data
    FundPortfolio,   // Fund portfolio
    
    // Index APIs
    IndexBasic,      // Basic index information
    IndexDaily,      // Daily index data
    IndexWeekly,     // Weekly index data
    IndexMonthly,    // Monthly index data
    IndexDailyBasic, // Daily basic index data
    
    // Market Data APIs
    TradeCal,        // Trading calendar
    Margin,          // Margin trading data
    MarginDetail,    // Detailed margin data
    Moneyflow,       // Money flow data
    MoneyflowMktDc,  // Market money flow
    MoneyflowIndustryThs, // Industry money flow
    
    // Financial APIs
    FinaIndicator,   // Financial indicators
    FinaMainbz,      // Main business data
    FinaMainbzVip,   // VIP main business data
    Balancesheet,    // Balance sheet
    Income,          // Income statement
    Cashflow,        // Cash flow statement
    
    // Other APIs
    StkHoldernumber, // Shareholder numbers
    ThsIndex,        // THS index data
    ThsMember,       // THS member data
    ThsDaily,        // THS daily data
    ThsHot,          // THS hot data
    
    // US Market APIs
    UsBasic,         // US stock basic info
    UsDaily,         // US stock daily data
    
    // Custom API
    Custom(String),  // For any other API by name
}
```

#### Error Types

Comprehensive error handling with specific error types.

```rust
#[derive(Debug)]
pub enum TushareError {
    HttpError(reqwest::Error),           // HTTP request errors
    ApiError { code: i32, message: String }, // API response errors
    SerializationError(serde_json::Error),   // JSON parsing errors
    TimeoutError,                        // Network timeout errors
    InvalidToken,                        // Invalid API token
    Other(String),                       // Other errors
}

pub type TushareResult<T> = Result<T, TushareError>;
```

#### Logging Configuration

```rust
#[derive(Debug, Clone)]
pub enum LogLevel {
    Off,    // No logging
    Error,  // Errors only
    Warn,   // Errors and warnings
    Info,   // Basic information (default)
    Debug,  // Detailed information
    Trace,  // All information including raw data
}

#[derive(Debug, Clone)]
pub struct LogConfig {
    pub level: LogLevel,              // Log level
    pub log_requests: bool,           // Log request parameters
    pub log_responses: bool,          // Log response content
    pub log_sensitive_data: bool,     // Log sensitive data (tokens)
    pub log_performance: bool,        // Log performance metrics
}
```

## 🔧 Advanced Usage

### Processing Response Data

```rust
let response = client.call_api(request!(Api::StockBasic, {
    "list_status" => "L"
}, [
    "ts_code", "name", "industry"
])).await?;

// Access field names
println!("Fields: {:?}", response.data.fields);

// Process each data row
for item in response.data.items {
    if item.len() >= 3 {
        println!("Stock: {} - {} ({})", item[0], item[1], item[2]);
    }
}
```

### Custom API Usage

```rust
// For APIs not yet included in the enum
let response = client.call_api(request!(Api::Custom("new_api".to_string()), {
    "param1" => "value1"
}, [
    "field1", "field2"
])).await?;
```

## 🔍 Examples

Check out the `examples/` directory for complete working examples:

```bash
# Run basic usage example
cargo run --example logging_example

# Run tracing integration example
cargo run --example tracing_example --features tracing
```

## 📋 Changelog

For a detailed history of changes, new features, and bug fixes, see [CHANGELOG.md](CHANGELOG.md).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Support

- 📖 [Documentation](https://docs.rs/tushare-api)
- 🐛 [Issue Tracker](https://github.com/rock117/tushare-api/issues)
- 💬 [Discussions](https://github.com/rock117/tushare-api/discussions)
