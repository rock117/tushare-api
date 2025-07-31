//! Example demonstrating custom type support with rust_decimal
//! 
//! This example shows how to implement FromTushareValue and FromOptionalTushareValue
//! traits for custom types like rust_decimal::Decimal, allowing automatic conversion
//! in the procedural macro system.

use tushare_api::{
    TushareClient, Api, TushareRequest, TushareEntityList, 
    FromTushareValue, FromOptionalTushareValue, TushareError,
    DeriveFromTushareData, params, fields
};
use serde_json::Value;

// For this example, we'll simulate rust_decimal::Decimal with a simple wrapper
// In real usage, you would use the actual rust_decimal crate
#[derive(Debug, Clone, PartialEq)]
pub struct Decimal(f64);

impl Decimal {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
    
    pub fn to_f64(&self) -> f64 {
        self.0
    }
}

impl std::str::FromStr for Decimal {
    type Err = std::num::ParseFloatError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f64>().map(Decimal::new)
    }
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Implement FromTushareValue for Decimal
impl FromTushareValue for Decimal {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::String(s) => {
                if s.is_empty() {
                    Err(TushareError::ParseError("Empty string cannot be converted to Decimal".to_string()))
                } else {
                    s.parse().map_err(|e| {
                        TushareError::ParseError(format!("Failed to parse decimal from string '{}': {}", s, e))
                    })
                }
            }
            Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    Ok(Decimal::new(f))
                } else {
                    Err(TushareError::ParseError("Invalid number format for Decimal".to_string()))
                }
            }
            Value::Null => {
                Err(TushareError::ParseError("Cannot convert null to Decimal (use Option<Decimal> for nullable fields)".to_string()))
            }
            _ => Err(TushareError::ParseError("Value is not a valid decimal".to_string()))
        }
    }
}

// Implement FromOptionalTushareValue for Decimal
impl FromOptionalTushareValue for Decimal {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        match value {
            Value::Null => Ok(None),
            Value::String(s) if s.is_empty() => Ok(None),
            _ => Decimal::from_tushare_value(value).map(Some)
        }
    }
}

// Define a struct that uses custom Decimal types
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct StockPrice {
    #[tushare(field = "ts_code")]
    ts_code: String,
    
    #[tushare(field = "trade_date")]
    trade_date: String,
    
    #[tushare(field = "open")]
    open_price: Decimal,
    
    #[tushare(field = "high")]
    high_price: Decimal,
    
    #[tushare(field = "low")]
    low_price: Decimal,
    
    #[tushare(field = "close")]
    close_price: Decimal,
    
    #[tushare(field = "vol")]
    volume: Option<Decimal>,
    
    #[tushare(field = "amount")]
    amount: Option<Decimal>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    env_logger::init();
    
    // Create client from environment variable
    let client = TushareClient::from_env()?;
    
    // Create request for daily stock data
    let request = TushareRequest::new(
        Api::Daily,
        params![
            "ts_code" => "000001.SZ",
            "start_date" => "20240101",
            "end_date" => "20240131"
        ],
        fields![
            "ts_code", "trade_date", "open", "high", "low", "close", "vol", "amount"
        ]
    );
    
    println!("Fetching daily stock data with custom Decimal types...");
    
    // Call API and get typed response with automatic conversion
    let stock_prices: TushareEntityList<StockPrice> = client.call_api_as(request).await?;
    
    println!("Retrieved {} stock price records", stock_prices.len());
    println!("Has more data: {}", stock_prices.has_more());
    println!("Total count: {}", stock_prices.count());
    
    // Display first few records
    for (i, price) in stock_prices.iter().take(5).enumerate() {
        println!("\nRecord {}:", i + 1);
        println!("  Stock Code: {}", price.ts_code);
        println!("  Trade Date: {}", price.trade_date);
        println!("  Open Price: {}", price.open_price);
        println!("  High Price: {}", price.high_price);
        println!("  Low Price: {}", price.low_price);
        println!("  Close Price: {}", price.close_price);
        
        if let Some(vol) = &price.volume {
            println!("  Volume: {}", vol);
        }
        
        if let Some(amount) = &price.amount {
            println!("  Amount: {}", amount);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_decimal_from_tushare_value() {
        // Test string conversion
        let value = json!("123.45");
        let decimal = Decimal::from_tushare_value(&value).unwrap();
        assert_eq!(decimal.to_f64(), 123.45);
        
        // Test number conversion
        let value = json!(67.89);
        let decimal = Decimal::from_tushare_value(&value).unwrap();
        assert_eq!(decimal.to_f64(), 67.89);
        
        // Test error cases
        let value = json!(null);
        assert!(Decimal::from_tushare_value(&value).is_err());
        
        let value = json!("");
        assert!(Decimal::from_tushare_value(&value).is_err());
    }
    
    #[test]
    fn test_decimal_from_optional_tushare_value() {
        // Test null conversion
        let value = json!(null);
        let result = Decimal::from_optional_tushare_value(&value).unwrap();
        assert!(result.is_none());
        
        // Test empty string conversion
        let value = json!("");
        let result = Decimal::from_optional_tushare_value(&value).unwrap();
        assert!(result.is_none());
        
        // Test valid value conversion
        let value = json!("123.45");
        let result = Decimal::from_optional_tushare_value(&value).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().to_f64(), 123.45);
    }
    
    #[test]
    fn test_stock_price_conversion() {
        use tushare_api::traits::FromTushareData;
        
        // Use the API field names as defined in the tushare attributes
        let fields = vec![
            "ts_code".to_string(),
            "trade_date".to_string(),
            "open".to_string(),
            "high".to_string(),
            "low".to_string(),
            "close".to_string(),
            "vol".to_string(),
            "amount".to_string(),
        ];
        
        let values = vec![
            json!("000001.SZ"),
            json!("20240115"),
            json!("10.50"),
            json!("10.80"),
            json!("10.30"),
            json!("10.75"),
            json!("1000000"),
            json!("10750000"),
        ];
        
        let stock_price = StockPrice::from_row(&fields, &values).unwrap();
        
        assert_eq!(stock_price.ts_code, "000001.SZ");
        assert_eq!(stock_price.trade_date, "20240115");
        assert_eq!(stock_price.open_price.to_f64(), 10.50);
        assert_eq!(stock_price.high_price.to_f64(), 10.80);
        assert_eq!(stock_price.low_price.to_f64(), 10.30);
        assert_eq!(stock_price.close_price.to_f64(), 10.75);
        assert!(stock_price.volume.is_some());
        assert_eq!(stock_price.volume.unwrap().to_f64(), 1000000.0);
        assert!(stock_price.amount.is_some());
        assert_eq!(stock_price.amount.unwrap().to_f64(), 10750000.0);
    }
}
