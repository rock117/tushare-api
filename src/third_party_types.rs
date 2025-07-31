//! Third-party type implementations for FromTushareValue and FromOptionalTushareValue traits
//!
//! This module provides conditional implementations for popular third-party types
//! when their corresponding feature flags are enabled.

// Conditional imports based on enabled features
#[cfg(any(feature = "rust_decimal", feature = "bigdecimal", feature = "chrono", feature = "uuid"))]
use serde_json::Value;

#[cfg(any(feature = "rust_decimal", feature = "bigdecimal", feature = "chrono", feature = "uuid"))]
use crate::error::TushareError;

#[cfg(any(feature = "rust_decimal", feature = "bigdecimal", feature = "chrono", feature = "uuid"))]
use crate::traits::{FromTushareValue, FromOptionalTushareValue};

// =============================================================================
// rust_decimal::Decimal support
// =============================================================================

#[cfg(feature = "rust_decimal")]
mod rust_decimal_support {
    use super::*;
    use rust_decimal::Decimal;

    impl FromTushareValue for Decimal {
        fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
            match value {
                Value::String(s) => {
                    s.parse().map_err(|e| {
                        TushareError::ParseError(format!("Failed to parse decimal from string '{}': {}", s, e))
                    })
                },
                Value::Number(n) => {
                    if let Some(f) = n.as_f64() {
                        Decimal::try_from(f).map_err(|e| {
                            TushareError::ParseError(format!("Failed to convert number {} to decimal: {}", f, e))
                        })
                    } else {
                        Err(TushareError::ParseError(format!(
                            "Cannot convert number {:?} to decimal", n
                        )))
                    }
                },
                Value::Null => Err(TushareError::ParseError(
                    "Cannot convert null to decimal".to_string()
                )),
                _ => Err(TushareError::ParseError(format!(
                    "Cannot convert {:?} to decimal", value
                ))),
            }
        }
    }

    impl FromOptionalTushareValue for Decimal {
        fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
            if value.is_null() {
                Ok(None)
            } else {
                match value {
                    Value::String(s) if s.is_empty() => Ok(None),
                    _ => Decimal::from_tushare_value(value).map(Some)
                }
            }
        }
    }
}

// =============================================================================
// bigdecimal::BigDecimal support
// =============================================================================

#[cfg(feature = "bigdecimal")]
mod bigdecimal_support {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    impl FromTushareValue for BigDecimal {
        fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
            match value {
                Value::String(s) => {
                    BigDecimal::from_str(s).map_err(|e| {
                        TushareError::ParseError(format!("Failed to parse BigDecimal from string '{}': {}", s, e))
                    })
                },
                Value::Number(n) => {
                    if let Some(f) = n.as_f64() {
                        BigDecimal::try_from(f).map_err(|e| {
                            TushareError::ParseError(format!("Failed to convert number {} to BigDecimal: {}", f, e))
                        })
                    } else {
                        Err(TushareError::ParseError(format!(
                            "Cannot convert number {:?} to BigDecimal", n
                        )))
                    }
                },
                Value::Null => Err(TushareError::ParseError(
                    "Cannot convert null to BigDecimal".to_string()
                )),
                _ => Err(TushareError::ParseError(format!(
                    "Cannot convert {:?} to BigDecimal", value
                ))),
            }
        }
    }

    impl FromOptionalTushareValue for BigDecimal {
        fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
            if value.is_null() {
                Ok(None)
            } else {
                match value {
                    Value::String(s) if s.is_empty() => Ok(None),
                    _ => BigDecimal::from_tushare_value(value).map(Some)
                }
            }
        }
    }
}

// =============================================================================
// chrono date/time types support
// =============================================================================

#[cfg(feature = "chrono")]
mod chrono_support {
    use super::*;
    use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

    impl FromTushareValue for NaiveDate {
        fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
            match value {
                Value::String(s) => {
                    // Try common date formats in order of likelihood
                    let formats = [
                        "%Y%m%d",           // 20240315
                        "%Y-%m-%d",         // 2024-03-15
                        "%Y/%m/%d",         // 2024/03/15
                        "%d/%m/%Y",         // 15/03/2024
                        "%m/%d/%Y",         // 03/15/2024
                        "%d-%m-%Y",         // 15-03-2024
                        "%m-%d-%Y",         // 03-15-2024
                        "%d.%m.%Y",         // 15.03.2024
                        "%Y.%m.%d",         // 2024.03.15
                        "%Y年%m月%d日",      // 2024年03月15日
                        "%Y-%m-%d %H:%M:%S", // 2024-03-15 00:00:00 (extract date part)
                    ];
                    
                    for format in &formats {
                        if let Ok(date) = NaiveDate::parse_from_str(s, format) {
                            return Ok(date);
                        }
                        // Special handling for datetime strings - extract date part
                        if format.contains("%H:%M:%S") {
                            if let Ok(datetime) = NaiveDateTime::parse_from_str(s, format) {
                                return Ok(datetime.date());
                            }
                        }
                    }
                    
                    Err(TushareError::ParseError(format!(
                        "Failed to parse date from string '{}'. Supported formats: YYYYMMDD, YYYY-MM-DD, YYYY/MM/DD, DD/MM/YYYY, MM/DD/YYYY, DD-MM-YYYY, MM-DD-YYYY, DD.MM.YYYY, YYYY.MM.DD, YYYY年MM月DD日", s
                    )))
                },
                Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        // Assume it's YYYYMMDD format
                        let date_str = i.to_string();
                        if date_str.len() == 8 {
                            NaiveDate::parse_from_str(&date_str, "%Y%m%d").map_err(|e| {
                                TushareError::ParseError(format!("Failed to parse date from number {}: {}", i, e))
                            })
                        } else {
                            Err(TushareError::ParseError(format!(
                                "Invalid date number format: {}. Expected YYYYMMDD", i
                            )))
                        }
                    } else {
                        Err(TushareError::ParseError(format!(
                            "Cannot convert number {:?} to date", n
                        )))
                    }
                },
                _ => Err(TushareError::ParseError(format!(
                    "Cannot convert {:?} to date", value
                ))),
            }
        }
    }

    impl FromOptionalTushareValue for NaiveDate {
        fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
            if value.is_null() {
                Ok(None)
            } else {
                match value {
                    Value::String(s) if s.is_empty() => Ok(None),
                    _ => NaiveDate::from_tushare_value(value).map(Some)
                }
            }
        }
    }

    impl FromTushareValue for NaiveDateTime {
        fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
            match value {
                Value::String(s) => {
                    // Try common datetime formats
                    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y%m%d %H:%M:%S") {
                        Ok(dt)
                    } else if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                        Ok(dt)
                    } else if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y/%m/%d %H:%M:%S") {
                        Ok(dt)
                    } else if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
                        Ok(dt)
                    } else {
                        Err(TushareError::ParseError(format!(
                            "Failed to parse datetime from string '{}'. Expected formats: YYYYMMDD HH:MM:SS, YYYY-MM-DD HH:MM:SS, YYYY/MM/DD HH:MM:SS, or YYYY-MM-DDTHH:MM:SS", s
                        )))
                    }
                },
                _ => Err(TushareError::ParseError(format!(
                    "Cannot convert {:?} to datetime", value
                ))),
            }
        }
    }

    impl FromOptionalTushareValue for NaiveDateTime {
        fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
            if value.is_null() {
                Ok(None)
            } else {
                match value {
                    Value::String(s) if s.is_empty() => Ok(None),
                    _ => NaiveDateTime::from_tushare_value(value).map(Some)
                }
            }
        }
    }

    impl FromTushareValue for DateTime<Utc> {
        fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
            match value {
                Value::String(s) => {
                    // Try parsing as RFC3339 first, then fall back to other formats
                    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
                        Ok(dt.with_timezone(&Utc))
                    } else if let Ok(naive_dt) = NaiveDateTime::from_tushare_value(value) {
                        Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc))
                    } else {
                        Err(TushareError::ParseError(format!(
                            "Failed to parse UTC datetime from string '{}'", s
                        )))
                    }
                },
                _ => Err(TushareError::ParseError(format!(
                    "Cannot convert {:?} to UTC datetime", value
                ))),
            }
        }
    }

    impl FromOptionalTushareValue for DateTime<Utc> {
        fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
            if value.is_null() {
                Ok(None)
            } else {
                match value {
                    Value::String(s) if s.is_empty() => Ok(None),
                    _ => DateTime::<Utc>::from_tushare_value(value).map(Some)
                }
            }
        }
    }
}

// =============================================================================
// uuid::Uuid support
// =============================================================================

#[cfg(feature = "uuid")]
mod uuid_support {
    use super::*;
    use uuid::Uuid;

    impl FromTushareValue for Uuid {
        fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
            match value {
                Value::String(s) => {
                    Uuid::parse_str(s).map_err(|e| {
                        TushareError::ParseError(format!("Failed to parse UUID from string '{}': {}", s, e))
                    })
                },
                _ => Err(TushareError::ParseError(format!(
                    "Cannot convert {:?} to UUID", value
                ))),
            }
        }
    }

    impl FromOptionalTushareValue for Uuid {
        fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
            if value.is_null() {
                Ok(None)
            } else {
                match value {
                    Value::String(s) if s.is_empty() => Ok(None),
                    _ => Uuid::from_tushare_value(value).map(Some)
                }
            }
        }
    }
}
