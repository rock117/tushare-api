use std::fmt;
use std::error::Error as StdError;

/// Tushare API 错误类型
#[derive(Debug)]
pub enum TushareError {
    /// HTTP 请求错误
    HttpError(reqwest::Error),
    /// API 响应错误（包含错误码和错误信息）
    ApiError {
        code: i32,
        message: String,
    },
    /// JSON 序列化/反序列化错误
    SerializationError(serde_json::Error),
    /// 网络超时错误
    TimeoutError,
    /// 无效的 API Token
    InvalidToken,
    /// 其他错误
    Other(String),
}

impl fmt::Display for TushareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TushareError::HttpError(err) => write!(f, "HTTP 请求错误: {err}"),
            TushareError::ApiError { code, message } => {
                write!(f, "API 错误 (代码: {code}): {message}")
            }
            TushareError::SerializationError(err) => write!(f, "序列化错误: {err}"),
            TushareError::TimeoutError => write!(f, "请求超时"),
            TushareError::InvalidToken => write!(f, "无效的 API Token"),
            TushareError::Other(msg) => write!(f, "其他错误: {msg}"),
        }
    }
}

impl StdError for TushareError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            TushareError::HttpError(err) => Some(err),
            TushareError::SerializationError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for TushareError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            TushareError::TimeoutError
        } else {
            TushareError::HttpError(err)
        }
    }
}

impl From<serde_json::Error> for TushareError {
    fn from(err: serde_json::Error) -> Self {
        TushareError::SerializationError(err)
    }
}

/// Tushare API 结果类型
pub type TushareResult<T> = Result<T, TushareError>;
