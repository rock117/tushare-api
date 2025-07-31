//! Debug test for custom type support

use tushare_api::{
    FromTushareValue, FromOptionalTushareValue, TushareError,
    DeriveFromTushareData
};
use serde_json::{Value, json};

// Simple custom type
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleDecimal(f64);

impl FromTushareValue for SimpleDecimal {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::String(s) => {
                s.parse::<f64>().map(SimpleDecimal).map_err(|e| {
                    TushareError::ParseError(format!("Failed to parse: {}", e))
                })
            }
            Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    Ok(SimpleDecimal(f))
                } else {
                    Err(TushareError::ParseError("Invalid number".to_string()))
                }
            }
            _ => Err(TushareError::ParseError("Invalid value".to_string()))
        }
    }
}

impl FromOptionalTushareValue for SimpleDecimal {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            SimpleDecimal::from_tushare_value(value).map(Some)
        }
    }
}

// Simple test struct
#[derive(Debug, Clone, DeriveFromTushareData)]
pub struct SimpleStock {
    #[tushare(field = "code")]
    stock_code: String,
    
    #[tushare(field = "price")]
    stock_price: SimpleDecimal,
    
    #[tushare(field = "volume")]
    stock_volume: Option<SimpleDecimal>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tushare_api::traits::FromTushareData;
    
    #[test]
    fn test_simple_custom_type() {
        let fields = vec![
            "code".to_string(),
            "price".to_string(),
            "volume".to_string(),
        ];
        
        let values = vec![
            json!("000001.SZ"),
            json!("10.50"),
            json!("1000000"),
        ];
        
        let stock = SimpleStock::from_row(&fields, &values).unwrap();
        
        assert_eq!(stock.stock_code, "000001.SZ");
        assert_eq!(stock.stock_price.0, 10.50);
        assert!(stock.stock_volume.is_some());
        assert_eq!(stock.stock_volume.unwrap().0, 1000000.0);
    }
    
    #[test]
    fn test_missing_optional_field() {
        let fields = vec![
            "code".to_string(),
            "price".to_string(),
            // volume field is missing
        ];
        
        let values = vec![
            json!("000001.SZ"),
            json!("10.50"),
        ];
        
        let stock = SimpleStock::from_row(&fields, &values).unwrap();
        
        assert_eq!(stock.stock_code, "000001.SZ");
        assert_eq!(stock.stock_price.0, 10.50);
        assert!(stock.stock_volume.is_none());
    }
}

fn main() {
    println!("Debug test for custom types");
}
