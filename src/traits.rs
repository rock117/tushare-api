//! Traits for Tushare API data conversion
//!
//! This module contains traits that define how to convert Tushare API response data
//! into Rust structs. The main trait is `FromTushareData` which can be implemented
//! manually or automatically using the derive macro from `tushare-derive`.

use crate::error::TushareError;
use crate::types::{TushareResponse, TushareEntityList};
use serde_json::Value;

/// Trait for converting individual JSON values to custom types
/// 
/// This trait allows users to define how to convert a single JSON value
/// from Tushare API responses into their custom types. It's designed to
/// work with the procedural macro system for automatic field conversion.
/// 
/// # Example
/// 
/// ```rust
/// use tushare_api::{FromTushareValue, TushareError};
/// use serde_json::Value;
/// 
/// // Custom type example
/// #[derive(Debug, Clone, PartialEq)]
/// struct CustomDecimal(f64);
/// 
/// impl std::str::FromStr for CustomDecimal {
///     type Err = std::num::ParseFloatError;
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         s.parse::<f64>().map(CustomDecimal)
///     }
/// }
/// 
/// impl FromTushareValue for CustomDecimal {
///     fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
///         match value {
///             Value::String(s) => s.parse().map_err(|e| {
///                 TushareError::ParseError(format!("Failed to parse decimal: {}", e))
///             }),
///             Value::Number(n) => {
///                 if let Some(f) = n.as_f64() {
///                     Ok(CustomDecimal(f))
///                 } else {
///                     Err(TushareError::ParseError("Invalid number format".to_string()))
///                 }
///             }
///             _ => Err(TushareError::ParseError("Value is not a valid decimal".to_string()))
///         }
///     }
/// }
/// ```
pub trait FromTushareValue: Sized {
    /// Convert a JSON value to this type
    /// 
    /// # Arguments
    /// 
    /// * `value` - The JSON value to convert
    fn from_tushare_value(value: &Value) -> Result<Self, TushareError>;
}

// Note: Basic Rust type implementations have been moved to src/basic_types.rs
// This includes implementations for: String, f64, f32, i64, i32, i16, i8, u64, u32, u16, u8, usize, isize, bool

/// Trait for converting optional JSON values to custom types
/// 
/// This trait handles the conversion of potentially null or missing JSON values
/// to optional custom types. It's designed to work with the procedural macro
/// system for automatic optional field conversion.
/// 
/// # Example
/// 
/// ```rust
/// use tushare_api::{FromTushareValue, FromOptionalTushareValue, TushareError};
/// use serde_json::Value;
/// 
/// // Custom type example (same as above)
/// #[derive(Debug, Clone, PartialEq)]
/// struct CustomDecimal(f64);
/// 
/// impl std::str::FromStr for CustomDecimal {
///     type Err = std::num::ParseFloatError;
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         s.parse::<f64>().map(CustomDecimal)
///     }
/// }
/// 
/// impl FromTushareValue for CustomDecimal {
///     fn from_tushare_value(value: &Value) -> Result<Self, TushareError> {
///         match value {
///             Value::String(s) => s.parse().map_err(|e| {
///                 TushareError::ParseError(format!("Failed to parse decimal: {}", e))
///             }),
///             Value::Number(n) => {
///                 if let Some(f) = n.as_f64() {
///                     Ok(CustomDecimal(f))
///                 } else {
///                     Err(TushareError::ParseError("Invalid number format".to_string()))
///                 }
///             }
///             _ => Err(TushareError::ParseError("Value is not a valid decimal".to_string()))
///         }
///     }
/// }
/// 
/// impl FromOptionalTushareValue for CustomDecimal {
///     fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError> {
///         if value.is_null() {
///             Ok(None)
///         } else {
///             match value {
///                 Value::String(s) if s.is_empty() => Ok(None),
///                 _ => CustomDecimal::from_tushare_value(value).map(Some)
///             }
///         }
///     }
/// }
/// ```
pub trait FromOptionalTushareValue: Sized {
    /// Convert an optional JSON value to this type
    /// 
    /// # Arguments
    /// 
    /// * `value` - The JSON value to convert (may be null)
    fn from_optional_tushare_value(value: &Value) -> Result<Option<Self>, TushareError>;
}

// Note: Basic Rust type implementations for FromOptionalTushareValue have been moved to src/basic_types.rs

/// Trait for converting Tushare API response data into Rust structs
/// 
/// This trait defines how to convert a single row of data from a Tushare API response
/// into a Rust struct. It can be implemented manually or automatically using the
/// `#[derive(FromTushareData)]` macro from the `tushare-derive` crate.
/// 
/// # Example
/// 
/// ```rust
/// use tushare_api::traits::FromTushareData;
/// use tushare_api::error::TushareError;
/// use serde_json::Value;
/// 
/// struct Stock {
///     ts_code: String,
///     name: String,
/// }
/// 
/// impl FromTushareData for Stock {
///     fn from_row(fields: &[String], values: &[Value]) -> Result<Self, TushareError> {
///         // Manual implementation
///         let ts_code_idx = fields.iter().position(|f| f == "ts_code")
///             .ok_or_else(|| TushareError::ParseError("Missing ts_code field".to_string()))?;
///         let name_idx = fields.iter().position(|f| f == "name")
///             .ok_or_else(|| TushareError::ParseError("Missing name field".to_string()))?;
///             
///         Ok(Stock {
///             ts_code: values[ts_code_idx].as_str()
///                 .ok_or_else(|| TushareError::ParseError("Invalid ts_code".to_string()))?
///                 .to_string(),
///             name: values[name_idx].as_str()
///                 .ok_or_else(|| TushareError::ParseError("Invalid name".to_string()))?
///                 .to_string(),
///         })
///     }
/// }
/// ```
/// 
/// # Using the derive macro
/// 
/// For most use cases, you can use the derive macro instead of manual implementation:
/// 
/// ```rust
/// use tushare_api::DeriveFromTushareData;
/// 
/// #[derive(Debug, Clone, DeriveFromTushareData)]
/// pub struct Stock {
///     ts_code: String,
///     name: String,
///     area: Option<String>,
/// }
/// ```
pub trait FromTushareData: Sized {
    /// Convert a single row of data to this type
    /// 
    /// # Arguments
    /// 
    /// * `fields` - Field names from the response
    /// * `values` - Values for this row
    fn from_row(fields: &[String], values: &[Value]) -> Result<Self, TushareError>;
}

/// Implementation of TryFrom<TushareResponse> for TushareEntityList<T>
/// 
/// This allows automatic conversion from API responses to typed entity lists.
/// It extracts pagination metadata and converts each data row to the target type T.
impl<T> TryFrom<TushareResponse> for TushareEntityList<T>
where
    T: FromTushareData,
{
    type Error = TushareError;
    
    fn try_from(response: TushareResponse) -> Result<Self, Self::Error> {
        let data = response.data;
        let mut items = Vec::new();
        
        // Convert each row to the target type
        for row in &data.items {
            let item = T::from_row(&data.fields, row)?;
            items.push(item);
        }
        
        Ok(TushareEntityList::new(
            items,
            data.has_more,
            data.count,
        ))
    }
}

/// Helper function for parsing values with custom date format (non-optional types)
/// 
/// This function is used by the procedural macro when a `date_format` attribute is specified.
/// It attempts to parse the value using the custom format for supported chrono types.
/// 
/// # Arguments
/// 
/// * `value` - The JSON value to parse
/// * `format` - The custom date format string (e.g., "%d/%m/%Y")
/// 
/// # Returns
/// 
/// Returns the parsed value of type T or an error if parsing fails.
pub fn from_tushare_value_with_date_format<T>(
    value: &serde_json::Value,
    format: &str,
) -> Result<T, crate::error::TushareError>
where
    T: FromTushareValueWithFormat,
{
    T::from_tushare_value_with_format(value, format)
}

/// Helper function for parsing optional values with custom date format
/// 
/// This function is used by the procedural macro when a `date_format` attribute is specified
/// for optional fields. It handles null/empty values gracefully.
/// 
/// # Arguments
/// 
/// * `value` - The JSON value to parse (may be null)
/// * `format` - The custom date format string (e.g., "%d/%m/%Y")
/// 
/// # Returns
/// 
/// Returns Some(parsed_value) for valid values, None for null/empty, or an error for invalid formats.
pub fn from_optional_tushare_value_with_date_format<T>(
    value: &serde_json::Value,
    format: &str,
) -> Result<Option<T>, crate::error::TushareError>
where
    T: FromTushareValueWithFormat,
{
    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::String(s) if s.is_empty() => Ok(None),
        _ => Ok(Some(T::from_tushare_value_with_format(value, format)?)),
    }
}

/// Trait for types that support custom date format parsing
/// 
/// This trait is implemented for chrono date/time types to enable
/// custom format parsing through the `#[tushare(date_format = "...")]` attribute.
pub trait FromTushareValueWithFormat: Sized {
    /// Parse a value using a custom date format
    /// 
    /// # Arguments
    /// 
    /// * `value` - The JSON value to parse
    /// * `format` - The custom date format string
    /// 
    /// # Returns
    /// 
    /// Returns the parsed value or an error if parsing fails.
    fn from_tushare_value_with_format(
        value: &serde_json::Value,
        format: &str,
    ) -> Result<Self, crate::error::TushareError>;
}
