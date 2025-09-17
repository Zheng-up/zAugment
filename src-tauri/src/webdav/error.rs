use thiserror::Error;
use std::time::Duration;

/// 增强的WebDAV错误类型
#[derive(Error, Debug, Clone)]
pub enum WebDAVError {
    /// 网络相关错误 - 通常可重试
    #[error("网络错误: {0}")]
    NetworkError(String),
    
    /// 超时错误 - 可重试
    #[error("请求超时: {0}")]
    TimeoutError(String),
    
    /// 认证失败 - 不可重试
    #[error("认证失败：用户名或密码错误")]
    AuthenticationError,
    
    /// 权限不足 - 不可重试
    #[error("权限不足：没有访问权限")]
    PermissionDenied,
    
    /// 文件不存在 - 不可重试（对于某些操作）
    #[error("文件不存在")]
    NotFound,
    
    /// 服务器错误 - 部分可重试
    #[error("服务器错误 {code}: {message}")]
    ServerError { code: u16, message: String },
    
    /// 解析错误 - 不可重试
    #[error("解析错误: {0}")]
    ParseError(String),
    
    /// 配置错误 - 不可重试
    #[error("配置错误: {0}")]
    InvalidConfig(String),
    
    /// 文件系统错误 - 部分可重试
    #[error("文件系统错误: {0}")]
    FileSystemError(String),
    
    /// 空间不足 - 不可重试
    #[error("磁盘空间不足")]
    InsufficientSpace,
    
    /// 文件被锁定 - 可重试
    #[error("文件被锁定或正在使用中")]
    FileLocked,
    
    /// 网络连接丢失 - 可重试
    #[error("网络连接丢失")]
    ConnectionLost,
    
    /// 速率限制 - 可重试（但需要更长等待）
    #[error("请求过于频繁，请稍后重试")]
    RateLimited,
    
    /// 服务不可用 - 可重试
    #[error("服务暂时不可用")]
    ServiceUnavailable,
    
    /// 重试次数超限
    #[error("重试次数已达上限，操作失败")]
    MaxRetriesExceeded,
    
    /// 操作被取消
    #[error("操作被用户取消")]
    OperationCancelled,
}

impl WebDAVError {
    /// 判断错误是否可重试
    pub fn is_retryable(&self) -> bool {
        match self {
            // 可重试的错误类型
            WebDAVError::NetworkError(_) |
            WebDAVError::TimeoutError(_) |
            WebDAVError::ConnectionLost |
            WebDAVError::ServiceUnavailable |
            WebDAVError::FileLocked |
            WebDAVError::RateLimited => true,
            
            // 服务器错误中的某些状态码可重试
            WebDAVError::ServerError { code, .. } => {
                matches!(code, 
                    500 | // Internal Server Error
                    502 | // Bad Gateway
                    503 | // Service Unavailable
                    504   // Gateway Timeout
                )
            },
            
            // 文件系统错误中的某些情况可重试
            WebDAVError::FileSystemError(msg) => {
                msg.contains("temporarily unavailable") ||
                msg.contains("resource busy") ||
                msg.contains("try again")
            },
            
            // 不可重试的错误类型
            _ => false,
        }
    }
    
    /// 获取重试延迟时间（毫秒）
    pub fn get_retry_delay(&self) -> Duration {
        match self {
            WebDAVError::RateLimited => Duration::from_secs(60), // 速率限制等待更久
            WebDAVError::ServiceUnavailable => Duration::from_secs(30), // 服务不可用等待30秒
            WebDAVError::FileLocked => Duration::from_secs(5), // 文件锁定等待5秒
            _ => Duration::from_secs(1), // 默认1秒
        }
    }
    
    /// 从reqwest错误转换
    pub fn from_reqwest_error(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            WebDAVError::TimeoutError(error.to_string())
        } else if error.is_connect() {
            WebDAVError::ConnectionLost
        } else if error.is_request() {
            WebDAVError::NetworkError(error.to_string())
        } else {
            WebDAVError::NetworkError(error.to_string())
        }
    }
    
    /// 从HTTP状态码转换
    pub fn from_status_code(code: u16, message: Option<String>) -> Self {
        let msg = message.unwrap_or_else(|| format!("HTTP {}", code));
        
        match code {
            401 => WebDAVError::AuthenticationError,
            403 => WebDAVError::PermissionDenied,
            404 => WebDAVError::NotFound,
            409 => WebDAVError::FileLocked,
            429 => WebDAVError::RateLimited,
            500..=599 => WebDAVError::ServerError { code, message: msg },
            _ => WebDAVError::ServerError { code, message: msg },
        }
    }
    
    /// 从IO错误转换
    pub fn from_io_error(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => WebDAVError::NotFound,
            std::io::ErrorKind::PermissionDenied => WebDAVError::PermissionDenied,
            std::io::ErrorKind::TimedOut => WebDAVError::TimeoutError(error.to_string()),
            std::io::ErrorKind::ConnectionRefused |
            std::io::ErrorKind::ConnectionAborted |
            std::io::ErrorKind::ConnectionReset => WebDAVError::ConnectionLost,
            _ => {
                let msg = error.to_string();
                if msg.contains("No space left") || msg.contains("not enough space") {
                    WebDAVError::InsufficientSpace
                } else if msg.contains("resource busy") || msg.contains("locked") {
                    WebDAVError::FileLocked
                } else {
                    WebDAVError::FileSystemError(msg)
                }
            }
        }
    }
    
    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            WebDAVError::NetworkError(_) => "网络连接出现问题，请检查网络设置".to_string(),
            WebDAVError::TimeoutError(_) => "请求超时，请稍后重试".to_string(),
            WebDAVError::AuthenticationError => "用户名或密码错误，请检查WebDAV配置".to_string(),
            WebDAVError::PermissionDenied => "没有访问权限，请检查账户权限设置".to_string(),
            WebDAVError::NotFound => "文件或目录不存在".to_string(),
            WebDAVError::ServerError { code, .. } => {
                match code {
                    500..=503 => "服务器暂时不可用，请稍后重试".to_string(),
                    _ => format!("服务器错误 ({})", code),
                }
            },
            WebDAVError::ConnectionLost => "网络连接丢失，正在尝试重新连接...".to_string(),
            WebDAVError::RateLimited => "请求过于频繁，请稍后重试".to_string(),
            WebDAVError::ServiceUnavailable => "服务暂时不可用，请稍后重试".to_string(),
            WebDAVError::InsufficientSpace => "磁盘空间不足，请清理存储空间".to_string(),
            WebDAVError::FileLocked => "文件正在被其他程序使用，请稍后重试".to_string(),
            WebDAVError::MaxRetriesExceeded => "重试次数过多，操作失败".to_string(),
            WebDAVError::OperationCancelled => "操作已取消".to_string(),
            _ => self.to_string(),
        }
    }
}

/// 重试配置
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// 最大重试次数
    pub max_retries: usize,
    /// 基础延迟时间（毫秒）
    pub base_delay: Duration,
    /// 最大延迟时间（毫秒）
    pub max_delay: Duration,
    /// 退避倍数
    pub backoff_multiplier: f64,
    /// 是否启用抖动
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

impl RetryConfig {
    /// 创建快速重试配置（用于轻量级操作）
    pub fn fast() -> Self {
        Self {
            max_retries: 2,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 1.5,
            jitter: true,
        }
    }
    
    /// 创建耐心重试配置（用于重要操作）
    pub fn patient() -> Self {
        Self {
            max_retries: 5,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
    
    /// 创建网络重试配置（用于网络敏感操作）
    pub fn network() -> Self {
        Self {
            max_retries: 4,
            base_delay: Duration::from_millis(800),
            max_delay: Duration::from_secs(45),
            backoff_multiplier: 2.2,
            jitter: true,
        }
    }
}

/// 重试策略
pub trait RetryStrategy {
    fn should_retry(&self, error: &WebDAVError, attempt: usize) -> bool;
    fn get_delay(&self, error: &WebDAVError, attempt: usize) -> Duration;
    fn max_retries(&self) -> usize;
}

/// 指数退避重试策略
#[derive(Debug, Clone)]
pub struct ExponentialBackoffStrategy {
    config: RetryConfig,
}

impl ExponentialBackoffStrategy {
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }
}

impl RetryStrategy for ExponentialBackoffStrategy {
    fn should_retry(&self, error: &WebDAVError, attempt: usize) -> bool {
        if attempt >= self.config.max_retries {
            return false;
        }
        
        error.is_retryable()
    }
    
    fn get_delay(&self, error: &WebDAVError, attempt: usize) -> Duration {
        let error_delay = error.get_retry_delay();
        let exponential_delay = Duration::from_millis(
            (self.config.base_delay.as_millis() as f64 * 
             self.config.backoff_multiplier.powi(attempt as i32)) as u64
        );
        
        let delay = std::cmp::max(error_delay, exponential_delay);
        let capped_delay = std::cmp::min(delay, self.config.max_delay);
        
        if self.config.jitter {
            // 添加随机抖动（±25%）
            let jitter = rand::random::<f64>() * 0.5 - 0.25; // -25% to +25%
            let jittered_ms = (capped_delay.as_millis() as f64 * (1.0 + jitter)) as u64;
            Duration::from_millis(jittered_ms.max(1))
        } else {
            capped_delay
        }
    }
    
    fn max_retries(&self) -> usize {
        self.config.max_retries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_retryability() {
        assert!(WebDAVError::NetworkError("test".to_string()).is_retryable());
        assert!(WebDAVError::TimeoutError("test".to_string()).is_retryable());
        assert!(!WebDAVError::AuthenticationError.is_retryable());
        assert!(!WebDAVError::PermissionDenied.is_retryable());
        
        // 测试服务器错误
        assert!(WebDAVError::ServerError { code: 500, message: "Internal Error".to_string() }.is_retryable());
        assert!(WebDAVError::ServerError { code: 503, message: "Service Unavailable".to_string() }.is_retryable());
        assert!(!WebDAVError::ServerError { code: 400, message: "Bad Request".to_string() }.is_retryable());
    }
    
    #[test]
    fn test_retry_strategy() {
        let strategy = ExponentialBackoffStrategy::new(RetryConfig::default());
        
        // 可重试错误
        let retryable_error = WebDAVError::NetworkError("test".to_string());
        assert!(strategy.should_retry(&retryable_error, 0));
        assert!(strategy.should_retry(&retryable_error, 1));
        assert!(strategy.should_retry(&retryable_error, 2));
        assert!(!strategy.should_retry(&retryable_error, 3)); // 超过最大重试次数
        
        // 不可重试错误
        let non_retryable_error = WebDAVError::AuthenticationError;
        assert!(!strategy.should_retry(&non_retryable_error, 0));
    }
    
    #[test]
    fn test_delay_calculation() {
        let strategy = ExponentialBackoffStrategy::new(RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            jitter: false, // 禁用抖动以便测试
        });
        
        let error = WebDAVError::NetworkError("test".to_string());
        
        // 测试指数退避
        let delay1 = strategy.get_delay(&error, 0);
        let delay2 = strategy.get_delay(&error, 1);
        
        assert!(delay2 >= delay1); // 延迟应该递增
    }
}
