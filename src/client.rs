use reqwest::Client;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::error::{TushareError, TushareResult};
use crate::types::{TushareRequest, TushareResponse, TushareEntityList};
use crate::api::{Api, serialize_api_name};
use crate::logging::{LogConfig, LogLevel, Logger};
use serde::{Serialize};
use serde_json;
use std::time::{SystemTime, UNIX_EPOCH};

/// HTTP client configuration for reqwest::Client
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    /// Connection timeout duration
    pub connect_timeout: Duration,
    /// Request timeout duration
    pub timeout: Duration,
    /// Maximum idle connections per host
    pub pool_max_idle_per_host: usize,
    /// Pool idle timeout duration
    pub pool_idle_timeout: Duration,
    /// User agent string
    pub user_agent: Option<String>,
    /// Enable TCP_NODELAY to reduce latency
    pub tcp_nodelay: bool,
    /// TCP keep-alive duration
    pub tcp_keepalive: Option<Duration>,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            timeout: Duration::from_secs(30),
            pool_max_idle_per_host: 20,  // Increased for better performance
            pool_idle_timeout: Duration::from_secs(90),  // Longer idle timeout
            user_agent: Some("tushare-api-rust/1.0.0".to_string()),
            tcp_nodelay: true,  // Reduce latency
            tcp_keepalive: Some(Duration::from_secs(60)),  // Keep connections alive
        }
    }
}

impl HttpClientConfig {
    /// Create a new HTTP client configuration with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set connection timeout
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }
    
    /// Set request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    /// Set maximum idle connections per host
    pub fn with_pool_max_idle_per_host(mut self, max_idle: usize) -> Self {
        self.pool_max_idle_per_host = max_idle;
        self
    }
    
    /// Set pool idle timeout
    pub fn with_pool_idle_timeout(mut self, timeout: Duration) -> Self {
        self.pool_idle_timeout = timeout;
        self
    }
    
    /// Set user agent string
    pub fn with_user_agent<S: Into<String>>(mut self, user_agent: S) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
    
    /// Enable or disable TCP_NODELAY
    pub fn with_tcp_nodelay(mut self, enabled: bool) -> Self {
        self.tcp_nodelay = enabled;
        self
    }
    
    /// Set TCP keep-alive duration
    pub fn with_tcp_keepalive(mut self, duration: Option<Duration>) -> Self {
        self.tcp_keepalive = duration;
        self
    }
    
    /// Build reqwest::Client with this configuration
    pub(crate) fn build_client(&self) -> Result<Client, reqwest::Error> {
        let mut builder = Client::builder()
            .connect_timeout(self.connect_timeout)
            .timeout(self.timeout)
            .pool_max_idle_per_host(self.pool_max_idle_per_host)
            .pool_idle_timeout(self.pool_idle_timeout)
            .tcp_nodelay(self.tcp_nodelay);
            
        if let Some(ref user_agent) = self.user_agent {
            builder = builder.user_agent(user_agent);
        }
        
        if let Some(keepalive) = self.tcp_keepalive {
            builder = builder.tcp_keepalive(keepalive);
        }
        
        builder.build()
    }
}

/// Internal request structure with token included
#[derive(Debug, Serialize)]
struct InternalTushareRequest {
    #[serde(serialize_with = "serialize_api_name")]
    api_name: Api,
    token: String,
    params: HashMap<String, String>,
    fields: Vec<String>,
}

/// Tushare API client
#[derive(Debug)]
pub struct TushareClient {
    token: String,
    client: Client,
    logger: Logger,
}

/// Tushare client builder
#[derive(Debug)]
pub struct TushareClientBuilder {
    token: Option<String>,
    http_config: HttpClientConfig,
    log_config: LogConfig,
}

impl TushareClientBuilder {
    pub fn new() -> Self {
        Self {
            token: None,
            http_config: HttpClientConfig::default(),
            log_config: LogConfig::default(),
        }
    }

    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn with_connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.http_config = self.http_config.with_connect_timeout(connect_timeout);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.http_config = self.http_config.with_timeout(timeout);
        self
    }
    
    /// Set HTTP client configuration
    pub fn with_http_config(mut self, http_config: HttpClientConfig) -> Self {
        self.http_config = http_config;
        self
    }
    
    /// Set maximum idle connections per host
    pub fn with_pool_max_idle_per_host(mut self, max_idle: usize) -> Self {
        self.http_config = self.http_config.with_pool_max_idle_per_host(max_idle);
        self
    }
    
    /// Set pool idle timeout
    pub fn with_pool_idle_timeout(mut self, timeout: Duration) -> Self {
        self.http_config = self.http_config.with_pool_idle_timeout(timeout);
        self
    }

    pub fn with_log_config(mut self, log_config: LogConfig) -> Self {
        self.log_config = log_config;
        self
    }

    /// Set log level
    pub fn with_log_level(mut self, level: LogLevel) -> Self {
        self.log_config.level = level;
        self
    }

    /// Enable or disable request logging
    pub fn log_requests(mut self, enabled: bool) -> Self {
        self.log_config.log_requests = enabled;
        self
    }

    /// Enable or disable response logging
    pub fn log_responses(mut self, enabled: bool) -> Self {
        self.log_config.log_responses = enabled;
        self
    }

    /// Enable or disable sensitive data logging
    pub fn log_sensitive_data(mut self, enabled: bool) -> Self {
        self.log_config.log_sensitive_data = enabled;
        self
    }

    /// Enable or disable performance metrics logging
    pub fn log_performance(mut self, enabled: bool) -> Self {
        self.log_config.log_performance = enabled;
        self
    }

    pub fn build(self) -> TushareResult<TushareClient> {
        let token = self.token.ok_or(TushareError::InvalidToken)?;
        
        let client = self.http_config.build_client()
            .map_err(TushareError::HttpError)?;

        Ok(TushareClient {
            token,
            client,
            logger: Logger::new(self.log_config),
        })
    }
}

impl TushareClient {
    /// Create client builder
    pub fn builder() -> TushareClientBuilder {
        TushareClientBuilder::new()
    }



    /// Create a new Tushare client with default timeout settings
    /// 
    /// # Arguments
    /// 
    /// * `token` - Tushare API Token
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use tushare_api::TushareClient;
    /// 
    /// let client = TushareClient::new("your_token_here");
    /// ```
    pub fn new(token: &str) -> Self {
        Self::with_timeout(token, Duration::from_secs(10), Duration::from_secs(30))
    }

    /// Create a new Tushare client from TUSHARE_TOKEN environment variable with default timeout settings
    /// 
    /// # Errors
    /// 
    /// Returns `TushareError::InvalidToken` if TUSHARE_TOKEN environment variable does not exist or is empty
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use tushare_api::{TushareClient, TushareResult};
    /// 
    /// // Requires TUSHARE_TOKEN environment variable to be set
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

    /// Create a new Tushare client from TUSHARE_TOKEN environment variable with custom timeout settings
    /// 
    /// # Arguments
    /// 
    /// * `connect_timeout` - Connection timeout duration
    /// * `timeout` - Request timeout duration
    /// 
    /// # Errors
    /// 
    /// Returns `TushareError::InvalidToken` if TUSHARE_TOKEN environment variable does not exist or is empty
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use tushare_api::{TushareClient, TushareResult};
    /// use std::time::Duration;
    /// 
    /// // Requires TUSHARE_TOKEN environment variable to be set
    /// let client = TushareClient::from_env_with_timeout(
    ///     Duration::from_secs(5),  // Connection timeout 5 seconds
    ///     Duration::from_secs(60)  // Request timeout 60 seconds
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

    /// Create a new Tushare client with custom timeout settings
    /// 
    /// # Arguments
    /// 
    /// * `token` - Tushare API Token
    /// * `connect_timeout` - Connection timeout duration
    /// * `timeout` - Request timeout duration
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use tushare_api::TushareClient;
    /// use std::time::Duration;
    /// 
    /// let client = TushareClient::with_timeout(
    ///     "your_token_here",
    ///     Duration::from_secs(5),  // Connection timeout 5 seconds
    ///     Duration::from_secs(60)  // Request timeout 60 seconds
    /// );
    /// ```
    pub fn with_timeout(token: &str, connect_timeout: Duration, timeout: Duration) -> Self {
        let http_config = HttpClientConfig::new()
            .with_connect_timeout(connect_timeout)
            .with_timeout(timeout);
            
        let client = http_config.build_client()
            .expect("Failed to create HTTP client");

        TushareClient {
            token: token.to_string(),
            client,
            logger: Logger::new(LogConfig::default()),
        }
    }

    /// Call Tushare API with flexible string types support
    /// 
    /// # Arguments
    /// 
    /// * `request` - API request parameters, supports direct use of string literals
    /// 
    /// # Returns
    /// 
    /// Returns API response result
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use tushare_api::{TushareClient, TushareRequest, Api, params, fields, request};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = TushareClient::new("your_token_here");
    ///     
    ///     // Now you can use string literals directly!
    ///     let request = request!(Api::StockBasic, {
    ///         "list_status" => "L"
    ///     }, [
    ///         "ts_code", "name"
    ///     ]);
    ///     
    ///     let response = client.call_api(&request).await?;
    ///     println!("Response: {:?}", response);
    /// #   Ok(())
    /// # }
    /// ```
    pub async fn call_api<T>(&self, request: &T) -> TushareResult<TushareResponse>
    where
        for<'a> &'a T: TryInto<TushareRequest>,
        for<'a> <&'a T as TryInto<TushareRequest>>::Error: Into<TushareError>,
    {
        let request = request
            .try_into()
            .map_err(Into::into)?;
        let request_id = generate_request_id();
        let start_time = Instant::now();
        // Log API call start
        self.logger.log_api_start(
            &request_id,
            &request.api_name.name(),
            request.params.len(),
            request.fields.len()
        );
        
        // Log detailed request information (if enabled)
        let token_preview_string = if self.logger.config().log_sensitive_data {
            Some(format!("token: {}***", &self.token[..self.token.len().min(8)]))
        } else {
            None
        };
        
        self.logger.log_request_details(
            &request_id,
            &request.api_name.name(),
            &format!("{:?}", request.params),
            &format!("{:?}", request.fields),
            token_preview_string.as_deref()
        );
        
        let internal_request = InternalTushareRequest {
            api_name: request.api_name,
            token: self.token.clone(),
            params: request.params,
            fields: request.fields,
        };

        self.logger.log_http_request(&request_id);
        
        let response = self.client
            .post("http://api.tushare.pro")
            .json(&internal_request)
            .send()
            .await
            .map_err(|e| {
                let elapsed = start_time.elapsed();
                self.logger.log_http_error(&request_id, elapsed, &e.to_string());
                e
            })?;

        let status = response.status();
        self.logger.log_http_response(&request_id, status.as_u16());
        
        let response_text = response.text().await
            .map_err(|e| {
                let elapsed = start_time.elapsed();
                self.logger.log_response_read_error(&request_id, elapsed, &e.to_string());
                e
            })?;
        self.logger.log_raw_response(&request_id, &response_text);
        
        let tushare_response: TushareResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                let elapsed = start_time.elapsed();
                self.logger.log_json_parse_error(&request_id, elapsed, &e.to_string(), &response_text);
                e
            })?;

        let elapsed = start_time.elapsed();
        
        if tushare_response.code != 0 {
            let message = format!("error code: {}, error msg: {}", tushare_response.code, tushare_response.msg.clone().unwrap_or_default());
            self.logger.log_api_error(&request_id, elapsed, tushare_response.code, &message);
            return Err(TushareError::ApiError {
                code: tushare_response.code,
                message
            });
        }

        // Log success information and performance metrics
        self.logger.log_api_success(&request_id, elapsed, tushare_response.data.clone().map(|data| data.items.len()).unwrap_or(0));
        
        // Log response details (if enabled)
        self.logger.log_response_details(
            &request_id,
            &tushare_response.request_id,
            &format!("{:?}", tushare_response.data.as_ref().map(|d| &d.fields))
        );

        Ok(tushare_response)
    }

    /// 调用 Tushare API，并将响应的 `data.items` 解析为强类型的 [`TushareEntityList<T>`]。
    ///
    /// 这是 [`Self::call_api`] 的便捷封装：先执行请求，再把响应转换为实体列表。
    ///
    /// # Type Parameters
    ///
    /// - `T`: 单行数据对应的实体类型（需要实现 [`crate::traits::FromTushareData`]）。
    /// - `R`: 请求类型（需要实现 `TryInto<TushareRequest>`），通常可由参数自动推导。
    ///
    /// # Errors
    ///
    /// - 请求构造失败、网络/HTTP 错误、JSON/数据映射失败等都会以 [`TushareError`] 返回。
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tushare_api::{TushareClient, TushareRequest, TushareEntityList, Api, request, DeriveFromTushareData, params, fields};
    /// # #[derive(Debug, Clone, DeriveFromTushareData)]
    /// # struct Stock { ts_code: String }
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = TushareClient::from_env()?;
    /// let stocks: TushareEntityList<Stock> = client
    ///     .call_api_as(request!(Api::StockBasic, {}, ["ts_code"]))
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub async fn call_api_as<T, R>(&self, request: R) -> TushareResult<TushareEntityList<T>>
    where
        T: crate::traits::FromTushareData,
        for<'a> &'a R: TryInto<TushareRequest>,
        for<'a> <&'a R as TryInto<TushareRequest>>::Error: Into<TushareError>,
    {
        let response = self.call_api(&request).await?;
        TushareEntityList::try_from(response).map_err(Into::into)
    }
 }

 /// Generate a unique request ID for logging purposes
 fn generate_request_id() -> String {
     let timestamp = SystemTime::now()
         .duration_since(UNIX_EPOCH)
         .unwrap_or_default()
         .as_nanos();
     format!("req_{}", timestamp)
 }

 mod tests {
    use crate::{fields, params, Api, TushareClient, TushareRequest};

    #[tokio::test]
    async fn test() {
        unsafe { std::env::set_var("TUSHARE_TOKEN", "xxxx"); }
        let client = TushareClient::from_env().unwrap();
        let response = client.call_api(&r#"
                   {
                        "api_name": "stock_basic",
                        "params": { "list_stauts": "L"},
                        "fields": [ "ts_code",
                                "symbol",
                                "name",
                                "area",
                                "industry",
                                "list_date",
                                "exchange",
                                "market"]
                    }
            "#
        ).await;
        println!("resposne = {:?}", response);
        // let parmas = params!(
        //     "list_status" => "L",
        //     "limit" => "100"
        // );
        // let req = TushareRequest::new(Api::StockBasic, parmas, fields!("ts_code"));
        // let response = client.call_api(req).await.unwrap();
        // println!("resposne = {:?}", response);
    }
}