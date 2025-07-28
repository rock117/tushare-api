use reqwest::Client;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::error::{TushareError, TushareResult};
use crate::api::{Api, serialize_api_name};
use crate::types::{TushareRequest, TushareResponse};
use crate::logging::{LogLevel, LogConfig, Logger};
use serde::{Serialize};
use serde_json;
use uuid::Uuid;

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
    connect_timeout: Option<Duration>,
    timeout: Option<Duration>,
    log_config: LogConfig,
}

impl TushareClientBuilder {
    pub fn new() -> Self {
        Self {
            token: None,
            connect_timeout: None,
            timeout: None,
            log_config: LogConfig::default(),
        }
    }

    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn with_connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.connect_timeout = Some(connect_timeout);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
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
        let connect_timeout = self.connect_timeout.unwrap_or(Duration::from_secs(10));
        let timeout = self.timeout.unwrap_or(Duration::from_secs(30));

        let client = Client::builder()
            .connect_timeout(connect_timeout)
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

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
        let client = Client::builder()
            .connect_timeout(connect_timeout)
            .timeout(timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
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
    ///     let response = client.call_api(request).await?;
    ///     println!("Response: {:?}", response);
    /// #   Ok(())
    /// # }
    /// ```
    pub async fn call_api(&self, request: TushareRequest) -> TushareResult<TushareResponse> {
        let request_id = Uuid::new_v4().to_string();
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
            let error_msg = tushare_response.msg.clone().unwrap_or_else(|| "Unknown API error".to_string());
            self.logger.log_api_error(&request_id, elapsed, tushare_response.code, &error_msg);
            return Err(TushareError::ApiError {
                code: tushare_response.code,
                message: error_msg,
            });
        }

        // Log success information and performance metrics
        self.logger.log_api_success(&request_id, elapsed, tushare_response.data.items.len());
        
        // Log response details (if enabled)
        self.logger.log_response_details(
            &request_id,
            &tushare_response.request_id,
            &format!("{:?}", tushare_response.data.fields)
        );

        Ok(tushare_response)
    }
}
