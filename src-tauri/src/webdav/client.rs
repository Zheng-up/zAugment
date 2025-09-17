use reqwest::{Client, StatusCode};
use std::path::Path;
use chrono::{DateTime, Utc};
use super::config::WebDAVConfig;
use super::error::WebDAVError;
use super::retry::{ImprovedRetryExecutor, RetryProgress};
use super::error::RetryConfig;

#[derive(Debug, Clone)]
pub struct WebDAVClient {
    client: Client,
    config: WebDAVConfig,
    retry_executor: ImprovedRetryExecutor,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub last_modified: DateTime<Utc>,
    pub is_directory: bool,
    pub etag: Option<String>,
}



impl WebDAVClient {
    /// 创建新的WebDAV客户端
    pub fn new(config: WebDAVConfig) -> Result<Self, WebDAVError> {
        Self::new_with_retry_config(config, RetryConfig::network())
    }
    
    /// 创建新的WebDAV客户端，使用自定义重试配置
    pub fn new_with_retry_config(config: WebDAVConfig, retry_config: RetryConfig) -> Result<Self, WebDAVError> {
        if !config.is_valid() {
            return Err(WebDAVError::InvalidConfig("配置信息不完整".to_string()));
        }

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            .user_agent("ZAugment-WebDAV-Client/1.0")
            .build()
            .map_err(|e| WebDAVError::from_reqwest_error(e))?;

        let retry_executor = ImprovedRetryExecutor::new(retry_config);

        Ok(Self { 
            client, 
            config,
            retry_executor,
        })
    }
    
    /// 创建快速重试的WebDAV客户端
    pub fn new_fast_retry(config: WebDAVConfig) -> Result<Self, WebDAVError> {
        Self::new_with_retry_config(config, RetryConfig::fast())
    }
    
    /// 创建耐心重试的WebDAV客户端
    pub fn new_patient_retry(config: WebDAVConfig) -> Result<Self, WebDAVError> {
        Self::new_with_retry_config(config, RetryConfig::patient())
    }

    /// 测试连接
    pub async fn test_connection(&self) -> Result<bool, WebDAVError> {
        self.retry_executor.execute(|| {
            let client = &self.client;
            let config = &self.config;
            async move {
                let response = client
                    .request(reqwest::Method::from_bytes(b"PROPFIND").unwrap(), &config.server_url)
                    .basic_auth(&config.username, Some(&config.password))
                    .header("Depth", "0")
                    .send()
                    .await
                    .map_err(WebDAVError::from_reqwest_error)?;

                match response.status() {
                    StatusCode::OK | StatusCode::MULTI_STATUS => Ok(true),
                    code => Err(WebDAVError::from_status_code(code.as_u16(), None)),
                }
            }
        }).await
    }
    
    /// 测试连接并提供进度反馈
    pub async fn test_connection_with_progress<F>(&self, progress_callback: F) -> Result<bool, WebDAVError>
    where
        F: FnMut(RetryProgress) + Send,
    {
        self.retry_executor.execute_with_progress(
            || {
                let client = &self.client;
                let config = &self.config;
                async move {
                    let response = client
                        .request(reqwest::Method::from_bytes(b"PROPFIND").unwrap(), &config.server_url)
                        .basic_auth(&config.username, Some(&config.password))
                        .header("Depth", "0")
                        .send()
                        .await
                        .map_err(WebDAVError::from_reqwest_error)?;

                    match response.status() {
                        StatusCode::OK | StatusCode::MULTI_STATUS => Ok(true),
                        code => Err(WebDAVError::from_status_code(code.as_u16(), None)),
                    }
                }
            },
            progress_callback,
        ).await
    }

    /// 检查文件是否存在
    pub async fn file_exists(&self, remote_path: &str) -> Result<bool, WebDAVError> {
        let url = self.build_url(remote_path)?;
        
        self.retry_executor.execute(|| {
            let client = &self.client;
            let config = &self.config;
            let url = url.clone();
            async move {
                let response = client
                    .head(&url)
                    .basic_auth(&config.username, Some(&config.password))
                    .send()
                    .await
                    .map_err(WebDAVError::from_reqwest_error)?;

                match response.status() {
                    StatusCode::OK => Ok(true),
                    StatusCode::NOT_FOUND => Ok(false),
                    code => Err(WebDAVError::from_status_code(code.as_u16(), None)),
                }
            }
        }).await
    }

    /// 获取文件信息
    pub async fn get_file_info(&self, remote_path: &str) -> Result<Option<FileInfo>, WebDAVError> {
        let url = self.build_url(remote_path)?;
        
        let propfind_body = r#"<?xml version="1.0" encoding="utf-8" ?>
<D:propfind xmlns:D="DAV:">
    <D:prop>
        <D:getlastmodified/>
        <D:getcontentlength/>
        <D:getetag/>
        <D:resourcetype/>
        <D:displayname/>
    </D:prop>
</D:propfind>"#;

        let response = self.client
            .request(reqwest::Method::from_bytes(b"PROPFIND").unwrap(), &url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .header("Depth", "0")
            .header("Content-Type", "application/xml")
            .body(propfind_body)
            .send()
            .await
            .map_err(WebDAVError::from_reqwest_error)?;

        println!("PROPFIND请求URL: {}", url);
        println!("PROPFIND响应状态码: {}", response.status());
        
        match response.status() {
            StatusCode::MULTI_STATUS => {
                let text = response.text().await.map_err(WebDAVError::from_reqwest_error)?;
                println!("PROPFIND响应内容: {}", text);
                self.parse_propfind_response(&text, remote_path)
            }
            StatusCode::NOT_FOUND => {
                println!("文件未找到 (404)");
                Ok(None)
            },
            StatusCode::UNAUTHORIZED => {
                println!("认证失败 (401)");
                Err(WebDAVError::AuthenticationError)
            },
            code => {
                println!("其他错误状态码: {}", code);
                Err(WebDAVError::from_status_code(code.as_u16(), None))
            },
        }
    }

    /// 下载文件
    pub async fn download_file(&self, remote_path: &str) -> Result<Vec<u8>, WebDAVError> {
        let url = self.build_url(remote_path)?;
        
        self.retry_executor.execute(|| {
            let client = &self.client;
            let config = &self.config;
            let url = url.clone();
            async move {
                let response = client
                    .get(&url)
                    .basic_auth(&config.username, Some(&config.password))
                    .send()
                    .await
                    .map_err(WebDAVError::from_reqwest_error)?;

                match response.status() {
                    StatusCode::OK => {
                        let bytes = response.bytes().await.map_err(WebDAVError::from_reqwest_error)?;
                        Ok(bytes.to_vec())
                    }
                    code => Err(WebDAVError::from_status_code(code.as_u16(), None)),
                }
            }
        }).await
    }
    
    /// 下载文件并提供进度反馈
    pub async fn download_file_with_progress<F>(
        &self, 
        remote_path: &str,
        progress_callback: F
    ) -> Result<Vec<u8>, WebDAVError>
    where
        F: FnMut(RetryProgress) + Send,
    {
        let url = self.build_url(remote_path)?;
        
        self.retry_executor.execute_with_progress(
            || {
                let client = &self.client;
                let config = &self.config;
                let url = url.clone();
                async move {
                    let response = client
                        .get(&url)
                        .basic_auth(&config.username, Some(&config.password))
                        .send()
                        .await
                        .map_err(WebDAVError::from_reqwest_error)?;

                    match response.status() {
                        StatusCode::OK => {
                            let bytes = response.bytes().await.map_err(WebDAVError::from_reqwest_error)?;
                            Ok(bytes.to_vec())
                        }
                        code => Err(WebDAVError::from_status_code(code.as_u16(), None)),
                    }
                }
            },
            progress_callback,
        ).await
    }

    /// 上传文件
    pub async fn upload_file(&self, remote_path: &str, content: &[u8]) -> Result<(), WebDAVError> {
        // 首先确保目录存在
        self.ensure_directory_exists(remote_path).await?;
        
        let url = self.build_url(remote_path)?;
        let content_vec = content.to_vec(); // 克隆内容以在重试中使用
        
        self.retry_executor.execute(|| {
            let client = &self.client;
            let config = &self.config;
            let url = url.clone();
            let content = content_vec.clone();
            async move {
                let response = client
                    .put(&url)
                    .basic_auth(&config.username, Some(&config.password))
                    .header("Content-Type", "application/json")
                    .body(content)
                    .send()
                    .await
                    .map_err(WebDAVError::from_reqwest_error)?;

                match response.status() {
                    StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
                    code => Err(WebDAVError::from_status_code(code.as_u16(), None)),
                }
            }
        }).await
    }
    
    /// 上传文件并提供进度反馈
    pub async fn upload_file_with_progress<F>(
        &self, 
        remote_path: &str, 
        content: &[u8],
        progress_callback: F
    ) -> Result<(), WebDAVError>
    where
        F: FnMut(RetryProgress) + Send,
    {
        // 首先确保目录存在
        self.ensure_directory_exists(remote_path).await?;
        
        let url = self.build_url(remote_path)?;
        let content_vec = content.to_vec();
        
        self.retry_executor.execute_with_progress(
            || {
                let client = &self.client;
                let config = &self.config;
                let url = url.clone();
                let content = content_vec.clone();
                async move {
                    let response = client
                        .put(&url)
                        .basic_auth(&config.username, Some(&config.password))
                        .header("Content-Type", "application/json")
                        .body(content)
                        .send()
                        .await
                        .map_err(WebDAVError::from_reqwest_error)?;

                    match response.status() {
                        StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
                        code => Err(WebDAVError::from_status_code(code.as_u16(), None)),
                    }
                }
            },
            progress_callback,
        ).await
    }

    /// 删除文件
    pub async fn delete_file(&self, remote_path: &str) -> Result<(), WebDAVError> {
        let url = self.build_url(remote_path)?;
        
        let response = self.client
            .delete(&url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .map_err(WebDAVError::from_reqwest_error)?;

        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            StatusCode::NOT_FOUND => Err(WebDAVError::NotFound),
            StatusCode::UNAUTHORIZED => Err(WebDAVError::AuthenticationError),
            code => Err(WebDAVError::from_status_code(code.as_u16(), None)),
        }
    }

    /// 创建目录
    pub async fn create_directory(&self, remote_path: &str) -> Result<(), WebDAVError> {
        let url = self.build_url(remote_path)?;
        
        let response = self.client
            .request(reqwest::Method::from_bytes(b"MKCOL").unwrap(), &url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .map_err(WebDAVError::from_reqwest_error)?;

        match response.status() {
            StatusCode::CREATED | StatusCode::OK => Ok(()),
            StatusCode::METHOD_NOT_ALLOWED => Ok(()), // 目录已存在
            StatusCode::UNAUTHORIZED => Err(WebDAVError::AuthenticationError),
            code => Err(WebDAVError::from_status_code(code.as_u16(), None)),
        }
    }

    /// 构建完整URL
    fn build_url(&self, remote_path: &str) -> Result<String, WebDAVError> {
        let mut base_url = self.config.server_url.clone();
        if !base_url.ends_with('/') {
            base_url.push('/');
        }
        
        let path = if remote_path.starts_with('/') {
            &remote_path[1..]
        } else {
            remote_path
        };
        
        Ok(format!("{}{}", base_url, path))
    }

    /// 确保目录存在
    async fn ensure_directory_exists(&self, file_path: &str) -> Result<(), WebDAVError> {
        if let Some(parent) = Path::new(file_path).parent() {
            let parent_str = parent.to_string_lossy().replace('\\', "/");
            if !parent_str.is_empty() && parent_str != "." {
                self.create_directory(&parent_str).await?;
            }
        }
        Ok(())
    }

    /// 解析PROPFIND响应
    fn parse_propfind_response(&self, xml: &str, remote_path: &str) -> Result<Option<FileInfo>, WebDAVError> {
        // 简单的XML解析（实际应用中应该使用专门的XML解析库）
        // 这里只是提取基本信息
        
        // 支持大写和小写的DAV命名空间前缀
        let has_content_length = xml.contains("<D:getcontentlength>") || xml.contains("<d:getcontentlength>");
        
        if has_content_length {
            let size = self.extract_xml_value(xml, "getcontentlength")
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or(0);
            
            let last_modified = self.extract_xml_value(xml, "getlastmodified")
                .and_then(|s| chrono::DateTime::parse_from_rfc2822(&s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);
            
            let etag = self.extract_xml_value(xml, "getetag");
            
            let is_directory = xml.contains("<D:collection/>") || xml.contains("<d:collection/>");
            
            let name = Path::new(remote_path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            
            println!("解析到文件信息: 名称={}, 大小={}, 是否目录={}", name, size, is_directory);
            
            Ok(Some(FileInfo {
                name,
                path: remote_path.to_string(),
                size,
                last_modified,
                is_directory,
                etag,
            }))
        } else {
            println!("XML中未找到getcontentlength标签");
            Ok(None)
        }
    }

    /// 从XML中提取值
    fn extract_xml_value(&self, xml: &str, tag: &str) -> Option<String> {
        // 尝试大写前缀
        let start_tag_upper = format!("<D:{}>", tag);
        let end_tag_upper = format!("</D:{}>", tag);
        
        if let Some(start) = xml.find(&start_tag_upper) {
            let content_start = start + start_tag_upper.len();
            if let Some(end) = xml[content_start..].find(&end_tag_upper) {
                let content = &xml[content_start..content_start + end];
                return Some(content.trim().to_string());
            }
        }
        
        // 尝试小写前缀
        let start_tag_lower = format!("<d:{}>", tag);
        let end_tag_lower = format!("</d:{}>", tag);
        
        if let Some(start) = xml.find(&start_tag_lower) {
            let content_start = start + start_tag_lower.len();
            if let Some(end) = xml[content_start..].find(&end_tag_lower) {
                let content = &xml[content_start..content_start + end];
                return Some(content.trim().to_string());
            }
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url() {
        let config = WebDAVConfig::new(
            "https://dav.jianguoyun.com/dav/".to_string(),
            "test@example.com".to_string(),
            "password123".to_string(),
        );
        
        let client = WebDAVClient::new(config).unwrap();
        let url = client.build_url("ZAugment/tokens.json").unwrap();
        assert_eq!(url, "https://dav.jianguoyun.com/dav/ZAugment/tokens.json");
    }
}
