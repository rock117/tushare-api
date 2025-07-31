//! Example demonstrating custom date format parsing
//!
//! This example shows how to handle custom date formats using wrapper types
//! and custom implementations when you need specific date parsing behavior.

use tushare_api::{DeriveFromTushareData, traits::{FromTushareValue, FromOptionalTushareValue}, error::TushareError};
use serde_json::Value;

// Custom date wrapper with specific format
#[derive(Debug, Clone)]
pub struct CustomDate(pub chrono::NaiveDate);

impl FromTushareValue for CustomDate {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::String(s) => {
                // Try your specific custom format first
                if let Ok(date) = chrono::NaiveDate::parse_from_str(s, "%d/%m/%Y") {
                    Ok(CustomDate(date))
                } else if let Ok(date) = chrono::NaiveDate::parse_from_str(s, "%m-%d-%Y") {
                    Ok(CustomDate(date))
                } else {
                    // Fallback to standard formats
                    chrono::NaiveDate::from_tushare_value(value).map(CustomDate)
                }
            },
            _ => chrono::NaiveDate::from_tushare_value(value).map(CustomDate)
        }
    }
}

impl FromOptionalTushareValue for CustomDate {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            match value {
                Value::String(s) if s.is_empty() => Ok(None),
                _ => CustomDate::from_tushare_value(value).map(Some)
            }
        }
    }
}

// 特定格式的日期类型
#[derive(Debug, Clone)]
pub struct TushareDate(pub chrono::NaiveDate);

impl FromTushareValue for TushareDate {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::String(s) => {
                // Tushare 特定格式：YYYYMMDD
                chrono::NaiveDate::parse_from_str(s, "%Y%m%d")
                    .map(TushareDate)
                    .map_err(|e| TushareError::ParseError(format!(
                        "Failed to parse Tushare date from '{}': {}. Expected YYYYMMDD format", s, e
                    )))
            },
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    let date_str = i.to_string();
                    if date_str.len() == 8 {
                        chrono::NaiveDate::parse_from_str(&date_str, "%Y%m%d")
                            .map(TushareDate)
                            .map_err(|e| TushareError::ParseError(format!(
                                "Failed to parse Tushare date from number {}: {}", i, e
                            )))
                    } else {
                        Err(TushareError::ParseError(format!(
                            "Invalid Tushare date number: {}. Expected 8-digit YYYYMMDD", i
                        )))
                    }
                } else {
                    Err(TushareError::ParseError(format!(
                        "Cannot convert number {:?} to Tushare date", n
                    )))
                }
            },
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to Tushare date", value
            ))),
        }
    }
}

impl FromOptionalTushareValue for TushareDate {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            match value {
                Value::String(s) if s.is_empty() => Ok(None),
                _ => TushareDate::from_tushare_value(value).map(Some)
            }
        }
    }
}

// 使用自定义日期格式的结构体
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct StockWithCustomDate {
    #[tushare(field = "ts_code")]
    pub stock_code: String,
    
    #[tushare(field = "name")]
    pub name: String,
    
    // 使用标准 chrono 日期（支持多种格式）
    #[tushare(field = "list_date")]
    pub list_date: Option<chrono::NaiveDate>,
    
    // 使用自定义日期格式
    #[tushare(field = "trade_date")]
    pub trade_date: TushareDate,
    
    // 使用另一种自定义格式
    #[tushare(field = "custom_date")]
    pub custom_date: Option<CustomDate>,
}

fn main() {
    println!("Custom Date Format Example");
    println!("==========================");
    
    // 测试不同的日期格式解析
    test_date_parsing();
}

fn test_date_parsing() {
    use serde_json::json;
    
    println!("Testing various date formats:");
    
    // 测试标准 chrono::NaiveDate（现在支持更多格式）
    let test_dates = [
        json!("20240315"),           // YYYYMMDD
        json!("2024-03-15"),         // YYYY-MM-DD
        json!("2024/03/15"),         // YYYY/MM/DD
        json!("15/03/2024"),         // DD/MM/YYYY
        json!("03/15/2024"),         // MM/DD/YYYY
        json!("15-03-2024"),         // DD-MM-YYYY
        json!("03-15-2024"),         // MM-DD-YYYY
        json!("15.03.2024"),         // DD.MM.YYYY
        json!("2024.03.15"),         // YYYY.MM.DD
        json!("2024年03月15日"),      // 中文格式
        json!(20240315),             // 数字格式
    ];
    
    for (i, date_value) in test_dates.iter().enumerate() {
        match chrono::NaiveDate::from_tushare_value(date_value) {
            Ok(date) => println!("  Test {}: {} -> {}", i + 1, date_value, date),
            Err(e) => println!("  Test {}: {} -> Error: {}", i + 1, date_value, e),
        }
    }
    
    println!("\nTesting TushareDate (strict YYYYMMDD):");
    let tushare_dates = [
        json!("20240315"),
        json!(20240315),
        json!("2024-03-15"),  // 这个应该失败
    ];
    
    for (i, date_value) in tushare_dates.iter().enumerate() {
        match TushareDate::from_tushare_value(date_value) {
            Ok(date) => println!("  Test {}: {} -> {}", i + 1, date_value, date.0),
            Err(e) => println!("  Test {}: {} -> Error: {}", i + 1, date_value, e),
        }
    }
    
    println!("\nTesting CustomDate (DD/MM/YYYY and MM-DD-YYYY priority):");
    let custom_dates = [
        json!("15/03/2024"),   // DD/MM/YYYY
        json!("03-15-2024"),   // MM-DD-YYYY
        json!("20240315"),     // 回退到标准格式
    ];
    
    for (i, date_value) in custom_dates.iter().enumerate() {
        match CustomDate::from_tushare_value(date_value) {
            Ok(date) => println!("  Test {}: {} -> {}", i + 1, date_value, date.0),
            Err(e) => println!("  Test {}: {} -> Error: {}", i + 1, date_value, e),
        }
    }
}
