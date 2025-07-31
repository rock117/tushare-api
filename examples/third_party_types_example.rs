//! Example demonstrating third-party type support with feature flags
//!
//! This example shows how to use third-party types like rust_decimal::Decimal,
//! bigdecimal::BigDecimal, chrono date/time types, and uuid::Uuid with the
//! tushare-api library when the corresponding feature flags are enabled.
//!
//! To run this example with all third-party types:
//! ```bash
//! cargo run --example third_party_types_example --features all_types
//! ```
//!
//! Or with specific types:
//! ```bash
//! cargo run --example third_party_types_example --features "rust_decimal,chrono"
//! ```

// Example struct using rust_decimal::Decimal (when feature is enabled)
#[cfg(feature = "rust_decimal")]
use tushare_api::DeriveFromTushareData;

#[cfg(feature = "rust_decimal")]
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct FinancialData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    #[tushare(field = "trade_date")]
    pub date: String,
    
    /// High-precision decimal for financial calculations
    #[tushare(field = "close")]
    pub close_price: rust_decimal::Decimal,
    
    #[tushare(field = "vol")]
    pub volume: Option<rust_decimal::Decimal>,
    
    #[tushare(field = "amount")]
    pub amount: Option<rust_decimal::Decimal>,
}

// Example struct using bigdecimal::BigDecimal (when feature is enabled)
#[cfg(feature = "bigdecimal")]
use tushare_api::DeriveFromTushareData;

#[cfg(feature = "bigdecimal")]
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct BigDecimalData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    /// Using BigDecimal for very large numbers
    #[tushare(field = "market_cap")]
    pub market_cap: bigdecimal::BigDecimal,
    
    #[tushare(field = "pe_ratio")]
    pub pe_ratio: Option<bigdecimal::BigDecimal>,
}

// Example struct using chrono date/time types (when feature is enabled)
#[cfg(feature = "chrono")]
use tushare_api::DeriveFromTushareData;

#[cfg(feature = "chrono")]
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct DateTimeData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    /// Date parsing from YYYYMMDD format
    #[tushare(field = "trade_date")]
    pub trade_date: chrono::NaiveDate,
    
    /// Optional datetime field
    #[tushare(field = "update_time")]
    pub update_time: Option<chrono::NaiveDateTime>,
    
    /// UTC datetime
    #[tushare(field = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

// Example struct using uuid::Uuid (when feature is enabled)
#[cfg(feature = "uuid")]
use tushare_api::DeriveFromTushareData;

#[cfg(feature = "uuid")]
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct UuidData {
    #[tushare(field = "id")]
    pub record_id: uuid::Uuid,
    
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    #[tushare(field = "session_id")]
    pub session_id: Option<uuid::Uuid>,
}

// Example struct combining multiple third-party types
#[cfg(all(feature = "rust_decimal", feature = "chrono"))]
use tushare_api::DeriveFromTushareData;

#[cfg(all(feature = "rust_decimal", feature = "chrono"))]
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct CombinedData {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    #[tushare(field = "trade_date")]
    pub trade_date: chrono::NaiveDate,
    
    #[tushare(field = "close")]
    pub close_price: rust_decimal::Decimal,
    
    #[tushare(field = "vol")]
    pub volume: Option<rust_decimal::Decimal>,
    
    #[tushare(field = "update_time")]
    pub update_time: Option<chrono::NaiveDateTime>,
}

fn main() {
    println!("Third-party Type Support Example");
    println!("=================================");
    
    // Show which features are enabled
    #[cfg(feature = "rust_decimal")]
    println!("✓ rust_decimal support enabled");
    
    #[cfg(feature = "bigdecimal")]
    println!("✓ bigdecimal support enabled");
    
    #[cfg(feature = "chrono")]
    println!("✓ chrono support enabled");
    
    #[cfg(feature = "uuid")]
    println!("✓ uuid support enabled");
    
    // Show which features are disabled
    #[cfg(not(feature = "rust_decimal"))]
    println!("✗ rust_decimal support disabled");
    
    #[cfg(not(feature = "bigdecimal"))]
    println!("✗ bigdecimal support disabled");
    
    #[cfg(not(feature = "chrono"))]
    println!("✗ chrono support disabled");
    
    #[cfg(not(feature = "uuid"))]
    println!("✗ uuid support disabled");
    
    println!();
    
    // Demonstrate usage with mock data
    demonstrate_usage();
}

fn demonstrate_usage() {
    use serde_json::json;
    use tushare_api::traits::FromTushareValue;
    
    println!("Demonstrating third-party type conversions:");
    println!();
    
    // Test rust_decimal::Decimal
    #[cfg(feature = "rust_decimal")]
    {
        println!("rust_decimal::Decimal examples:");
        let price_str = json!("123.456");
        let price_num = json!(123.456);
        
        match rust_decimal::Decimal::from_tushare_value(&price_str) {
            Ok(decimal) => println!("  String '123.456' -> Decimal: {}", decimal),
            Err(e) => println!("  Error parsing string: {}", e),
        }
        
        match rust_decimal::Decimal::from_tushare_value(&price_num) {
            Ok(decimal) => println!("  Number 123.456 -> Decimal: {}", decimal),
            Err(e) => println!("  Error parsing number: {}", e),
        }
        println!();
    }
    
    // Test bigdecimal::BigDecimal
    #[cfg(feature = "bigdecimal")]
    {
        println!("bigdecimal::BigDecimal examples:");
        let big_num = json!("999999999999999999999.123456789");
        
        match bigdecimal::BigDecimal::from_tushare_value(&big_num) {
            Ok(big_decimal) => println!("  Large number -> BigDecimal: {}", big_decimal),
            Err(e) => println!("  Error parsing: {}", e),
        }
        println!();
    }
    
    // Test chrono date types
    #[cfg(feature = "chrono")]
    {
        println!("chrono date/time examples:");
        let date_yyyymmdd = json!("20240315");
        let date_iso = json!("2024-03-15");
        let datetime = json!("2024-03-15 14:30:00");
        
        match chrono::NaiveDate::from_tushare_value(&date_yyyymmdd) {
            Ok(date) => println!("  '20240315' -> NaiveDate: {}", date),
            Err(e) => println!("  Error parsing YYYYMMDD: {}", e),
        }
        
        match chrono::NaiveDate::from_tushare_value(&date_iso) {
            Ok(date) => println!("  '2024-03-15' -> NaiveDate: {}", date),
            Err(e) => println!("  Error parsing ISO date: {}", e),
        }
        
        match chrono::NaiveDateTime::from_tushare_value(&datetime) {
            Ok(dt) => println!("  '2024-03-15 14:30:00' -> NaiveDateTime: {}", dt),
            Err(e) => println!("  Error parsing datetime: {}", e),
        }
        println!();
    }
    
    // Test uuid::Uuid
    #[cfg(feature = "uuid")]
    {
        println!("uuid::Uuid examples:");
        let uuid_str = json!("550e8400-e29b-41d4-a716-446655440000");
        
        match uuid::Uuid::from_tushare_value(&uuid_str) {
            Ok(uuid) => println!("  UUID string -> Uuid: {}", uuid),
            Err(e) => println!("  Error parsing UUID: {}", e),
        }
        println!();
    }
    
    println!("To enable more types, run with feature flags:");
    println!("  cargo run --example third_party_types_example --features rust_decimal");
    println!("  cargo run --example third_party_types_example --features \"rust_decimal,chrono\"");
    println!("  cargo run --example third_party_types_example --features all_types");
}
