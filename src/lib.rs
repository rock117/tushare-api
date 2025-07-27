//! # Tushare API Rust Library
//! 
//! 这是一个用于访问 Tushare API 的 Rust 库，提供获取 A 股股票数据的功能。
//! 
//! ## 使用示例
//! 
//! ```rust,no_run
//! use tushare_api::{TushareClient, Stock};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = TushareClient::new("your_token_here");
//!     let stocks = client.get_stock_list().await?;
//!     
//!     for stock in stocks.iter().take(10) {
//!         println!("{}: {}", stock.ts_code, stock.name);
//!     }
//!     
//!     Ok(())
//! }
//! ```

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tushare API 请求结构体
#[derive(Debug, Serialize, Deserialize)]
struct TushareRequest {
    api_name: String,
    token: String,
    params: HashMap<String, String>,
    fields: String,
}

/// Tushare API 响应结构体
#[derive(Debug, Deserialize)]
struct TushareResponse {
    request_id: String,
    code: i32,
    msg: Option<String>,
    data: TushareData,
}

/// Tushare API 数据结构体
#[derive(Debug, Deserialize)]
struct TushareData {
    fields: Vec<String>,
    items: Vec<Vec<serde_json::Value>>,
}

/// 股票信息结构体
#[derive(Debug, Clone)]
pub struct Stock {
    /// 股票代码（如：000001.SZ）
    pub ts_code: String,
    /// 股票简称（如：平安银行）
    pub symbol: String,
    /// 公司名称
    pub name: String,
    /// 地区
    pub area: String,
    /// 行业
    pub industry: String,
    /// 市场类型
    pub market: String,
    /// 上市日期
    pub list_date: String,
}

impl Stock {
    /// 从 API 返回的数组数据创建 Stock 实例
    fn from_vec(fields: &[String], values: &[serde_json::Value]) -> Option<Self> {
        let mut stock = Stock {
            ts_code: String::new(),
            symbol: String::new(),
            name: String::new(),
            area: String::new(),
            industry: String::new(),
            market: String::new(),
            list_date: String::new(),
        };

        for (i, field) in fields.iter().enumerate() {
            if let Some(value) = values.get(i) {
                let str_value = value.as_str().unwrap_or("").to_string();
                match field.as_str() {
                    "ts_code" => stock.ts_code = str_value,
                    "symbol" => stock.symbol = str_value,
                    "name" => stock.name = str_value,
                    "area" => stock.area = str_value,
                    "industry" => stock.industry = str_value,
                    "market" => stock.market = str_value,
                    "list_date" => stock.list_date = str_value,
                    _ => {}
                }
            }
        }
        Some(stock)
    }
}

/// Tushare API 客户端
pub struct TushareClient {
    token: String,
    client: Client,
}

impl TushareClient {
    /// 创建新的 Tushare 客户端
    /// 
    /// # 参数
    /// 
    /// * `token` - Tushare API Token
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use tushare_api::TushareClient;
    /// 
    /// let client = TushareClient::new("your_token_here");
    /// ```
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
            client: Client::new(),
        }
    }

    /// 获取 A 股股票列表
    /// 
    /// # 返回值
    /// 
    /// 返回 `Result<Vec<Stock>, Box<dyn std::error::Error>>`，包含所有上市状态的 A 股股票信息
    /// 
    /// # 示例
    /// 
    /// ```rust,no_run
    /// use tushare_api::TushareClient;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = TushareClient::new("your_token_here");
    ///     let stocks = client.get_stock_list().await?;
    ///     println!("获取到 {} 只股票", stocks.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_stock_list(&self) -> Result<Vec<Stock>, Box<dyn std::error::Error>> {
        let mut params = HashMap::new();
        params.insert("list_status".to_string(), "L".to_string()); // L表示上市状态
        
        let request = TushareRequest {
            api_name: "stock_basic".to_string(),
            token: self.token.clone(),
            params,
            fields: "ts_code,symbol,name,area,industry,market,list_date".to_string(),
        };

        let response = self.client
            .post("http://api.tushare.pro")
            .json(&request)
            .send()
            .await?;

        let tushare_response: TushareResponse = response.json().await?;
        
        if tushare_response.code != 0 {
            return Err(format!("API错误: {}", tushare_response.msg.unwrap_or("未知错误".to_string())).into());
        }

        let mut stocks = Vec::new();
        for item in &tushare_response.data.items {
            if let Some(stock) = Stock::from_vec(&tushare_response.data.fields, item) {
                stocks.push(stock);
            }
        }

        Ok(stocks)
    }

    /// 根据股票代码获取特定股票信息
    /// 
    /// # 参数
    /// 
    /// * `ts_code` - 股票代码（如：000001.SZ）
    /// 
    /// # 返回值
    /// 
    /// 返回 `Result<Option<Stock>, Box<dyn std::error::Error>>`
    pub async fn get_stock_by_code(&self, ts_code: &str) -> Result<Option<Stock>, Box<dyn std::error::Error>> {
        let mut params = HashMap::new();
        params.insert("ts_code".to_string(), ts_code.to_string());
        
        let request = TushareRequest {
            api_name: "stock_basic".to_string(),
            token: self.token.clone(),
            params,
            fields: "ts_code,symbol,name,area,industry,market,list_date".to_string(),
        };

        let response = self.client
            .post("http://api.tushare.pro")
            .json(&request)
            .send()
            .await?;

        let tushare_response: TushareResponse = response.json().await?;
        
        if tushare_response.code != 0 {
            return Err(format!("API错误: {}", tushare_response.msg.unwrap_or("未知错误".to_string())).into());
        }

        if let Some(item) = tushare_response.data.items.first() {
            Ok(Stock::from_vec(&tushare_response.data.fields, item))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_creation() {
        let fields = vec![
            "ts_code".to_string(),
            "symbol".to_string(),
            "name".to_string(),
            "area".to_string(),
            "industry".to_string(),
            "market".to_string(),
            "list_date".to_string(),
        ];
        
        let values = vec![
            serde_json::Value::String("000001.SZ".to_string()),
            serde_json::Value::String("000001".to_string()),
            serde_json::Value::String("平安银行".to_string()),
            serde_json::Value::String("深圳".to_string()),
            serde_json::Value::String("银行".to_string()),
            serde_json::Value::String("主板".to_string()),
            serde_json::Value::String("19910403".to_string()),
        ];

        let stock = Stock::from_vec(&fields, &values).unwrap();
        assert_eq!(stock.ts_code, "000001.SZ");
        assert_eq!(stock.name, "平安银行");
    }

    #[test]
    fn test_client_creation() {
        let client = TushareClient::new("test_token");
        assert_eq!(client.token, "test_token");
    }
}
