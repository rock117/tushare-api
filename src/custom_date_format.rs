//! Custom date format implementations for FromTushareValueWithFormat trait
//!
//! This module provides implementations for chrono date/time types that support
//! custom date format parsing through the `#[tushare(date_format = "...")]` attribute.

#[cfg(feature = "chrono")]
use crate::traits::FromTushareValueWithFormat;
#[cfg(feature = "chrono")]
use crate::error::TushareError;

#[cfg(feature = "chrono")]
impl FromTushareValueWithFormat for chrono::NaiveDate {
    fn from_tushare_value_with_format(
        value: &serde_json::Value,
        format: &str,
    ) -> Result<Self, TushareError> {
        match value {
            serde_json::Value::String(s) => {
                // Try parsing with the custom format
                chrono::NaiveDate::parse_from_str(s, format)
                    .map_err(|e| TushareError::ParseError(
                        format!("Failed to parse date '{}' with format '{}': {}", s, format, e)
                    ))
            }
            serde_json::Value::Number(n) => {
                // For numeric values, try to parse as YYYYMMDD first, then use custom format
                if let Some(i) = n.as_i64() {
                    let date_str = i.to_string();
                    
                    // Try YYYYMMDD format first for numeric values
                    if date_str.len() == 8 {
                        if let Ok(date) = chrono::NaiveDate::parse_from_str(&date_str, "%Y%m%d") {
                            return Ok(date);
                        }
                    }
                    
                    // Fall back to custom format
                    chrono::NaiveDate::parse_from_str(&date_str, format)
                        .map_err(|e| TushareError::ParseError(
                            format!("Failed to parse numeric date '{}' with format '{}': {}", date_str, format, e)
                        ))
                } else {
                    Err(TushareError::ParseError(
                        format!("Invalid numeric date value: {}", n)
                    ))
                }
            }
            _ => Err(TushareError::ParseError(
                format!("Expected string or number for date parsing, got: {:?}", value)
            )),
        }
    }
}

#[cfg(feature = "chrono")]
impl FromTushareValueWithFormat for chrono::NaiveDateTime {
    fn from_tushare_value_with_format(
        value: &serde_json::Value,
        format: &str,
    ) -> Result<Self, TushareError> {
        match value {
            serde_json::Value::String(s) => {
                // Try parsing with the custom format
                chrono::NaiveDateTime::parse_from_str(s, format)
                    .map_err(|e| TushareError::ParseError(
                        format!("Failed to parse datetime '{}' with format '{}': {}", s, format, e)
                    ))
            }
            serde_json::Value::Number(n) => {
                // For numeric values, convert to string and try parsing
                let datetime_str = n.to_string();
                chrono::NaiveDateTime::parse_from_str(&datetime_str, format)
                    .map_err(|e| TushareError::ParseError(
                        format!("Failed to parse numeric datetime '{}' with format '{}': {}", datetime_str, format, e)
                    ))
            }
            _ => Err(TushareError::ParseError(
                format!("Expected string or number for datetime parsing, got: {:?}", value)
            )),
        }
    }
}

#[cfg(feature = "chrono")]
impl FromTushareValueWithFormat for chrono::DateTime<chrono::Utc> {
    fn from_tushare_value_with_format(
        value: &serde_json::Value,
        format: &str,
    ) -> Result<Self, TushareError> {
        match value {
            serde_json::Value::String(s) => {
                // Try parsing with the custom format
                chrono::DateTime::parse_from_str(s, format)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .map_err(|e| TushareError::ParseError(
                        format!("Failed to parse UTC datetime '{}' with format '{}': {}", s, format, e)
                    ))
            }
            serde_json::Value::Number(n) => {
                // For numeric values, convert to string and try parsing
                let datetime_str = n.to_string();
                chrono::DateTime::parse_from_str(&datetime_str, format)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .map_err(|e| TushareError::ParseError(
                        format!("Failed to parse numeric UTC datetime '{}' with format '{}': {}", datetime_str, format, e)
                    ))
            }
            _ => Err(TushareError::ParseError(
                format!("Expected string or number for UTC datetime parsing, got: {:?}", value)
            )),
        }
    }
}
