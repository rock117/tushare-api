//! Utility functions for working with Tushare API responses

use crate::error::TushareError;
use crate::types::TushareResponse;
use crate::traits::FromTushareData;
use serde_json::Value;

/// Convert TushareResponse to `Vec<T>` where T implements FromTushareData
pub fn response_to_vec<T: FromTushareData>(response: TushareResponse) -> Result<Vec<T>, TushareError> {
    let mut results = Vec::new();
    if response.data.is_none() {
        return Ok(results);
    }
    let Some(data) = response.data else {
        return Ok(results);
    };
    for item in  data.items {
        let converted = T::from_row(&data.fields, &item)?;
        results.push(converted);
    }
    
    Ok(results)
}

/// Helper function to get field value by name
pub fn get_field_value<'a>(fields: &[String], values: &'a [Value], field_name: &str) -> Result<&'a Value, TushareError> {
    let index = fields.iter()
        .position(|f| f == field_name)
        .ok_or_else(|| TushareError::ParseError(format!("Missing field: {}", field_name)))?;
        
    values.get(index)
        .ok_or_else(|| TushareError::ParseError(format!("Value not found for field: {}", field_name)))
}

/// Helper function to get string field value
pub fn get_string_field(fields: &[String], values: &[Value], field_name: &str) -> Result<String, TushareError> {
    let value = get_field_value(fields, values, field_name)?;
    value.as_str()
        .ok_or_else(|| TushareError::ParseError(format!("Field {} is not a string", field_name)))
        .map(|s| s.to_string())
}

/// Helper function to get optional string field value
pub fn get_optional_string_field(fields: &[String], values: &[Value], field_name: &str) -> Result<Option<String>, TushareError> {
    match get_field_value(fields, values, field_name) {
        Ok(value) => {
            if value.is_null() {
                Ok(None)
            } else {
                value.as_str()
                    .ok_or_else(|| TushareError::ParseError(format!("Field {} is not a string", field_name)))
                    .map(|s| Some(s.to_string()))
            }
        }
        Err(_) => Ok(None), // Field not present
    }
}

/// Helper function to get float field value
pub fn get_float_field(fields: &[String], values: &[Value], field_name: &str) -> Result<f64, TushareError> {
    let value = get_field_value(fields, values, field_name)?;
    value.as_f64()
        .ok_or_else(|| TushareError::ParseError(format!("Field {} is not a number", field_name)))
}

/// Helper function to get optional float field value
pub fn get_optional_float_field(fields: &[String], values: &[Value], field_name: &str) -> Result<Option<f64>, TushareError> {
    match get_field_value(fields, values, field_name) {
        Ok(value) => {
            if value.is_null() {
                Ok(None)
            } else {
                value.as_f64()
                    .ok_or_else(|| TushareError::ParseError(format!("Field {} is not a number", field_name)))
                    .map(Some)
            }
        }
        Err(_) => Ok(None), // Field not present
    }
}

/// Helper function to get integer field value
pub fn get_int_field(fields: &[String], values: &[Value], field_name: &str) -> Result<i64, TushareError> {
    let value = get_field_value(fields, values, field_name)?;
    value.as_i64()
        .ok_or_else(|| TushareError::ParseError(format!("Field {} is not an integer", field_name)))
}

/// Helper function to get optional integer field value
pub fn get_optional_int_field(fields: &[String], values: &[Value], field_name: &str) -> Result<Option<i64>, TushareError> {
    match get_field_value(fields, values, field_name) {
        Ok(value) => {
            if value.is_null() {
                Ok(None)
            } else {
                value.as_i64()
                    .ok_or_else(|| TushareError::ParseError(format!("Field {} is not an integer", field_name)))
                    .map(Some)
            }
        }
        Err(_) => Ok(None), // Field not present
    }
}

/// Helper function to get boolean field value
pub fn get_bool_field(fields: &[String], values: &[Value], field_name: &str) -> Result<bool, TushareError> {
    let value = get_field_value(fields, values, field_name)?;
    value.as_bool()
        .ok_or_else(|| TushareError::ParseError(format!("Field {} is not a boolean", field_name)))
}

/// Helper function to get optional boolean field value
pub fn get_optional_bool_field(fields: &[String], values: &[Value], field_name: &str) -> Result<Option<bool>, TushareError> {
    match get_field_value(fields, values, field_name) {
        Ok(value) => {
            if value.is_null() {
                Ok(None)
            } else {
                value.as_bool()
                    .ok_or_else(|| TushareError::ParseError(format!("Field {} is not a boolean", field_name)))
                    .map(Some)
            }
        }
        Err(_) => Ok(None), // Field not present
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TushareData, TushareResponse};
    use serde_json::json;

    #[derive(Debug, PartialEq)]
    struct TestStock {
        ts_code: String,
        symbol: String,
        name: String,
        price: Option<f64>,
    }

    impl FromTushareData for TestStock {
        fn from_row(fields: &[String], values: &[Value]) -> Result<Self, TushareError> {
            Ok(TestStock {
                ts_code: get_string_field(fields, values, "ts_code")?,
                symbol: get_string_field(fields, values, "symbol")?,
                name: get_string_field(fields, values, "name")?,
                price: get_optional_float_field(fields, values, "price")?,
            })
        }
    }

    #[test]
    fn test_response_to_vec() {
        let response = TushareResponse {
            request_id: "test".to_string(),
            code: 0,
            msg: None,
            data: TushareData {
                fields: vec![
                    "ts_code".to_string(),
                    "symbol".to_string(),
                    "name".to_string(),
                    "price".to_string(),
                ],
                items: vec![
                    vec![
                        json!("000001.SZ"),
                        json!("000001"),
                        json!("平安银行"),
                        json!(10.5),
                    ],
                    vec![
                        json!("000002.SZ"),
                        json!("000002"),
                        json!("万科A"),
                        json!(null),
                    ],
                ],
                has_more: false,
                count: 2,
            },
        };

        let stocks: Vec<TestStock> = response_to_vec(response).unwrap();
        
        assert_eq!(stocks.len(), 2);
        assert_eq!(stocks[0].ts_code, "000001.SZ");
        assert_eq!(stocks[0].symbol, "000001");
        assert_eq!(stocks[0].name, "平安银行");
        assert_eq!(stocks[0].price, Some(10.5));
        
        assert_eq!(stocks[1].ts_code, "000002.SZ");
        assert_eq!(stocks[1].symbol, "000002");
        assert_eq!(stocks[1].name, "万科A");
        assert_eq!(stocks[1].price, None);
    }
}
