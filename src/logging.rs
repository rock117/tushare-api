use log::{info, debug, error, trace, warn};

#[cfg(feature = "tracing")]
use tracing::{info as tracing_info, debug as tracing_debug, error as tracing_error, trace as tracing_trace, warn as tracing_warn};

/// 日志级别配置
#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    /// 关闭日志
    Off,
    /// 只记录错误
    Error,
    /// 记录错误和警告
    Warn,
    /// 记录基本信息（默认）
    Info,
    /// 记录详细调试信息
    Debug,
    /// 记录所有信息包括原始数据
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

/// 日志配置
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// 日志级别
    pub level: LogLevel,
    /// 是否记录请求参数
    pub log_requests: bool,
    /// 是否记录响应内容
    pub log_responses: bool,
    /// 是否记录敏感数据（如 token）
    pub log_sensitive_data: bool,
    /// 是否记录性能指标
    pub log_performance: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            log_requests: true,
            log_responses: false, // 响应可能很大，默认不记录
            log_sensitive_data: false, // 默认不记录敏感数据
            log_performance: true,
        }
    }
}

/// 日志记录器
#[derive(Debug)]
pub struct Logger {
    config: LogConfig,
}

impl Logger {
    /// 创建新的日志记录器
    pub fn new(config: LogConfig) -> Self {
        Self { config }
    }

    /// 检查是否应该记录指定级别的日志
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

    /// 安全记录日志（根据配置决定是否记录敏感信息）
    pub fn log_safe(&self, level: LogLevel, message: &str, sensitive_data: Option<&str>) {
        if !self.should_log(&level) {
            return;
        }

        let full_message = if self.config.log_sensitive_data {
            if let Some(sensitive) = sensitive_data {
                format!("{} [敏感数据: {}]", message, sensitive)
            } else {
                message.to_string()
            }
        } else {
            message.to_string()
        };

        // 根据编译特性选择日志后端
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

    /// 记录 API 调用开始
    pub fn log_api_start(&self, request_id: &str, api_name: &str, params_count: usize, fields_count: usize) {
        self.log_safe(
            LogLevel::Info,
            &format!(
                "[{}] 开始调用 Tushare API: {}, 参数数量: {}, 字段数量: {}",
                request_id, api_name, params_count, fields_count
            ),
            None,
        );
    }

    /// 记录请求详情
    pub fn log_request_details(&self, request_id: &str, api_name: &str, params: &str, fields: &str, token_preview: Option<&str>) {
        if !self.config.log_requests {
            return;
        }

        self.log_safe(
            LogLevel::Debug,
            &format!(
                "[{}] API 请求详情 - API: {}, 参数: {}, 字段: {}",
                request_id, api_name, params, fields
            ),
            token_preview,
        );
    }

    /// 记录 HTTP 请求发送
    pub fn log_http_request(&self, request_id: &str) {
        self.log_safe(
            LogLevel::Debug,
            &format!("[{}] 发送 HTTP 请求到 Tushare API", request_id),
            None,
        );
    }

    /// 记录 HTTP 请求失败
    pub fn log_http_error(&self, request_id: &str, elapsed: std::time::Duration, error: &str) {
        self.log_safe(
            LogLevel::Error,
            &format!(
                "[{}] HTTP 请求失败，耗时: {:?}, 错误: {}",
                request_id, elapsed, error
            ),
            None,
        );
    }

    /// 记录 HTTP 响应接收
    pub fn log_http_response(&self, request_id: &str, status_code: u16) {
        self.log_safe(
            LogLevel::Debug,
            &format!("[{}] 收到 HTTP 响应，状态码: {}", request_id, status_code),
            None,
        );
    }

    /// 记录响应读取失败
    pub fn log_response_read_error(&self, request_id: &str, elapsed: std::time::Duration, error: &str) {
        self.log_safe(
            LogLevel::Error,
            &format!(
                "[{}] 读取响应内容失败，耗时: {:?}, 错误: {}",
                request_id, elapsed, error
            ),
            None,
        );
    }

    /// 记录原始响应内容
    pub fn log_raw_response(&self, request_id: &str, response_text: &str) {
        if !self.config.log_responses {
            return;
        }

        self.log_safe(
            LogLevel::Trace,
            &format!("[{}] 原始响应内容: {}", request_id, response_text),
            None,
        );
    }

    /// 记录 JSON 解析失败
    pub fn log_json_parse_error(&self, request_id: &str, elapsed: std::time::Duration, error: &str, response_text: &str) {
        let response_preview = if self.config.log_responses {
            response_text
        } else {
            "[已隐藏]"
        };

        self.log_safe(
            LogLevel::Error,
            &format!(
                "[{}] JSON 解析失败，耗时: {:?}, 错误: {}, 响应内容: {}",
                request_id, elapsed, error, response_preview
            ),
            None,
        );
    }

    /// 记录 API 错误
    pub fn log_api_error(&self, request_id: &str, elapsed: std::time::Duration, code: i32, message: &str) {
        self.log_safe(
            LogLevel::Error,
            &format!(
                "[{}] API 返回错误，耗时: {:?}, 错误码: {}, 错误信息: {}",
                request_id, elapsed, code, message
            ),
            None,
        );
    }

    /// 记录 API 调用成功
    pub fn log_api_success(&self, request_id: &str, elapsed: std::time::Duration, data_count: usize) {
        if self.config.log_performance {
            self.log_safe(
                LogLevel::Info,
                &format!(
                    "[{}] API 调用成功，耗时: {:?}, 返回数据行数: {}",
                    request_id, elapsed, data_count
                ),
                None,
            );
        } else {
            self.log_safe(
                LogLevel::Info,
                &format!("[{}] API 调用成功", request_id),
                None,
            );
        }
    }

    /// 记录响应详情
    pub fn log_response_details(&self, request_id: &str, response_request_id: &str, fields: &str) {
        if !self.config.log_responses {
            return;
        }

        self.log_safe(
            LogLevel::Debug,
            &format!(
                "[{}] 响应详情 - 请求ID: {}, 字段: {}",
                request_id, response_request_id, fields
            ),
            None,
        );
    }

    /// 获取日志配置的引用
    pub fn config(&self) -> &LogConfig {
        &self.config
    }
}
