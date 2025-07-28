use std::fmt;
use std::error::Error as StdError;

/// Tushare API error types
#[derive(Debug)]
pub enum TushareError {
    /// HTTP request error
    HttpError(reqwest::Error),
    /// API response error (contains error code and error message)
    ApiError {
        code: i32,
        message: String,
    },
    /// JSON serialization/deserialization error
    SerializationError(serde_json::Error),
    /// Network timeout error
    TimeoutError,
    /// Invalid API Token
    InvalidToken,
    /// Other errors
    Other(String),
}

impl fmt::Display for TushareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TushareError::HttpError(err) => write!(f, "HTTP request error: {err}"),
            TushareError::ApiError { code, message } => {
                write!(f, "API error (code: {code}): {message}")
            }
            TushareError::SerializationError(err) => write!(f, "Serialization error: {err}"),
            TushareError::TimeoutError => write!(f, "Request timeout"),
            TushareError::InvalidToken => write!(f, "Invalid API Token"),
            TushareError::Other(msg) => write!(f, "Other error: {msg}"),
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

/// Tushare API result type
pub type TushareResult<T> = Result<T, TushareError>;
