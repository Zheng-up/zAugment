use super::error::{WebDAVError, RetryStrategy, ExponentialBackoffStrategy, RetryConfig};
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;



/// 重试进度信息
#[derive(Debug, Clone)]
pub struct RetryProgress {
    pub attempt: usize,
    pub max_attempts: usize,
    pub last_error: Option<WebDAVError>,
    pub next_delay: Option<Duration>,
}

impl RetryProgress {
    /// 是否是最后一次尝试
    pub fn is_final_attempt(&self) -> bool {
        self.attempt >= self.max_attempts
    }
    
    /// 获取进度百分比
    pub fn progress_percentage(&self) -> f32 {
        if self.max_attempts == 0 {
            100.0
        } else {
            (self.attempt as f32 / self.max_attempts as f32) * 100.0
        }
    }
    
    /// 获取用户友好的状态描述
    pub fn status_message(&self) -> String {
        if let Some(error) = &self.last_error {
            if self.is_final_attempt() {
                format!("操作失败: {}", error.user_friendly_message())
            } else if let Some(delay) = self.next_delay {
                format!(
                    "第 {} 次尝试失败，{}后重试: {}",
                    self.attempt,
                    format_duration(delay),
                    error.user_friendly_message()
                )
            } else {
                format!(
                    "第 {} 次尝试失败: {}",
                    self.attempt,
                    error.user_friendly_message()
                )
            }
        } else if self.attempt == 0 {
            "开始操作...".to_string()
        } else {
            "操作成功完成".to_string()
        }
    }
}



/// 专门的重试策略实现，包含max_retries信息
#[derive(Debug, Clone)]
pub struct RetryStrategyWithConfig {
    strategy: ExponentialBackoffStrategy,
    max_retries: usize,
}

impl RetryStrategyWithConfig {
    pub fn new(config: RetryConfig) -> Self {
        let max_retries = config.max_retries;
        Self {
            strategy: ExponentialBackoffStrategy::new(config),
            max_retries,
        }
    }
    
    pub fn max_retries(&self) -> usize {
        self.max_retries
    }
}

impl RetryStrategy for RetryStrategyWithConfig {
    fn should_retry(&self, error: &WebDAVError, attempt: usize) -> bool {
        self.strategy.should_retry(error, attempt)
    }
    
    fn get_delay(&self, error: &WebDAVError, attempt: usize) -> Duration {
        self.strategy.get_delay(error, attempt)
    }
    
    fn max_retries(&self) -> usize {
        self.max_retries
    }
}

/// 改进的重试执行器，使用新的策略
#[derive(Debug, Clone)]
pub struct ImprovedRetryExecutor {
    strategy: RetryStrategyWithConfig,
}

impl ImprovedRetryExecutor {
    /// 创建新的重试执行器
    pub fn new(config: RetryConfig) -> Self {
        Self {
            strategy: RetryStrategyWithConfig::new(config),
        }
    }
    
    /// 创建默认的重试执行器
    pub fn default() -> Self {
        Self::new(RetryConfig::default())
    }
    
    /// 创建快速重试执行器
    pub fn fast() -> Self {
        Self::new(RetryConfig::fast())
    }
    
    /// 创建耐心重试执行器
    pub fn patient() -> Self {
        Self::new(RetryConfig::patient())
    }
    
    /// 创建网络重试执行器
    pub fn network() -> Self {
        Self::new(RetryConfig::network())
    }
    
    /// 执行带重试的异步操作
    pub async fn execute<F, T, Fut>(&self, operation: F) -> Result<T, WebDAVError>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, WebDAVError>> + Send,
        T: Send,
    {
        let mut last_error = None;
        
        for attempt in 0..=self.strategy.max_retries() {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    last_error = Some(error.clone());
                    
                    // 检查是否应该重试
                    if !self.strategy.should_retry(&error, attempt) {
                        break;
                    }
                    
                    // 如果不是最后一次尝试，等待后重试
                    if attempt < self.strategy.max_retries() {
                        let delay = self.strategy.get_delay(&error, attempt);
                        
                        // 记录重试信息
                        log::warn!(
                            "操作失败，{}后重试 (尝试 {}/{}): {}",
                            format_duration(delay),
                            attempt + 1,
                            self.strategy.max_retries(),
                            error.user_friendly_message()
                        );
                        
                        sleep(delay).await;
                    }
                }
            }
        }
        
        // 所有重试都失败了
        Err(last_error.unwrap_or(WebDAVError::MaxRetriesExceeded))
    }
    
    /// 执行带重试和进度回调的异步操作
    pub async fn execute_with_progress<F, T, Fut, P>(
        &self,
        operation: F,
        mut progress_callback: P,
    ) -> Result<T, WebDAVError>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, WebDAVError>> + Send,
        T: Send,
        P: FnMut(RetryProgress) + Send,
    {
        let max_retries = self.strategy.max_retries();
        let mut last_error = None;
        
        progress_callback(RetryProgress {
            attempt: 0,
            max_attempts: max_retries + 1,
            last_error: None,
            next_delay: None,
        });
        
        for attempt in 0..=max_retries {
            match operation().await {
                Ok(result) => {
                    progress_callback(RetryProgress {
                        attempt: attempt + 1,
                        max_attempts: max_retries + 1,
                        last_error: None,
                        next_delay: None,
                    });
                    return Ok(result);
                }
                Err(error) => {
                    last_error = Some(error.clone());
                    
                    // 检查是否应该重试
                    if !self.strategy.should_retry(&error, attempt) {
                        break;
                    }
                    
                    // 如果不是最后一次尝试，等待后重试
                    if attempt < max_retries {
                        let delay = self.strategy.get_delay(&error, attempt);
                        
                        progress_callback(RetryProgress {
                            attempt: attempt + 1,
                            max_attempts: max_retries + 1,
                            last_error: Some(error.clone()),
                            next_delay: Some(delay),
                        });
                        
                        sleep(delay).await;
                    } else {
                        // 最后一次尝试失败
                        progress_callback(RetryProgress {
                            attempt: attempt + 1,
                            max_attempts: max_retries + 1,
                            last_error: Some(error.clone()),
                            next_delay: None,
                        });
                    }
                }
            }
        }
        
        // 所有重试都失败了
        Err(last_error.unwrap_or(WebDAVError::MaxRetriesExceeded))
    }
}

/// 格式化持续时间为用户友好的字符串
fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let millis = duration.subsec_millis();
    
    if seconds > 0 {
        if millis > 0 {
            format!("{}.{}秒", seconds, millis / 100)
        } else {
            format!("{}秒", seconds)
        }
    } else {
        format!("{}毫秒", millis)
    }
}

/// 便利的重试宏
#[macro_export]
macro_rules! retry_async {
    ($operation:expr) => {
        $crate::webdav::retry::ImprovedRetryExecutor::default()
            .execute(|| async { $operation })
            .await
    };
    
    ($operation:expr, $config:expr) => {
        $crate::webdav::retry::ImprovedRetryExecutor::new($config)
            .execute(|| async { $operation })
            .await
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    
    #[tokio::test]
    async fn test_retry_success_on_second_attempt() {
        let attempt_counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = attempt_counter.clone();
        
        let executor = ImprovedRetryExecutor::fast();
        
        let result = executor.execute(|| {
            let counter = counter_clone.clone();
            async move {
                let attempt = counter.fetch_add(1, Ordering::SeqCst);
                if attempt == 0 {
                    Err(WebDAVError::NetworkError("first attempt fails".to_string()))
                } else {
                    Ok("success")
                }
            }
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempt_counter.load(Ordering::SeqCst), 2); // 第一次失败，第二次成功
    }
    
    #[tokio::test]
    async fn test_retry_with_non_retryable_error() {
        let attempt_counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = attempt_counter.clone();
        
        let executor = ImprovedRetryExecutor::fast();
        
        let result = executor.execute(|| {
            let counter = counter_clone.clone();
            async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Err(WebDAVError::AuthenticationError) // 不可重试的错误
            }
        }).await;
        
        assert!(result.is_err());
        assert_eq!(attempt_counter.load(Ordering::SeqCst), 1); // 只尝试一次
    }
    
    #[tokio::test]
    async fn test_retry_progress_callback() {
        let attempt_counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = attempt_counter.clone();
        let progress_updates = Arc::new(std::sync::Mutex::new(Vec::new()));
        let progress_clone = progress_updates.clone();
        
        let executor = ImprovedRetryExecutor::new(RetryConfig {
            max_retries: 2,
            base_delay: Duration::from_millis(1), // 很短的延迟以加快测试
            ..RetryConfig::default()
        });
        
        let _result = executor.execute_with_progress(
            || {
                let counter = counter_clone.clone();
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Err(WebDAVError::NetworkError("always fails".to_string()))
                }
            },
            |progress| {
                progress_clone.lock().unwrap().push(progress.attempt);
            }
        ).await;
        
        let updates = progress_updates.lock().unwrap();
        assert!(updates.len() >= 3); // 至少有开始、重试、结束的更新
    }
}
