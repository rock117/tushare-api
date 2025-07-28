//! # Tushare API Rust Library
//! 
//! 这是一个用于访问 Tushare API 的通用 Rust 库，提供对 Tushare 各种 API 的访问功能。
//! 
//! # 基本使用方法
//! 
//! ```rust
//! use tushare_api::{TushareClient, TushareRequest, Api, params, fields};
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = TushareClient::new("your_token_here");
//!     
//!     // 现在可以直接使用字符串字面量！
//!     let req = TushareRequest {
//!         api_name: Api::StockBasic,
//!         params: params!("list_status" => "L"),
//!         fields: fields!["ts_code", "name"],
//!     };
//!     
//!     let response = client.call_api(req).await?;
//!     println!("Response: {:?}", response);
//! #   Ok(())
//! # }
//! ```

// 模块定义
mod error;
mod api;
mod types;
mod client;
mod logging;

// 公开导出
pub use error::{TushareError, TushareResult};
pub use api::Api;
pub use types::{TushareRequest, TushareRequestString, TushareResponse, TushareData};
pub use client::{TushareClient, TushareClientBuilder};
pub use logging::{LogLevel, LogConfig, Logger};

// 宏通过 #[macro_export] 已经在 crate 根部可用
// 无需重新导出

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_client_creation() {
        let client = TushareClient::new("test_token");
        // 注意：由于 token 字段是私有的，我们只能测试客户端创建是否成功
        // 这里只是验证不会 panic
    }

    #[test]
    fn test_request_creation() {
        let _client = TushareClient::new("test_token");
        
        // 使用新的简化方式
        let request = TushareRequest {
            api_name: Api::StockBasic,
            params: params!("test_param" => "test_value"),
            fields: fields!["field1", "field2"],
        };
        
        assert_eq!(request.api_name, Api::StockBasic);
        assert_eq!(request.fields.len(), 2);
    }

    #[test]
    fn test_request_macro() {
        // 测试 request! 宏的基本功能
        let request = request!(Api::StockBasic, {
            "list_status" => "L",
            "exchange" => "SSE"
        }, [
            "ts_code", "name", "industry"
        ]);
        
        assert_eq!(request.api_name, Api::StockBasic);
        assert_eq!(request.params.len(), 2);
        assert_eq!(request.params.get("list_status"), Some(&"L".to_string()));
        assert_eq!(request.params.get("exchange"), Some(&"SSE".to_string()));
        assert_eq!(request.fields.len(), 3);
        assert_eq!(request.fields[0], "ts_code");
        assert_eq!(request.fields[1], "name");
        assert_eq!(request.fields[2], "industry");
    }

    #[test]
    fn test_request_macro_empty() {
        // 测试 request! 宏处理空参数和字段
        let request = request!(Api::FundBasic, {}, []);
        
        assert_eq!(request.api_name, Api::FundBasic);
        assert_eq!(request.params.len(), 0);
        assert_eq!(request.fields.len(), 0);
    }

    #[test]
    fn test_request_macro_single_items() {
        // 测试 request! 宏处理单个参数和字段
        let request = request!(Api::Custom("test_api".to_string()), {
            "param1" => "value1"
        }, [
            "field1"
        ]);
        
        assert_eq!(request.api_name, Api::Custom("test_api".to_string()));
        assert_eq!(request.params.len(), 1);
        assert_eq!(request.params.get("param1"), Some(&"value1".to_string()));
        assert_eq!(request.fields.len(), 1);
        assert_eq!(request.fields[0], "field1");
    }

    #[test]
    fn test_api_name() {
        assert_eq!(Api::StockBasic.name(), "stock_basic");
        assert_eq!(Api::FundBasic.name(), "fund_basic");
        assert_eq!(Api::FundDaily.name(), "fund_daily");
        assert_eq!(Api::Custom("custom_api".to_string()).name(), "custom_api");
    }
}
