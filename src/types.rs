use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::api::{Api, serialize_api_name};

/// Tushare API 请求结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct TushareRequest {
    #[serde(serialize_with = "serialize_api_name")]
    pub api_name: Api,
    pub params: HashMap<String, String>,
    pub fields: Vec<String>,
}

/// Tushare API 响应结构体
#[derive(Debug, Deserialize)]
pub struct TushareResponse {
    pub request_id: String,
    pub code: i32,
    pub msg: Option<String>,
    pub data: TushareData,
}

/// Tushare API 数据结构体
#[derive(Debug, Deserialize)]
pub struct TushareData {
    pub fields: Vec<String>,
    pub items: Vec<Vec<serde_json::Value>>,
}
