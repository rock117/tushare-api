//! # Tushare API Client Library
//!
//! A comprehensive Rust client library for accessing Tushare financial data APIs.
//! This library provides a simple and efficient way to fetch financial data from Tushare,
//! with built-in support for request/response handling, error management, and logging.
//!
//! ## Features
//!
//! - **Easy-to-use API**: Simple client interface for making Tushare API calls
//! - **Type Safety**: Strong typing for requests and responses
//! - **Error Handling**: Comprehensive error types and handling
//! - **Logging Support**: Built-in logging with configurable levels
//! - **Async Support**: Full async/await support with tokio
//! - **Flexible Configuration**: Customizable HTTP client settings
//! - **Environment Integration**: Automatic token loading from environment variables
//! - **Automatic Conversion**: Derive macros for automatic struct conversion from API responses
//!
//! ## Quick Start
//!
//! ```no_run
//! use tushare_api::{TushareClient, Api, TushareRequest, TushareEntityList, params, fields};
//! use tushare_api::DeriveFromTushareData;
//!
//! // Define your data structure with derive macro
//! #[derive(Debug, Clone, DeriveFromTushareData)]
//! pub struct Stock {
//!     ts_code: String,
//!     symbol: String,
//!     name: String,
//!     area: Option<String>,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client from environment variable TUSHARE_TOKEN
//!     let client = TushareClient::from_env()?;
//!     
//!     // Create request using manual construction
//!     let request = TushareRequest::new(
//!         Api::StockBasic,
//!         params!("list_status" => "L"),
//!         fields!["ts_code", "symbol", "name", "area"]
//!     );
//!     
//!     // Make the API call with automatic conversion
//!     let stocks: TushareEntityList<Stock> = client.call_api_as(request).await?;
//!     
//!     println!("Received {} stocks", stocks.len());
//!     
//!     for stock in stocks.iter().take(5) {
//!         println!("{}: {}", stock.ts_code, stock.name);
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod api;
pub mod types;
pub mod client;
pub mod logging;
pub mod traits;
pub mod utils;
pub mod basic_types;
pub mod third_party_types;
pub mod custom_date_format;

// Re-export main types for convenience
pub use error::{TushareError, TushareResult};
pub use api::Api;
pub use types::{TushareRequest, TushareResponse, TushareData, TushareEntityList};
pub use client::{TushareClient, HttpClientConfig};
pub use logging::{LogConfig, LogLevel, Logger};
pub use traits::{FromTushareData, FromTushareValue, FromOptionalTushareValue};
pub use utils::response_to_vec;

// Macros are automatically exported at the crate root via #[macro_export]

// Re-export procedural macros from tushare-derive
pub use tushare_derive::{FromTushareData as DeriveFromTushareData};

// Re-export serde_json for user convenience
pub use serde_json;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TushareData;
    use serde_json::json;

    #[test]
    fn test_api_serialization() {
        let api = Api::StockBasic;
        assert_eq!(api.name(), "stock_basic");
    }

    #[test]
    fn test_tushare_request_creation() {
        let request = TushareRequest::new(
            Api::StockBasic,
            vec![("list_status".to_string(), "L".to_string())],
            vec!["ts_code".to_string(), "symbol".to_string()]
        );
        
        assert_eq!(request.api_name, Api::StockBasic);
        assert_eq!(request.params.len(), 1);
        assert_eq!(request.fields.len(), 2);
    }

    #[test]
    fn test_tushare_response_creation() {
        let response = TushareResponse {
            request_id: "test123".to_string(),
            code: 0,
            msg: None,
            data: Some(TushareData {
                fields: vec!["ts_code".to_string(), "name".to_string()],
                items: vec![
                    vec![json!("000001.SZ"), json!("平安银行")],
                    vec![json!("000002.SZ"), json!("万科A")],
                ],
                has_more: false,
                count: 2,
            }),
        };
        
        assert_eq!(response.code, 0);
        assert_eq!(response.data.map(|data| data.items.len()).unwrap_or(0), 2);
        assert_eq!(response.data.map(|data| data.items.len()).unwrap_or(0), 2);
    }
}
