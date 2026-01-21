use crate::error::{TushareError, TushareResult};
use crate::types::{TushareEntityList, TushareRequest, TushareResponse};
use crate::{Api, TushareClient};
use rand::Rng;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// Retry configuration for [`TushareClientEx`].
///
/// The retry logic is implemented at the wrapper layer so that [`TushareClient`]
/// can stay focused on a single HTTP request + response parsing.
///
/// Notes:
/// - Only retryable errors will be retried (currently network/timeout errors).
/// - The delay uses exponential backoff: `base_delay * 2^attempt`, capped by `max_delay`.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: usize,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(5),
        }
    }
}

/// Extended client wrapper that adds advanced behaviors on top of [`TushareClient`].
///
/// Currently supported:
/// - **Per-API minimum interval rate limiting (default: sleep)**
///   If an API is configured with a minimum interval (e.g. 10 seconds), repeated
///   calls to the same API will be automatically delayed so that two calls are at
///   least `min_interval` apart. Callers do not need to implement any sleep logic.
///
/// - **Retry with exponential backoff (optional)**
///   When enabled via [`Self::with_retry_config`], network/timeout failures will be
///   retried with exponential backoff.
///
/// This wrapper is designed to keep the core client stable while allowing you to
/// opt into additional behaviors.
#[derive(Debug)]
pub struct TushareClientEx {
    inner: TushareClient,
    api_min_intervals: HashMap<String, Duration>,
    api_next_allowed_at: Mutex<HashMap<String, Instant>>,
    retry: Option<RetryConfig>,
}

impl TushareClientEx {
    /// Create a new wrapper client.
    ///
    /// By default, no per-API interval limit is applied and retry is disabled.
    pub fn new(inner: TushareClient) -> Self {
        Self {
            inner,
            api_min_intervals: HashMap::new(),
            api_next_allowed_at: Mutex::new(HashMap::new()),
            retry: None,
        }
    }

    /// Configure a minimum interval between two calls of the same API.
    ///
    /// If the interval is not met, the wrapper will `sleep` until it becomes
    /// eligible to call.
    ///
    /// Example:
    ///
    /// ```rust,no_run
    /// use std::time::Duration;
    /// use tushare_api::{Api, TushareClient, TushareClientEx};
    ///
    /// # fn build(inner: TushareClient) -> TushareClientEx {
    /// TushareClientEx::new(inner)
    ///     .with_api_min_interval(Api::Daily, Duration::from_secs(10))
    /// # }
    /// ```
    pub fn with_api_min_interval(mut self, api: Api, min_interval: Duration) -> Self {
        self.api_min_intervals.insert(api.name(), min_interval);
        self
    }

    /// Enable retry with exponential backoff.
    ///
    /// Retryable errors:
    /// - [`TushareError::HttpError`]
    /// - [`TushareError::TimeoutError`]
    ///
    /// Non-retryable errors (by design):
    /// - [`TushareError::ApiError`] (business-level errors returned by Tushare)
    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry = Some(config);
        self
    }

    /// Borrow the underlying [`TushareClient`].
    pub fn inner(&self) -> &TushareClient {
        &self.inner
    }

    /// Consume the wrapper and return the underlying [`TushareClient`].
    pub fn into_inner(self) -> TushareClient {
        self.inner
    }

    /// Call API with configured rate limiting (sleep) and optional retry.
    pub async fn call_api<T>(&self, request: &T) -> TushareResult<TushareResponse>
    where
        for<'a> &'a T: TryInto<TushareRequest>,
        for<'a> <&'a T as TryInto<TushareRequest>>::Error: Into<TushareError>,
    {
        let request = request.try_into().map_err(Into::into)?;

        self.apply_api_min_interval_rate_limit(&request.api_name.name()).await;

        self.call_api_with_retry(request).await
    }

    pub async fn call_api_as<T, R>(&self, request: &R) -> TushareResult<TushareEntityList<T>>
    where
        T: crate::traits::FromTushareData,
        for<'a> &'a R: TryInto<TushareRequest>,
        for<'a> <&'a R as TryInto<TushareRequest>>::Error: Into<TushareError>,
    {
        let response = self.call_api(request).await?;
        TushareEntityList::try_from(response).map_err(Into::into)
    }

    async fn call_api_with_retry(&self, request: TushareRequest) -> TushareResult<TushareResponse> {
        let Some(cfg) = self.retry.clone() else {
            return self.inner.call_api_request(&request).await;
        };

        let mut attempt = 0usize;
        let api_name = request.api_name.name();

        loop {
            match self.inner.call_api_request(&request).await {
                Ok(resp) => return Ok(resp),
                Err(err) => {
                    let should_retry = attempt < cfg.max_retries && is_retryable_error(&err);
                    if !should_retry {
                        self.inner.logger().log_safe(
                            crate::logging::LogLevel::Error,
                            || {
                                format!(
                                    "tushare_api retry exhausted or non-retryable error; api={}, attempts={}, max_retries={}, err={}",
                                    api_name, attempt, cfg.max_retries, err
                                )
                            },
                            None,
                        );
                        return Err(err);
                    }

                    let delay = compute_backoff_delay(&cfg, attempt);
                    self.inner.logger().log_safe(
                        crate::logging::LogLevel::Warn,
                        || {
                            format!(
                                "tushare_api retrying; api={}, retry={}/{}, delay={:?}, err={}",
                                api_name,
                                attempt + 1,
                                cfg.max_retries,
                                delay,
                                err
                            )
                        },
                        None,
                    );
                    sleep(delay).await;
                    attempt += 1;
                }
            }
        }
    }

    async fn apply_api_min_interval_rate_limit(&self, api_name: &str) {
        let Some(min_interval) = self.api_min_intervals.get(api_name).copied() else {
            return;
        };

        let now = Instant::now();
        let wait = {
            let mut guard = self.api_next_allowed_at.lock().await;
            let next_allowed_at = guard.get(api_name).copied().unwrap_or(now);
            let base = if next_allowed_at > now { next_allowed_at } else { now };
            guard.insert(api_name.to_string(), base + min_interval);
            if base > now {
                base - now
            } else {
                Duration::from_secs(0)
            }
        };

        if !wait.is_zero() {
            sleep(wait).await;
        }
    }
}

fn is_retryable_error(err: &TushareError) -> bool {
    matches!(
        err,
        TushareError::HttpError(_) | TushareError::TimeoutError
    )
}

fn compute_backoff_delay(cfg: &RetryConfig, attempt: usize) -> Duration {
    let shift = attempt.min(31) as u32;
    let factor = 1u64.checked_shl(shift).unwrap_or(u64::MAX);
    let base = cfg.base_delay.saturating_mul(factor as u32);
    let capped = if base > cfg.max_delay { cfg.max_delay } else { base };

    // Equal jitter: capped/2 + random(0..=capped/2)
    // Compared to full jitter, this is less volatile while still spreading retries.
    let capped_ms = capped.as_millis().min(u64::MAX as u128) as u64;
    if capped_ms == 0 {
        return Duration::from_millis(0);
    }

    let half = capped_ms / 2;
    let jitter_ms = rand::thread_rng().gen_range(0..=half);
    Duration::from_millis(half + jitter_ms)
}

