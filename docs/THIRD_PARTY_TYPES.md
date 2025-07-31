# Third-Party Type Support

The `tushare-api` library provides built-in support for popular third-party types through optional feature flags. This allows you to use types like `rust_decimal::Decimal`, `chrono::NaiveDate`, and `uuid::Uuid` directly in your data structures without manual conversion code.

## Supported Types

### rust_decimal::Decimal
High-precision decimal arithmetic for financial calculations.

**Feature flag:** `rust_decimal`

**Supported conversions:**
- String numbers: `"123.456"` → `Decimal`
- JSON numbers: `123.456` → `Decimal`
- Optional fields with null/empty handling

**Example:**
```rust
use tushare_api::DeriveFromTushareData;

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FinancialData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    #[tushare(field = "close")]
    pub close_price: rust_decimal::Decimal,
    
    #[tushare(field = "vol")]
    pub volume: Option<rust_decimal::Decimal>,
}
```

### bigdecimal::BigDecimal
Arbitrary precision decimal numbers for very large values.

**Feature flag:** `bigdecimal`

**Supported conversions:**
- String numbers: `"999999999999999999999.123456789"` → `BigDecimal`
- JSON numbers → `BigDecimal`
- Optional fields with null/empty handling

**Example:**
```rust
use tushare_api::DeriveFromTushareData;

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct BigDecimalData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    #[tushare(field = "market_cap")]
    pub market_cap: bigdecimal::BigDecimal,
}
```

### chrono Date/Time Types
Date and time handling with multiple format support.

**Feature flag:** `chrono`

**Supported types:**
- `chrono::NaiveDate`
- `chrono::NaiveDateTime`
- `chrono::DateTime<Utc>`

**Supported date formats:**
- `YYYYMMDD`: `"20240315"` → `NaiveDate`
- `YYYY-MM-DD`: `"2024-03-15"` → `NaiveDate`
- `YYYY/MM/DD`: `"2024/03/15"` → `NaiveDate`
- Number format: `20240315` → `NaiveDate`

**Supported datetime formats:**
- `YYYYMMDD HH:MM:SS`: `"20240315 14:30:00"` → `NaiveDateTime`
- `YYYY-MM-DD HH:MM:SS`: `"2024-03-15 14:30:00"` → `NaiveDateTime`
- `YYYY/MM/DD HH:MM:SS`: `"2024/03/15 14:30:00"` → `NaiveDateTime`
- `YYYY-MM-DDTHH:MM:SS`: `"2024-03-15T14:30:00"` → `NaiveDateTime`
- RFC3339 format for `DateTime<Utc>`

**Example:**
```rust
use tushare_api::DeriveFromTushareData;

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct DateTimeData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    #[tushare(field = "trade_date")]
    pub trade_date: chrono::NaiveDate,
    
    #[tushare(field = "update_time")]
    pub update_time: Option<chrono::NaiveDateTime>,
}
```

### uuid::Uuid
UUID (Universally Unique Identifier) support.

**Feature flag:** `uuid`

**Supported conversions:**
- Standard UUID strings: `"550e8400-e29b-41d4-a716-446655440000"` → `Uuid`
- Optional fields with null/empty handling

**Example:**
```rust
use tushare_api::DeriveFromTushareData;

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct UuidData {
    #[tushare(field = "id")]
    pub record_id: uuid::Uuid,
    
    #[tushare(field = "session_id")]
    pub session_id: Option<uuid::Uuid>,
}
```

## Feature Flags

Add the desired features to your `Cargo.toml`:

```toml
[dependencies]
tushare-api = { version = "1.1", features = ["rust_decimal", "chrono"] }
```

### Available Features

- `rust_decimal` - Enable `rust_decimal::Decimal` support
- `bigdecimal` - Enable `bigdecimal::BigDecimal` support
- `chrono` - Enable chrono date/time types support
- `uuid` - Enable `uuid::Uuid` support
- `all_types` - Enable all third-party type support (convenience feature)

### Examples

**Single feature:**
```toml
tushare-api = { version = "1.1", features = ["rust_decimal"] }
```

**Multiple features:**
```toml
tushare-api = { version = "1.1", features = ["rust_decimal", "chrono", "uuid"] }
```

**All features:**
```toml
tushare-api = { version = "1.1", features = ["all_types"] }
```

## Usage Examples

### Complete Example with Multiple Types

```rust
use tushare_api::{TushareClient, Api, request, TushareEntityList, DeriveFromTushareData};

#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct EnhancedStock {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    #[tushare(field = "name")]
    pub name: String,
    
    // High-precision decimal for price
    #[tushare(field = "close")]
    pub close_price: rust_decimal::Decimal,
    
    // Date parsing from YYYYMMDD format
    #[tushare(field = "trade_date")]
    pub trade_date: chrono::NaiveDate,
    
    // Optional volume with decimal precision
    #[tushare(field = "vol")]
    pub volume: Option<rust_decimal::Decimal>,
    
    // Optional last update timestamp
    #[tushare(field = "update_time")]
    pub last_updated: Option<chrono::NaiveDateTime>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TushareClient::from_env()?;
    
    let stocks: TushareEntityList<EnhancedStock> = client.call_api_as(request!(
        Api::StockBasic, {
            "list_status" => "L",
            "limit" => "10"
        }, [
            "ts_code", "name", "close", "trade_date", "vol", "update_time"
        ]
    )).await?;
    
    for stock in stocks.iter() {
        println!("Stock: {} - Price: {} on {}", 
                 stock.name, 
                 stock.close_price, 
                 stock.trade_date);
    }
    
    Ok(())
}
```

### Testing Third-Party Type Conversions

```rust
use tushare_api::traits::FromTushareValue;
use serde_json::json;

// Test rust_decimal conversion
let price = rust_decimal::Decimal::from_tushare_value(&json!("123.456"))?;
println!("Decimal: {}", price);

// Test chrono date conversion
let date = chrono::NaiveDate::from_tushare_value(&json!("20240315"))?;
println!("Date: {}", date);

// Test UUID conversion
let id = uuid::Uuid::from_tushare_value(&json!("550e8400-e29b-41d4-a716-446655440000"))?;
println!("UUID: {}", id);
```

## Error Handling

All third-party type conversions provide detailed error messages:

```rust
use tushare_api::traits::FromTushareValue;
use serde_json::json;

// This will return a descriptive error
match rust_decimal::Decimal::from_tushare_value(&json!("invalid_number")) {
    Ok(decimal) => println!("Parsed: {}", decimal),
    Err(e) => println!("Parse error: {}", e), // "Failed to parse decimal from string 'invalid_number': ..."
}
```

## Benefits

1. **Type Safety**: Compile-time guarantees for data types
2. **Zero-Cost Abstractions**: No runtime overhead when features are disabled
3. **Automatic Conversion**: No manual parsing code required
4. **Error Handling**: Detailed error messages for debugging
5. **Optional Dependencies**: Only include what you need
6. **ORM Compatibility**: Works seamlessly with generated entity types

## Custom Types

If you need support for additional third-party types, you can implement the traits manually:

```rust
use tushare_api::traits::{FromTushareValue, FromOptionalTushareValue};
use tushare_api::error::TushareError;

impl FromTushareValue for YourCustomType {
    fn from_tushare_value(value: &serde_json::Value) -> Result<Self, TushareError> {
        // Your conversion logic here
    }
}

impl FromOptionalTushareValue for YourCustomType {
    fn from_optional_tushare_value(value: &serde_json::Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            Self::from_tushare_value(value).map(Some)
        }
    }
}
```

This enables the same automatic conversion for your custom types in derived structs.
