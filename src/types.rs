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
    pub data: TushareData,
}

/// Tushare API data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TushareData {
    pub fields: Vec<String>,
    pub items: Vec<Vec<serde_json::Value>>,
    pub has_more: bool,
    pub count: i64,
}
