use log::{info, debug, error, trace, warn};

#[cfg(feature = "tracing")]
use tracing::{info as tracing_info, debug as tracing_debug, error as tracing_error, trace as tracing_trace, warn as tracing_warn};

/// Log level configuration
#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    /// Disable logging
    Off,
    /// Log errors only
    Error,
    /// Log errors and warnings
    Warn,
    /// Log basic information (default)
    Info,
    /// Log detailed debug information
    Debug,
    /// Log all information including raw data
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

/// Log configuration
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Log level
    pub level: LogLevel,
    /// Whether to log request parameters
    pub log_requests: bool,
    /// Whether to log response content
    pub log_responses: bool,
    /// Whether to log sensitive data (such as token)
    pub log_sensitive_data: bool,
    /// Whether to log performance metrics
    pub log_performance: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            log_requests: true,
            log_responses: false, // Responses can be large, not logged by default
            log_sensitive_data: false, // Sensitive data not logged by default
            log_performance: true,
        }
    }
}

/// Logger
#[derive(Debug)]
pub struct Logger {
    config: LogConfig,
}

impl Logger {
    /// Create a new logger
    pub fn new(config: LogConfig) -> Self {
        Self { config }
    }

    /// Check if logging should be performed for the specified level
    pub fn should_log(&self, level: &LogLevel) -> bool {
        use LogLevel::*;
        match (&self.config.level, level) {
            (Off, _) => false,
            (Error, Error) => true,
            (Warn, Error | Warn) => true,
            (Info, Error | Warn | Info) => true,
            (Debug, Error | Warn | Info | Debug) => true,
            (Trace, _) => true,
            _ => false,
        }
    }

    /// Safely log messages with lazy evaluation (decides whether to record sensitive information based on configuration)
    pub fn log_safe<F>(&self, level: LogLevel, message_fn: F, sensitive_data: Option<&str>)
    where
        F: FnOnce() -> String,
    {
        if !self.should_log(&level) {
            return;
        }

        let message = message_fn();
        let full_message = if self.config.log_sensitive_data {
            if let Some(sensitive) = sensitive_data {
                format!("{} [Sensitive data: {}]", message, sensitive)
            } else {
                message
            }
        } else {
            message
        };

        // Choose logging backend based on compile features
        #[cfg(feature = "tracing")]
        match level {
            LogLevel::Error => tracing_error!("{}", full_message),
            LogLevel::Warn => tracing_warn!("{}", full_message),
            LogLevel::Info => tracing_info!("{}", full_message),
            LogLevel::Debug => tracing_debug!("{}", full_message),
            LogLevel::Trace => tracing_trace!("{}", full_message),
            LogLevel::Off => {}
        }
        
        #[cfg(not(feature = "tracing"))]
        match level {
            LogLevel::Error => error!("{}", full_message),
            LogLevel::Warn => warn!("{}", full_message),
            LogLevel::Info => info!("{}", full_message),
            LogLevel::Debug => debug!("{}", full_message),
            LogLevel::Trace => trace!("{}", full_message),
            LogLevel::Off => {}
        }
    }

    /// Log API call start
    pub fn log_api_start(&self, request_id: &str, api_name: &str, params_count: usize, fields_count: usize) {
        let request_id = request_id.to_string();
        let api_name = api_name.to_string();
        self.log_safe(
            LogLevel::Info,
            move || format!(
                "[{}] Starting Tushare API call: {}, params count: {}, fields count: {}",
                request_id, api_name, params_count, fields_count
            ),
            None,
        );
    }

    /// Log request details
    pub fn log_request_details(&self, request_id: &str, api_name: &str, params: &str, fields: &str, token_preview: Option<&str>) {
        if !self.config.log_requests {
            return;
        }

        let request_id = request_id.to_string();
        let api_name = api_name.to_string();
        let params = params.to_string();
        let fields = fields.to_string();
        self.log_safe(
            LogLevel::Debug,
            move || format!(
                "[{}] API request details - API: {}, params: {}, fields: {}",
                request_id, api_name, params, fields
            ),
            token_preview,
        );
    }

    /// Log HTTP request sending
    pub fn log_http_request(&self, request_id: &str) {
        let request_id = request_id.to_string();
        self.log_safe(
            LogLevel::Debug,
            move || format!("[{}] Sending HTTP request to Tushare API", request_id),
            None,
        );
    }

    /// Log HTTP request failure
    pub fn log_http_error(&self, request_id: &str, elapsed: std::time::Duration, error: &str) {
        let request_id = request_id.to_string();
        let error = error.to_string();
        self.log_safe(
            LogLevel::Error,
            move || format!(
                "[{}] HTTP request failed, duration: {:?}, error: {}",
                request_id, elapsed, error
            ),
            None,
        );
    }

    /// Log HTTP response reception
    pub fn log_http_response(&self, request_id: &str, status_code: u16) {
        let request_id = request_id.to_string();
        self.log_safe(
            LogLevel::Debug,
            move || format!("[{}] Received HTTP response, status code: {}", request_id, status_code),
            None,
        );
    }

    /// Log response reading failure
    pub fn log_response_read_error(&self, request_id: &str, elapsed: std::time::Duration, error: &str) {
        let request_id = request_id.to_string();
        let error = error.to_string();
        self.log_safe(
            LogLevel::Error,
            move || format!(
                "[{}] Failed to read response content, duration: {:?}, error: {}",
                request_id, elapsed, error
            ),
            None,
        );
    }

    /// Log raw response content
    pub fn log_raw_response(&self, request_id: &str, response_text: &str) {
        if !self.config.log_responses {
            return;
        }

        let request_id = request_id.to_string();
        let response_text = response_text.to_string();
        self.log_safe(
            LogLevel::Trace,
            move || format!("[{}] Raw response content: {}", request_id, response_text),
            None,
        );
    }

    /// Log JSON parsing failure
    pub fn log_json_parse_error(&self, request_id: &str, elapsed: std::time::Duration, error: &str, response_text: &str) {
        let request_id = request_id.to_string();
        let error = error.to_string();
        let response_preview = if self.config.log_responses {
            response_text.to_string()
        } else {
            "[Hidden]".to_string()
        };

        self.log_safe(
            LogLevel::Error,
            move || format!(
                "[{}] JSON parsing failed, duration: {:?}, error: {}, response content: {}",
                request_id, elapsed, error, response_preview
            ),
            None,
        );
    }

    /// Log API error
    pub fn log_api_error(&self, request_id: &str, elapsed: std::time::Duration, code: i32, message: &str) {
        let request_id = request_id.to_string();
        let message = message.to_string();
        self.log_safe(
            LogLevel::Error,
            move || format!(
                "[{}] API returned error, duration: {:?}, error code: {}, error message: {}",
                request_id, elapsed, code, message
            ),
            None,
        );
    }

    /// Log API call success
    pub fn log_api_success(&self, request_id: &str, elapsed: std::time::Duration, data_count: usize) {
        let request_id = request_id.to_string();
        if self.config.log_performance {
            self.log_safe(
                LogLevel::Info,
                move || format!(
                    "[{}] API call successful, duration: {:?}, data rows returned: {}",
                    request_id, elapsed, data_count
                ),
                None,
            );
        } else {
            self.log_safe(
                LogLevel::Info,
                move || format!("[{}] API call successful", request_id),
                None,
            );
        }
    }

    /// Log response details
    pub fn log_response_details(&self, request_id: &str, response_request_id: &str, fields: &str) {
        if !self.config.log_responses {
            return;
        }

        let request_id = request_id.to_string();
        let response_request_id = response_request_id.to_string();
        let fields = fields.to_string();
        self.log_safe(
            LogLevel::Debug,
            move || format!(
                "[{}] Response details - Request ID: {}, fields: {}",
                request_id, response_request_id, fields
            ),
            None,
        );
    }

    /// Get reference to log configuration
    pub fn config(&self) -> &LogConfig {
        &self.config
    }
}
