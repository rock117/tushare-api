use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::api::{Api, serialize_api_name};

/// Tushare API request structure
/// 
/// Supports flexible string type usage, allowing direct use of string literals and String variables
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TushareRequest {
    #[serde(serialize_with = "serialize_api_name")]
    pub api_name: Api,
    pub params: HashMap<String, String>,
    pub fields: Vec<String>,
}

impl TushareRequest {
    /// Create a new TushareRequest
    pub fn new<K, V, F, P, Fs>(api_name: Api, params: P, fields: Fs) -> Self
    where
        K: Into<String>,
        V: Into<String>,
        F: Into<String>,
        P: IntoIterator<Item = (K, V)>,
        Fs: IntoIterator<Item = F>,
    {
        let params = params
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        let fields = fields.into_iter().map(|f| f.into()).collect();
        
        Self {
            api_name,
            params,
            fields,
        }
    }
    
    /// Create parameters from string literals
    pub fn with_str_params<const N: usize>(api_name: Api, params: [(&str, &str); N], fields: &[&str]) -> Self {
        let params = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let fields = fields.iter().map(|f| f.to_string()).collect();
        
        Self {
            api_name,
            params,
            fields,
        }
    }
    
    /// Add parameter
    pub fn add_param<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }
    
    /// Add field
    pub fn add_field<F: Into<String>>(mut self, field: F) -> Self {
        self.fields.push(field.into());
        self
    }
}

/// Type alias retained for backward compatibility
pub type TushareRequestString = TushareRequest;

/// Macro for creating parameter HashMap
#[macro_export]
macro_rules! params {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), $value.to_string());
            )*
            map
        }
    };
}

/// Macro for creating fields Vec
#[macro_export]
macro_rules! fields {
    ($($field:expr),* $(,)?) => {
        vec![$($field.to_string()),*]
    };
}

/// More concise builder macro - directly create TushareRequest
#[macro_export]
macro_rules! request {
    ($api:expr, { $($key:expr => $value:expr),* $(,)? }, [ $($field:expr),* $(,)? ]) => {
        TushareRequest {
            api_name: $api,
            params: params!($($key => $value),*),
            fields: fields![$($field),*],
        }
    };
}

/// Tushare API response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TushareResponse {
    pub request_id: String,
    pub code: i32,
    pub msg: Option<String>,
    pub data: Option<TushareData>,
}

/// Tushare API data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TushareData {
    pub fields: Vec<String>,
    pub items: Vec<Vec<serde_json::Value>>,
    pub has_more: bool,
    pub count: i64,
}

/// Generic paginated entity list container
/// 
/// This is the new recommended way to handle paginated API responses.
/// It provides a clear, type-safe interface with built-in pagination metadata.
/// 
/// # Examples
/// 
/// ```rust
/// use tushare_api::{TushareClient, Api, request, TushareEntityList, params, fields, TushareRequest};
/// use tushare_api::DeriveFromTushareData;
/// 
/// #[derive(Debug, Clone, DeriveFromTushareData)]
/// pub struct Stock {
///     pub ts_code: String,
///     pub name: String,
/// }
/// 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = TushareClient::from_env()?;
/// 
/// let stocks: TushareEntityList<Stock> = client.call_api_as(request!(
///     Api::StockBasic, {
///         "list_status" => "L"
///     }, [
///         "ts_code", "name"
///     ]
/// )).await?;
/// 
/// println!("Current page: {} stocks", stocks.len());
/// println!("Total available: {} stocks", stocks.count());
/// println!("Has more pages: {}", stocks.has_more());
/// 
/// for stock in &stocks {
///     println!("{}: {}", stock.ts_code, stock.name);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TushareEntityList<T> {
    /// The actual data items in this page
    pub items: Vec<T>,
    /// Whether there are more pages available
    pub has_more: bool,
    /// Total number of records available across all pages
    pub count: i64,
}

impl<T> TushareEntityList<T> {
    /// Create a new TushareEntityList
    pub fn new(items: Vec<T>, has_more: bool, count: i64) -> Self {
        Self {
            items,
            has_more,
            count,
        }
    }
    
    /// Get the number of items in the current page
    pub fn len(&self) -> usize {
        self.items.len()
    }
    
    /// Check if the current page is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    /// Get items as a slice
    pub fn items(&self) -> &[T] {
        &self.items
    }
    
    /// Get mutable items as a slice
    pub fn items_mut(&mut self) -> &mut [T] {
        &mut self.items
    }
    
    /// Check if there are more pages available
    pub fn has_more(&self) -> bool {
        self.has_more
    }
    
    /// Get the total number of records across all pages
    pub fn count(&self) -> i64 {
        self.count
    }
    
    /// Get an iterator over the items
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.items.iter()
    }
    
    /// Get a mutable iterator over the items
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.items.iter_mut()
    }
    
    /// Convert into the inner `Vec<T>`
    pub fn into_items(self) -> Vec<T> {
        self.items
    }
}

// Implement Deref to allow direct access to Vec<T> methods
impl<T> std::ops::Deref for TushareEntityList<T> {
    type Target = Vec<T>;
    
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

// Implement DerefMut for mutable access
impl<T> std::ops::DerefMut for TushareEntityList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

// Implement IntoIterator for owned values
impl<T> IntoIterator for TushareEntityList<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

// Implement IntoIterator for borrowed values
impl<'a, T> IntoIterator for &'a TushareEntityList<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

// Implement IntoIterator for mutable borrowed values
impl<'a, T> IntoIterator for &'a mut TushareEntityList<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

// Implement From<Vec<T>> for convenience
impl<T> From<Vec<T>> for TushareEntityList<T> {
    fn from(items: Vec<T>) -> Self {
        Self {
            items,
            has_more: false,
            count: 0,
        }
    }
}
