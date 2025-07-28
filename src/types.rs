use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::api::{Api, serialize_api_name};

/// Tushare API 请求结构体
/// 
/// 支持灵活的字符串类型使用，可以直接使用字符串字面量和 String 变量
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TushareRequest {
    #[serde(serialize_with = "serialize_api_name")]
    pub api_name: Api,
    pub params: HashMap<String, String>,
    pub fields: Vec<String>,
}

impl TushareRequest {
    /// 创建新的 TushareRequest
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
    
    /// 从字符串字面量创建参数
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
    
    /// 添加参数
    pub fn add_param<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }
    
    /// 添加字段
    pub fn add_field<F: Into<String>>(mut self, field: F) -> Self {
        self.fields.push(field.into());
        self
    }
}

/// 为了向后兼容，保留类型别名
pub type TushareRequestString = TushareRequest;

/// 创建参数 HashMap 的宏
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

/// 创建字段 Vec 的宏
#[macro_export]
macro_rules! fields {
    ($($field:expr),* $(,)?) => {
        vec![$($field.to_string()),*]
    };
}

/// 更简洁的构建器宏 - 直接创建 TushareRequest
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

/// Tushare API 响应结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct TushareResponse {
    pub request_id: String,
    pub code: i32,
    pub msg: Option<String>,
    pub data: TushareData,
}

/// Tushare API 数据结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct TushareData {
    pub fields: Vec<String>,
    pub items: Vec<Vec<serde_json::Value>>,
}
