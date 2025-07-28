use serde::{Deserialize, Serialize};

/// Tushare API 枚举类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Api {
    StockBasic,
    FundBasic,
    FundDaily,
    Custom(String),
}

impl Api {
    pub fn name(&self) -> String {
        match self {
            Api::StockBasic => "stock_basic".to_string(),
            Api::FundBasic => "fund_basic".to_string(),
            Api::FundDaily => "fund_daily".to_string(),
            Api::Custom(name) => name.clone(),
        }
    }
}

/// 序列化 Api 枚举为字符串
pub fn serialize_api_name<S>(api: &Api, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&api.name())
}
