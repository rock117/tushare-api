//! Example demonstrating custom date format support with #[tushare(date_format = "...")] attribute
//!
//! This example shows how to use custom date formats for parsing date fields
//! from Tushare API responses using the procedural macro attribute.
//!
//! Run with:
//! ```bash
//! cargo run --example custom_date_format_example --features chrono
//! ```

#[cfg(feature = "chrono")]
mod chrono_example {
    use tushare_api::{DeriveFromTushareData, traits::{FromTushareValue, FromOptionalTushareValue}, error::TushareError};
    use serde_json::Value;

    /// Custom date wrapper that supports multiple input formats
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
                        // Fall back to standard parsing
                        chrono::NaiveDate::from_tushare_value(value).map(CustomDate)
                    }
                }
                _ => chrono::NaiveDate::from_tushare_value(value).map(CustomDate)
            }
        }
    }

    impl FromOptionalTushareValue for CustomDate {
        fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
            match value {
                Value::Null => Ok(None),
                _ => CustomDate::from_tushare_value(value).map(Some)
            }
        }
    }

    /// Another example: Tushare-specific date format (YYYYMMDD)
    #[derive(Debug, Clone)]
    pub struct TushareDate(pub chrono::NaiveDate);

    impl FromTushareValue for TushareDate {
        fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
            match value {
                Value::String(s) => {
                    // Parse YYYYMMDD format specifically
                    chrono::NaiveDate::parse_from_str(s, "%Y%m%d")
                        .map(TushareDate)
                        .map_err(|e| TushareError::ParseError(format!("Failed to parse date '{}': {}", s, e)))
                }
                Value::Number(n) => {
                    if let Some(date_int) = n.as_u64() {
                        let date_str = date_int.to_string();
                        if date_str.len() == 8 {
                            chrono::NaiveDate::parse_from_str(&date_str, "%Y%m%d")
                                .map(TushareDate)
                                .map_err(|e| TushareError::ParseError(format!("Failed to parse date '{}': {}", date_str, e)))
                        } else {
                            Err(TushareError::ParseError(format!("Invalid date format: {}", date_int)))
                        }
                    } else {
                        Err(TushareError::ParseError("Date must be a valid integer".to_string()))
                    }
                }
                _ => Err(TushareError::ParseError("Date must be a string or number".to_string()))
            }
        }
    }

    impl FromOptionalTushareValue for TushareDate {
        fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
            match value {
                Value::Null => Ok(None),
                _ => TushareDate::from_tushare_value(value).map(Some)
            }
        }
    }

    /// Example struct using custom date formats with procedural macro
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
        
        // UTC datetime format: YYYY-MM-DD HH:MM:SS +ZZZZ
        #[tushare(field = "utc_datetime", date_format = "%Y-%m-%d %H:%M:%S %z")]
        pub utc_datetime: chrono::DateTime<chrono::Utc>,
    }

    pub fn run_example() {
        println!("Custom Date Format Example");
        println!("==========================");
        
        // Test custom date parsing
        use serde_json::json;
        
        // Test CustomDate with different formats
        let date_values = [
            json!("15/03/2024"),  // European format
            json!("03-15-2024"),  // US format
            json!("20240315"),    // YYYYMMDD format
        ];
        
        for date_value in &date_values {
            match chrono::NaiveDate::from_tushare_value(date_value) {
                Ok(date) => println!("Parsed date: {} -> {}", date_value, date),
                Err(e) => println!("Failed to parse {}: {}", date_value, e),
            }
        }
        
        // Test TushareDate with YYYYMMDD format
        let tushare_dates = [
            json!("20240315"),
            json!(20240315),
        ];
        
        for date_value in &tushare_dates {
            match TushareDate::from_tushare_value(date_value) {
                Ok(date) => println!("Parsed Tushare date: {} -> {:?}", date_value, date),
                Err(e) => println!("Failed to parse Tushare date {}: {}", date_value, e),
            }
        }
        
        println!();
        println!("Custom Date Format Attributes:");
        println!("==============================");
        println!("Use #[tushare(date_format = \"...\")]  to specify custom formats:");
        println!("  European: #[tushare(date_format = \"%d/%m/%Y\")]");
        println!("  US:       #[tushare(date_format = \"%m-%d-%Y\")]");
        println!("  German:   #[tushare(date_format = \"%d.%m.%Y\")]");
        println!("  Chinese:  #[tushare(date_format = \"%Y年%m月%d日\")]");
        println!("  Custom:   #[tushare(date_format = \"%Y/%m/%d %H:%M\")]");
        println!();
        println!("This enables precise field-level date format control!");
    }
}

fn main() {
    #[cfg(feature = "chrono")]
    {
        chrono_example::run_example();
    }
    
    #[cfg(not(feature = "chrono"))]
    {
        println!("Custom Date Format Example");
        println!("==========================");
        println!();
        println!("This example requires the 'chrono' feature to be enabled.");
        println!("Run with: cargo run --example custom_date_format_example --features chrono");
        println!();
        println!("The chrono feature enables:");
        println!("  - Custom date format parsing with #[tushare(date_format = \"...\")]");
        println!("  - Support for chrono::NaiveDate, NaiveDateTime, DateTime<Utc>");
        println!("  - Multiple date format support (European, US, German, Chinese, etc.)");
    }
}
