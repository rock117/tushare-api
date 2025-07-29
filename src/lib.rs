//! # Tushare API Rust Library
//! 
//! This is a universal Rust library for accessing Tushare API, providing access to various Tushare APIs.
//! 
//! # Basic Usage
//! 
//! ```rust
//! use tushare_api::{TushareClient, TushareRequest, Api, params, fields};
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = TushareClient::new("your_token_here");
//!     
//!     // Now you can use string literals directly!
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

// Module definitions
mod error;
mod api;
mod types;
mod client;
mod logging;

// Public exports
pub use error::{TushareError, TushareResult};
pub use api::Api;
pub use types::{TushareRequest, TushareRequestString, TushareResponse, TushareData};
pub use client::{TushareClient, TushareClientBuilder, HttpClientConfig};
pub use logging::{LogLevel, LogConfig, Logger};

// Macros are already available at crate root via #[macro_export]
// No need to re-export

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_client_creation() {
        let client = TushareClient::new("test_token");
        // Note: Since the token field is private, we can only test if client creation succeeds
        // This just verifies it doesn't panic
    }

    #[test]
    fn test_request_creation() {
        let _client = TushareClient::new("test_token");
        
        // Using the new simplified approach
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
        // Test basic functionality of request! macro
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
        // Test request! macro handling empty params and fields
        let request = request!(Api::FundBasic, {}, []);
        
        assert_eq!(request.api_name, Api::FundBasic);
        assert_eq!(request.params.len(), 0);
        assert_eq!(request.fields.len(), 0);
    }

    #[test]
    fn test_request_macro_single_items() {
        // Test request! macro handling single param and field
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

    #[test]
    fn test_generic_conversion_trait() {
        // Test that we can define a custom type that converts from TushareResponse
        #[derive(Debug, PartialEq)]
        struct CustomData {
            count: usize,
        }

        impl TryFrom<TushareResponse> for CustomData {
            type Error = TushareError;

            fn try_from(response: TushareResponse) -> Result<Self, Self::Error> {
                Ok(CustomData {
                    count: response.data.items.len(),
                })
            }
        }

        // Create a mock TushareResponse
        let response = TushareResponse {
            request_id: "test".to_string(),
            code: 0,
            msg: None,
            data: TushareData {
                fields: vec!["field1".to_string()],
                items: vec![
                    vec![serde_json::Value::String("value1".to_string())],
                    vec![serde_json::Value::String("value2".to_string())],
                ],
            },
        };

        // Test the conversion
        let custom_data = CustomData::try_from(response).unwrap();
        assert_eq!(custom_data.count, 2);
    }
}
