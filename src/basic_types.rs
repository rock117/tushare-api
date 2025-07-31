//! Basic Rust type implementations for FromTushareValue and FromOptionalTushareValue traits
//!
//! This module provides implementations of the conversion traits for all standard Rust primitive types.

use serde_json::Value;
use crate::error::TushareError;
use crate::traits::{FromTushareValue, FromOptionalTushareValue};

// =============================================================================
// FromTushareValue implementations for basic types
// =============================================================================

impl FromTushareValue for String {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::String(s) => Ok(s.clone()),
            Value::Number(n) => Ok(n.to_string()),
            Value::Bool(b) => Ok(b.to_string()),
            Value::Null => Ok(String::new()),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to String", value
            ))),
        }
    }
}

impl FromTushareValue for f64 {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Number(n) => n.as_f64().ok_or_else(|| {
                TushareError::ParseError(format!("Cannot convert {:?} to f64", n))
            }),
            Value::String(s) => s.parse().map_err(|_| {
                TushareError::ParseError(format!("Cannot parse '{}' as f64", s))
            }),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to f64", value
            ))),
        }
    }
}

impl FromTushareValue for f32 {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Number(n) => n.as_f64().and_then(|f| {
                if f.is_finite() && f >= f32::MIN as f64 && f <= f32::MAX as f64 {
                    Some(f as f32)
                } else {
                    None
                }
            }).ok_or_else(|| {
                TushareError::ParseError(format!("Cannot convert {:?} to f32", n))
            }),
            Value::String(s) => s.parse().map_err(|_| {
                TushareError::ParseError(format!("Cannot parse '{}' as f32", s))
            }),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to f32", value
            ))),
        }
    }
}

impl FromTushareValue for i64 {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Number(n) => n.as_i64().ok_or_else(|| {
                TushareError::ParseError(format!("Cannot convert {:?} to i64", n))
            }),
            Value::String(s) => s.parse().map_err(|_| {
                TushareError::ParseError(format!("Cannot parse '{}' as i64", s))
            }),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to i64", value
            ))),
        }
    }
}

impl FromTushareValue for i32 {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Number(n) => n.as_i64().and_then(|i| {
                if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                    Some(i as i32)
                } else {
                    None
                }
            }).ok_or_else(|| {
                TushareError::ParseError(format!("Cannot convert {:?} to i32", n))
            }),
            Value::String(s) => s.parse().map_err(|_| {
                TushareError::ParseError(format!("Cannot parse '{}' as i32", s))
            }),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to i32", value
            ))),
        }
    }
}

impl FromTushareValue for i16 {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Number(n) => n.as_i64().and_then(|i| {
                if i >= i16::MIN as i64 && i <= i16::MAX as i64 {
                    Some(i as i16)
                } else {
                    None
                }
            }).ok_or_else(|| {
                TushareError::ParseError(format!("Cannot convert {:?} to i16", n))
            }),
            Value::String(s) => s.parse().map_err(|_| {
                TushareError::ParseError(format!("Cannot parse '{}' as i16", s))
            }),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to i16", value
            ))),
        }
    }
}

impl FromTushareValue for i8 {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Number(n) => n.as_i64().and_then(|i| {
                if i >= i8::MIN as i64 && i <= i8::MAX as i64 {
                    Some(i as i8)
                } else {
                    None
                }
            }).ok_or_else(|| {
                TushareError::ParseError(format!("Cannot convert {:?} to i8", n))
            }),
            Value::String(s) => s.parse().map_err(|_| {
                TushareError::ParseError(format!("Cannot parse '{}' as i8", s))
            }),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to i8", value
            ))),
        }
    }
}

impl FromTushareValue for u64 {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Number(n) => n.as_u64().ok_or_else(|| {
                TushareError::ParseError(format!("Cannot convert {:?} to u64", n))
            }),
            Value::String(s) => s.parse().map_err(|_| {
                TushareError::ParseError(format!("Cannot parse '{}' as u64", s))
            }),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to u64", value
            ))),
        }
    }
}

impl FromTushareValue for u32 {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Number(n) => n.as_u64().and_then(|u| {
                if u <= u32::MAX as u64 {
                    Some(u as u32)
                } else {
                    None
                }
            }).ok_or_else(|| {
                TushareError::ParseError(format!("Cannot convert {:?} to u32", n))
            }),
            Value::String(s) => s.parse().map_err(|_| {
                TushareError::ParseError(format!("Cannot parse '{}' as u32", s))
            }),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to u32", value
            ))),
        }
    }
}

impl FromTushareValue for u16 {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Number(n) => n.as_u64().and_then(|u| {
                if u <= u16::MAX as u64 {
                    Some(u as u16)
                } else {
                    None
                }
            }).ok_or_else(|| {
                TushareError::ParseError(format!("Cannot convert {:?} to u16", n))
            }),
            Value::String(s) => s.parse().map_err(|_| {
                TushareError::ParseError(format!("Cannot parse '{}' as u16", s))
            }),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to u16", value
            ))),
        }
    }
}

impl FromTushareValue for u8 {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Number(n) => n.as_u64().and_then(|u| {
                if u <= u8::MAX as u64 {
                    Some(u as u8)
                } else {
                    None
                }
            }).ok_or_else(|| {
                TushareError::ParseError(format!("Cannot convert {:?} to u8", n))
            }),
            Value::String(s) => s.parse().map_err(|_| {
                TushareError::ParseError(format!("Cannot parse '{}' as u8", s))
            }),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to u8", value
            ))),
        }
    }
}

impl FromTushareValue for usize {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Number(n) => n.as_u64().and_then(|u| {
                if u <= usize::MAX as u64 {
                    Some(u as usize)
                } else {
                    None
                }
            }).ok_or_else(|| {
                TushareError::ParseError(format!("Cannot convert {:?} to usize", n))
            }),
            Value::String(s) => s.parse().map_err(|_| {
                TushareError::ParseError(format!("Cannot parse '{}' as usize", s))
            }),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to usize", value
            ))),
        }
    }
}

impl FromTushareValue for isize {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Number(n) => n.as_i64().and_then(|i| {
                if i >= isize::MIN as i64 && i <= isize::MAX as i64 {
                    Some(i as isize)
                } else {
                    None
                }
            }).ok_or_else(|| {
                TushareError::ParseError(format!("Cannot convert {:?} to isize", n))
            }),
            Value::String(s) => s.parse().map_err(|_| {
                TushareError::ParseError(format!("Cannot parse '{}' as isize", s))
            }),
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to isize", value
            ))),
        }
    }
}

impl FromTushareValue for bool {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::Bool(b) => Ok(*b),
            Value::String(s) => match s.to_lowercase().as_str() {
                "true" | "1" | "yes" | "y" => Ok(true),
                "false" | "0" | "no" | "n" | "" => Ok(false),
                _ => Err(TushareError::ParseError(format!(
                    "Cannot parse '{}' as bool", s
                ))),
            },
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(i != 0)
                } else if let Some(f) = n.as_f64() {
                    Ok(f != 0.0)
                } else {
                    Err(TushareError::ParseError(format!(
                        "Cannot convert {:?} to bool", n
                    )))
                }
            },
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to bool", value
            ))),
        }
    }
}

// =============================================================================
// FromOptionalTushareValue implementations for basic types
// =============================================================================

impl FromOptionalTushareValue for String {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            match value {
                Value::String(s) if s.is_empty() => Ok(None),
                _ => String::from_tushare_value(value).map(Some)
            }
        }
    }
}

impl FromOptionalTushareValue for f64 {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            f64::from_tushare_value(value).map(Some)
        }
    }
}

impl FromOptionalTushareValue for f32 {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            f32::from_tushare_value(value).map(Some)
        }
    }
}

impl FromOptionalTushareValue for i64 {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            i64::from_tushare_value(value).map(Some)
        }
    }
}

impl FromOptionalTushareValue for i32 {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            i32::from_tushare_value(value).map(Some)
        }
    }
}

impl FromOptionalTushareValue for i16 {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            i16::from_tushare_value(value).map(Some)
        }
    }
}

impl FromOptionalTushareValue for i8 {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            i8::from_tushare_value(value).map(Some)
        }
    }
}

impl FromOptionalTushareValue for u64 {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            u64::from_tushare_value(value).map(Some)
        }
    }
}

impl FromOptionalTushareValue for u32 {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            u32::from_tushare_value(value).map(Some)
        }
    }
}

impl FromOptionalTushareValue for u16 {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            u16::from_tushare_value(value).map(Some)
        }
    }
}

impl FromOptionalTushareValue for u8 {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            u8::from_tushare_value(value).map(Some)
        }
    }
}

impl FromOptionalTushareValue for usize {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            usize::from_tushare_value(value).map(Some)
        }
    }
}

impl FromOptionalTushareValue for isize {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            isize::from_tushare_value(value).map(Some)
        }
    }
}

impl FromOptionalTushareValue for bool {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            bool::from_tushare_value(value).map(Some)
        }
    }
}

// Additional type implementations

impl FromTushareValue for char {
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
        match value {
            Value::String(s) => {
                let mut chars = s.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) => Ok(c), // Exactly one character
                    (Some(_), Some(_)) => Err(TushareError::ParseError(format!(
                        "String '{}' contains more than one character", s
                    ))),
                    (None, None) => Err(TushareError::ParseError(
                        "Cannot convert empty string to char".to_string()
                    )),
                    (None, Some(_)) => unreachable!("This case is impossible"),
                }
            },
            Value::Number(n) => {
                if let Some(i) = n.as_u64() {
                    if i <= u32::MAX as u64 {
                        char::from_u32(i as u32).ok_or_else(|| {
                            TushareError::ParseError(format!("Invalid Unicode code point: {}", i))
                        })
                    } else {
                        Err(TushareError::ParseError(format!(
                            "Number {} is too large for Unicode code point", i
                        )))
                    }
                } else {
                    Err(TushareError::ParseError(format!(
                        "Cannot convert {:?} to char", n
                    )))
                }
            },
            _ => Err(TushareError::ParseError(format!(
                "Cannot convert {:?} to char", value
            ))),
        }
    }
}

impl FromOptionalTushareValue for char {
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
        if value.is_null() {
            Ok(None)
        } else {
            char::from_tushare_value(value).map(Some)
        }
    }
}

// Note: &str cannot implement FromTushareValue directly because it's a borrowed type
// and the trait requires returning an owned value. Users should use String instead
// and convert to &str as needed. However, we can provide a helper comment:
//
// For &str usage, convert from String:
// let s: String = String::from_tushare_value(value)?;
// let str_ref: &str = &s;
