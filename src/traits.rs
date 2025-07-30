//! Traits for Tushare API data conversion
//!
//! This module contains traits that define how to convert Tushare API response data
//! into Rust structs. The main trait is `FromTushareData` which can be implemented
//! manually or automatically using the derive macro from `tushare-derive`.

use crate::error::TushareError;
use crate::types::{TushareResponse, TushareEntityList};
use serde_json::Value;

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
