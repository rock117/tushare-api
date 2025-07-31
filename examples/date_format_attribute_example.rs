//! Example demonstrating custom date format parsing using #[tushare(date_format = "...")] attribute
//!
//! This example shows how to use the date_format attribute to specify custom date formats
//! directly on struct fields, enabling automatic parsing of various date formats.

#[cfg(feature = "chrono")]
use tushare_api::{DeriveFromTushareData, traits::FromTushareData};

#[cfg(feature = "chrono")]
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct CustomDateFormats {
    // Standard date field (uses built-in format detection)
    #[tushare(field = "trade_date")]
    pub trade_date: chrono::NaiveDate,
    
    // European date format: DD/MM/YYYY
    #[tushare(field = "european_date", date_format = "%d/%m/%Y")]
    pub european_date: chrono::NaiveDate,
    
    // US date format: MM-DD-YYYY
    #[tushare(field = "us_date", date_format = "%m-%d-%Y")]
    pub us_date: chrono::NaiveDate,
    
    // Custom datetime format
    #[tushare(field = "custom_datetime", date_format = "%Y/%m/%d %H:%M")]
    pub custom_datetime: chrono::NaiveDateTime,
    
    // Optional date with custom format
    #[tushare(field = "optional_date", date_format = "%d.%m.%Y")]
    pub optional_date: Option<chrono::NaiveDate>,
    
    // UTC datetime with custom format
    #[tushare(field = "utc_datetime", date_format = "%Y-%m-%d %H:%M:%S %z")]
    pub utc_datetime: chrono::DateTime<chrono::Utc>,
}

#[cfg(feature = "chrono")]
fn main() {
    println!("🗓️  Custom Date Format Attribute Example");
    println!("==========================================");
    
    // Simulate API response data with various date formats
    let fields = vec![
        "trade_date".to_string(),
        "european_date".to_string(),
        "us_date".to_string(),
        "custom_datetime".to_string(),
        "optional_date".to_string(),
        "utc_datetime".to_string(),
    ];
    
    let values = vec![
        serde_json::Value::String("20240315".to_string()),      // YYYYMMDD
        serde_json::Value::String("15/03/2024".to_string()),    // DD/MM/YYYY
        serde_json::Value::String("03-15-2024".to_string()),    // MM-DD-YYYY
        serde_json::Value::String("2024/03/15 14:30".to_string()), // YYYY/MM/DD HH:MM
        serde_json::Value::String("15.03.2024".to_string()),    // DD.MM.YYYY
        serde_json::Value::String("2024-03-15 14:30:00 +0000".to_string()), // UTC with timezone
    ];
    
    // Parse using the custom date formats
    match CustomDateFormats::from_row(&fields, &values) {
        Ok(data) => {
            println!("✅ Successfully parsed all date formats:");
            println!("   Standard trade_date: {}", data.trade_date);
            println!("   European date (DD/MM/YYYY): {}", data.european_date);
            println!("   US date (MM-DD-YYYY): {}", data.us_date);
            println!("   Custom datetime: {}", data.custom_datetime);
            println!("   Optional date: {:?}", data.optional_date);
            println!("   UTC datetime: {}", data.utc_datetime);
        },
        Err(e) => {
            println!("❌ Error parsing dates: {}", e);
        }
    }
    
    println!("\n🎯 Benefits of date_format attribute:");
    println!("   • Specify exact format per field");
    println!("   • No need for wrapper types");
    println!("   • Clear and declarative syntax");
    println!("   • Works with optional fields");
    println!("   • Supports all chrono date/time types");
    
    // Example with different date formats in the same struct
    println!("\n📅 Example with mixed date formats:");
    
    let mixed_fields = vec![
        "european_date".to_string(),
        "us_date".to_string(),
    ];
    
    let mixed_values = vec![
        serde_json::Value::String("31/12/2023".to_string()),    // DD/MM/YYYY
        serde_json::Value::String("12-31-2023".to_string()),    // MM-DD-YYYY
    ];
    
    // Create a subset struct for demonstration
    #[derive(Debug, Clone, DeriveFromTushareData)]
    struct MixedFormats {
        #[tushare(field = "european_date", date_format = "%d/%m/%Y")]
        pub european_date: chrono::NaiveDate,
        
        #[tushare(field = "us_date", date_format = "%m-%d-%Y")]
        pub us_date: chrono::NaiveDate,
    }
    
    match MixedFormats::from_row(&mixed_fields, &mixed_values) {
        Ok(data) => {
            println!("   European: {} -> {}", "31/12/2023", data.european_date);
            println!("   US: {} -> {}", "12-31-2023", data.us_date);
            println!("   Both represent the same date: {}", 
                data.european_date == data.us_date);
        },
        Err(e) => {
            println!("   Error: {}", e);
        }
    }
}

#[cfg(not(feature = "chrono"))]
fn main() {
    println!("❌ This example requires the 'chrono' feature to be enabled.");
    println!("   Run with: cargo run --example date_format_attribute_example --features chrono");
}
