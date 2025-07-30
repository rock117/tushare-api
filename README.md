# Tushare API - Rust Library

[![Crates.io](https://img.shields.io/crates/v/tushare-api.svg)](https://crates.io/crates/tushare-api)
[![Documentation](https://docs.rs/tushare-api/badge.svg)](https://docs.rs/tushare-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive Rust client library for accessing Tushare financial data APIs. This library provides type-safe, async access to all Tushare data interfaces.

## ✨ Features

- 🚀 **Async/Await Support**: Built for high-performance async operations
- 🔒 **Type Safety**: Strongly typed API enums and comprehensive error handling
- 🔧 **Developer Friendly**: Convenient macros and builder patterns
- 🌍 **Production Ready**: Comprehensive error handling and security features

## 📋 Requirements

- **Tushare API Token**: Register at [Tushare Pro](https://tushare.pro/) to get your API token

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tushare-api = "1.0.2"

# Optional: Enable tracing support
# tushare-api = { version = "1.0.2", features = ["tracing"] }
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
use tushare_api::{TushareClient, Api, request};
use tushare_derive::{FromTushareData, TushareResponseList};

// Define your struct with automatic conversion
#[derive(Debug, Clone, FromTushareData, TushareResponseList)]
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
    
    // Method 1: Using call_api_as for direct conversion
    let stocks: StockList = client.call_api_as(request!(Api::StockBasic, {
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
    
    Ok(())
}
```

#### Field Mapping and Optional Fields

```rust
use tushare_derive::{FromTushareData, TushareResponseList};

// Advanced struct with field mapping and optional fields
#[derive(Debug, Clone, FromTushareData, TushareResponseList)]
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

// The macro automatically generates StockInfoList wrapper type
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
    
    let stock_info: StockInfoList = client.call_api_as(request!(Api::StockBasic, {
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

#### Supported Field Types

The procedural macros support the following Rust types:

- `String` - Required string field
- `Option<String>` - Optional string field
- `f64` - Required floating-point number
- `Option<f64>` - Optional floating-point number
- `i64` - Required integer
- `Option<i64>` - Optional integer
- `bool` - Required boolean
- `Option<bool>` - Optional boolean

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
tushare-api = { version = "1.0.2", features = ["tracing"] }
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
tushare-api = "1.0.2"  # Without tracing feature
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

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Support

- 📖 [Documentation](https://docs.rs/tushare-api)
- 🐛 [Issue Tracker](https://github.com/rock117/tushare-api/issues)
- 💬 [Discussions](https://github.com/rock117/tushare-api/discussions)
