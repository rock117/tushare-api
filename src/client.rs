use reqwest::Client;
use std::time::Duration;
use crate::error::{TushareError, TushareResult};
use crate::types::{TushareRequest, TushareResponse};
use crate::api::serialize_api_name;
use serde::Serialize;
use std::collections::HashMap;
use crate::api::Api;

/// 内部使用的完整请求结构体（包含 token）
#[derive(Debug, Serialize)]
struct InternalTushareRequest {
    #[serde(serialize_with = "serialize_api_name")]
    api_name: Api,
    token: String,
    params: HashMap<String, String>,
    fields: Vec<String>,
}

/// Tushare API 客户端
pub struct TushareClient {
    token: String,
    client: Client,
}

impl TushareClient {
    /// 创建新的 Tushare 客户端（使用默认超时设置）
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
        Self::with_timeout(token, Duration::from_secs(10), Duration::from_secs(30))
    }

    /// 从环境变量 TUSHARE_TOKEN 创建新的 Tushare 客户端（使用默认超时设置）
    /// 
    /// # 错误
    /// 
    /// 如果环境变量 TUSHARE_TOKEN 不存在或为空，返回 `TushareError::InvalidToken`
    /// 
    /// # 示例
    /// 
    /// ```rust,no_run
    /// use tushare_api::{TushareClient, TushareResult};
    /// 
    /// // 需要设置环境变量 TUSHARE_TOKEN
    /// let client = TushareClient::from_env()?;
    /// # Ok::<(), tushare_api::TushareError>(())
    /// ```
    pub fn from_env() -> TushareResult<Self> {
        let token = std::env::var("TUSHARE_TOKEN")
            .map_err(|_| TushareError::InvalidToken)?
            .trim()
            .to_string();
        
        if token.is_empty() {
            return Err(TushareError::InvalidToken);
        }
        
        Ok(Self::new(&token))
    }

    /// 从环境变量 TUSHARE_TOKEN 创建新的 Tushare 客户端（自定义超时设置）
    /// 
    /// # 参数
    /// 
    /// * `connect_timeout` - 连接超时时间
    /// * `timeout` - 请求超时时间
    /// 
    /// # 错误
    /// 
    /// 如果环境变量 TUSHARE_TOKEN 不存在或为空，返回 `TushareError::InvalidToken`
    /// 
    /// # 示例
    /// 
    /// ```rust,no_run
    /// use tushare_api::{TushareClient, TushareResult};
    /// use std::time::Duration;
    /// 
    /// // 需要设置环境变量 TUSHARE_TOKEN
    /// let client = TushareClient::from_env_with_timeout(
    ///     Duration::from_secs(5),  // 连接超时 5 秒
    ///     Duration::from_secs(60)  // 请求超时 60 秒
    /// )?;
    /// # Ok::<(), tushare_api::TushareError>(())
    /// ```
    pub fn from_env_with_timeout(connect_timeout: Duration, timeout: Duration) -> TushareResult<Self> {
        let token = std::env::var("TUSHARE_TOKEN")
            .map_err(|_| TushareError::InvalidToken)?
            .trim()
            .to_string();
        
        if token.is_empty() {
            return Err(TushareError::InvalidToken);
        }
        
        Ok(Self::with_timeout(&token, connect_timeout, timeout))
    }

    /// 创建新的 Tushare 客户端（自定义超时设置）
    /// 
    /// # 参数
    /// 
    /// * `token` - Tushare API Token
    /// * `connect_timeout` - 连接超时时间
    /// * `timeout` - 请求超时时间
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use tushare_api::TushareClient;
    /// use std::time::Duration;
    /// 
    /// let client = TushareClient::with_timeout(
    ///     "your_token_here",
    ///     Duration::from_secs(5),  // 连接超时 5 秒
    ///     Duration::from_secs(60)  // 请求超时 60 秒
    /// );
    /// ```
    pub fn with_timeout(token: &str, connect_timeout: Duration, timeout: Duration) -> Self {
        let client = Client::builder()
            .connect_timeout(connect_timeout)
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            token: token.to_string(),
            client,
        }
    }

    /// 通用的 Tushare API 调用方法
    /// 
    /// # 参数
    /// 
    /// * `request` - Tushare API 请求结构体
    /// 
    /// # 返回值
    /// 
    /// 返回 `TushareResult<TushareResponse>`
    /// 
    /// # 示例
    /// 
    /// ```rust,no_run
    /// use tushare_api::{TushareClient, TushareRequest, Api, TushareResult};
    /// use std::collections::HashMap;
    /// 
    /// #[tokio::main]
    /// async fn main() -> TushareResult<()> {
    ///     let client = TushareClient::new("your_token_here");
    ///     
    ///     let mut params = HashMap::new();
    ///     params.insert("list_status".to_string(), "L".to_string());
    ///     
    ///     let request = TushareRequest {
    ///         api_name: Api::StockBasic,
    ///         params,
    ///         fields: vec!["ts_code".to_string(), "name".to_string()],
    ///     };
    ///     
    ///     let response = client.call_api(request).await?;
    ///     println!("API 调用成功，返回 {} 条记录", response.data.items.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn call_api(&self, request: TushareRequest) -> TushareResult<TushareResponse> {
        // 构建包含 token 的内部请求结构体
        let internal_request = InternalTushareRequest {
            api_name: request.api_name,
            token: self.token.clone(),
            params: request.params,
            fields: request.fields,
        };

        let response = self.client
            .post("http://api.tushare.pro")
            .json(&internal_request)
            .send()
            .await?;

        let tushare_response: TushareResponse = response.json().await?;
        
        if tushare_response.code != 0 {
            let message = tushare_response.msg.unwrap_or("未知错误".to_string());
            return Err(TushareError::ApiError {
                code: tushare_response.code,
                message,
            });
        }

        Ok(tushare_response)
    }
}
