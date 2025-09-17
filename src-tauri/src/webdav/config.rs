use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebDAVConfig {
    pub server_url: String,
    pub username: String,
    pub password: String,  // 注意：实际应用中应该加密存储
    pub enabled: bool,
    pub auto_sync: bool,
    pub sync_interval_minutes: u32,
    pub remote_path: String,
}

impl Default for WebDAVConfig {
    fn default() -> Self {
        Self {
            server_url: "https://dav.jianguoyun.com/dav/".to_string(),
            username: String::new(),
            password: String::new(),
            enabled: false,
            auto_sync: false,
            sync_interval_minutes: 30, // 默认30分钟同步一次
            remote_path: "/ZAugment/tokens.json".to_string(),
        }
    }
}

impl WebDAVConfig {
    pub fn new(server_url: String, username: String, password: String) -> Self {
        Self {
            server_url,
            username,
            password,
            enabled: true,
            auto_sync: false,
            sync_interval_minutes: 30,
            remote_path: "/ZAugment/tokens.json".to_string(),
        }
    }

    /// 验证配置是否完整
    pub fn is_valid(&self) -> bool {
        !self.server_url.is_empty() && !self.username.is_empty() && !self.password.is_empty()
    }

    /// 获取完整的远程文件URL
    pub fn get_remote_file_url(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut url = self.server_url.clone();
        if !url.ends_with('/') {
            url.push('/');
        }
        
        let remote_path = if self.remote_path.starts_with('/') {
            &self.remote_path[1..]
        } else {
            &self.remote_path
        };
        
        Ok(format!("{}{}", url, remote_path))
    }

    /// 安全地显示配置信息（隐藏密码）
    pub fn display_safe(&self) -> String {
        format!(
            "WebDAV配置: {}@{} (启用: {}, 自动同步: {})",
            self.username,
            self.server_url,
            self.enabled,
            self.auto_sync
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = WebDAVConfig::new(
            "https://dav.jianguoyun.com/dav/".to_string(),
            "test@example.com".to_string(),
            "password123".to_string(),
        );
        
        assert!(config.is_valid());
        assert_eq!(config.enabled, true);
    }

    #[test]
    fn test_remote_url_generation() {
        let config = WebDAVConfig::new(
            "https://dav.jianguoyun.com/dav/".to_string(),
            "test@example.com".to_string(),
            "password123".to_string(),
        );
        
        let url = config.get_remote_file_url().unwrap();
        assert_eq!(url, "https://dav.jianguoyun.com/dav/ZAugment/tokens.json");
    }
}
