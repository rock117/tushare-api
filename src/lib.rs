//! # Tushare API Rust Library
//! 
//! 这是一个用于访问 Tushare API 的通用 Rust 库，提供对 Tushare 各种 API 的访问功能。
//! 
//! ## 使用示例
//! 
//! ```rust,no_run
//! use tushare_api::{TushareClient, TushareRequest, Api, TushareResult};
//! use std::collections::HashMap;
//! 
//! #[tokio::main]
//! async fn main() -> TushareResult<()> {
//!     let client = TushareClient::new("your_token_here");
//!     
//!     let mut params = HashMap::new();
//!     params.insert("list_status".to_string(), "L".to_string());
//!     
//!     let request = TushareRequest {
//!         api_name: Api::StockBasic,
//!         params,
//!         fields: vec!["ts_code".to_string(), "name".to_string()],
//!     };
//!     
//!     let response = client.call_api(request).await?;
//!     println!("获取到 {} 条记录", response.data.items.len());
//!     
//!     Ok(())
//! }
//! ```

// 模块定义
mod error;
mod api;
mod types;
mod client;

// 公开导出
pub use error::{TushareError, TushareResult};
pub use api::Api;
pub use types::{TushareRequest, TushareResponse, TushareData};
pub use client::TushareClient;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_client_creation() {
        let client = TushareClient::new("test_token");
        // 注意：由于 token 字段是私有的，我们只能测试客户端创建是否成功
        // 这里只是验证不会 panic
    }

    #[test]
    fn test_request_creation() {
        let mut params = HashMap::new();
        params.insert("test_param".to_string(), "test_value".to_string());
        
        let request = TushareRequest {
            api_name: Api::Custom("test_api".to_string()),
            params,
            fields: vec!["field1".to_string(), "field2".to_string()],
        };
        
        assert_eq!(request.api_name, Api::Custom("test_api".to_string()));
        assert_eq!(request.fields.len(), 2);
    }

    #[test]
    fn test_api_name() {
        assert_eq!(Api::StockBasic.name(), "stock_basic");
        assert_eq!(Api::FundBasic.name(), "fund_basic");
        assert_eq!(Api::FundDaily.name(), "fund_daily");
        assert_eq!(Api::Custom("custom_api".to_string()).name(), "custom_api");
    }
}
