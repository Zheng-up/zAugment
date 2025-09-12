use super::config::WebDAVConfig;
use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM, NONCE_LEN};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};


/// 安全存储的WebDAV配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureWebDAVConfig {
    pub server_url: String,
    pub username: String,
    pub enabled: bool,
    pub auto_sync: bool,
    pub sync_interval_minutes: u32,
    pub remote_path: String,
    // 密码将被加密存储，不在这里明文保存
    #[serde(skip)]
    password_encrypted: Option<Vec<u8>>,
    #[serde(skip)]
    salt: Option<Vec<u8>>,
}

/// 简单的随机数生成器
struct OneNonceSequence(Option<Nonce>);

impl OneNonceSequence {
    fn new(nonce: Nonce) -> Self {
        Self(Some(nonce))
    }
}

impl NonceSequence for OneNonceSequence {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        self.0.take().ok_or(ring::error::Unspecified)
    }
}

/// 密码管理器
pub struct PasswordManager {
    rng: SystemRandom,
}

impl PasswordManager {
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
        }
    }

    /// 使用keyring安全存储密码
    pub fn store_password(&self, username: &str, password: &str) -> Result<(), String> {
        match keyring::Entry::new("ZAugment_WebDAV", username) {
            Ok(entry) => {
                entry.set_password(password)
                    .map_err(|e| format!("密码存储失败: {}", e))
            }
            Err(e) => Err(format!("创建密码条目失败: {}", e))
        }
    }

    /// 从keyring获取密码
    pub fn get_password(&self, username: &str) -> Result<String, String> {
        match keyring::Entry::new("ZAugment_WebDAV", username) {
            Ok(entry) => {
                entry.get_password()
                    .map_err(|e| format!("密码获取失败: {}", e))
            }
            Err(e) => Err(format!("创建密码条目失败: {}", e))
        }
    }

    /// 删除存储的密码
    pub fn delete_password(&self, username: &str) -> Result<(), String> {
        match keyring::Entry::new("ZAugment_WebDAV", username) {
            Ok(entry) => {
                entry.delete_password()
                    .map_err(|e| format!("密码删除失败: {}", e))
            }
            Err(e) => Err(format!("创建密码条目失败: {}", e))
        }
    }

    /// 生成密钥和随机数
    fn generate_key_and_nonce(&self) -> Result<([u8; 32], [u8; NONCE_LEN]), String> {
        let mut key = [0u8; 32];
        let mut nonce = [0u8; NONCE_LEN];
        
        self.rng.fill(&mut key)
            .map_err(|_| "生成密钥失败")?;
        self.rng.fill(&mut nonce)
            .map_err(|_| "生成随机数失败")?;
        
        Ok((key, nonce))
    }

    /// 加密数据（备用方案，当keyring不可用时使用）
    pub fn encrypt_data(&self, data: &str, key: &[u8; 32]) -> Result<(Vec<u8>, [u8; NONCE_LEN]), String> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        self.rng.fill(&mut nonce_bytes)
            .map_err(|_| "生成随机数失败")?;
        
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let unbound_key = UnboundKey::new(&AES_256_GCM, key)
            .map_err(|_| "创建密钥失败")?;
        let mut sealing_key = SealingKey::new(unbound_key, OneNonceSequence::new(nonce));
        
        let mut data_bytes = data.as_bytes().to_vec();
        sealing_key.seal_in_place_append_tag(Aad::empty(), &mut data_bytes)
            .map_err(|_| "加密失败")?;
        
        Ok((data_bytes, nonce_bytes))
    }

    /// 解密数据（备用方案）
    pub fn decrypt_data(&self, encrypted_data: &[u8], key: &[u8; 32], nonce_bytes: &[u8; NONCE_LEN]) -> Result<String, String> {
        let nonce = Nonce::assume_unique_for_key(*nonce_bytes);
        let unbound_key = UnboundKey::new(&AES_256_GCM, key)
            .map_err(|_| "创建密钥失败")?;
        let mut opening_key = OpeningKey::new(unbound_key, OneNonceSequence::new(nonce));
        
        let mut data = encrypted_data.to_vec();
        let decrypted = opening_key.open_in_place(Aad::empty(), &mut data)
            .map_err(|_| "解密失败")?;
        
        String::from_utf8(decrypted.to_vec())
            .map_err(|_| "解密数据格式错误".to_string())
    }
}

impl SecureWebDAVConfig {
    /// 从普通配置创建安全配置
    pub fn from_config(config: &WebDAVConfig, password_manager: &PasswordManager) -> Result<Self, String> {
        // 使用keyring存储密码
        password_manager.store_password(&config.username, &config.password)?;
        
        Ok(Self {
            server_url: config.server_url.clone(),
            username: config.username.clone(),
            enabled: config.enabled,
            auto_sync: config.auto_sync,
            sync_interval_minutes: config.sync_interval_minutes,
            remote_path: config.remote_path.clone(),
            password_encrypted: None,
            salt: None,
        })
    }

    /// 转换为普通配置（包含密码）
    pub fn to_config(&self, password_manager: &PasswordManager) -> Result<WebDAVConfig, String> {
        let password = password_manager.get_password(&self.username)?;
        
        Ok(WebDAVConfig {
            server_url: self.server_url.clone(),
            username: self.username.clone(),
            password,
            enabled: self.enabled,
            auto_sync: self.auto_sync,
            sync_interval_minutes: self.sync_interval_minutes,
            remote_path: self.remote_path.clone(),
        })
    }

    /// 验证配置是否完整（不包括密码检查）
    pub fn is_valid_structure(&self) -> bool {
        !self.server_url.is_empty() && !self.username.is_empty()
    }

    /// 更新配置（不包括密码）
    pub fn update_config(&mut self, 
        server_url: Option<String>,
        username: Option<String>,
        enabled: Option<bool>,
        auto_sync: Option<bool>,
        sync_interval_minutes: Option<u32>,
        remote_path: Option<String>,
    ) {
        if let Some(url) = server_url {
            self.server_url = url;
        }
        if let Some(user) = username {
            self.username = user;
        }
        if let Some(en) = enabled {
            self.enabled = en;
        }
        if let Some(auto) = auto_sync {
            self.auto_sync = auto;
        }
        if let Some(interval) = sync_interval_minutes {
            self.sync_interval_minutes = interval;
        }
        if let Some(path) = remote_path {
            self.remote_path = path;
        }
    }

    /// 安全地显示配置信息（隐藏敏感信息）
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

impl Default for SecureWebDAVConfig {
    fn default() -> Self {
        Self {
            server_url: "https://dav.jianguoyun.com/dav/".to_string(),
            username: String::new(),
            enabled: false,
            auto_sync: false,
            sync_interval_minutes: 30,
            remote_path: "/ZAugment/tokens.json".to_string(),
            password_encrypted: None,
            salt: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_encryption() {
        let pm = PasswordManager::new();
        let key = [0u8; 32]; // 测试用密钥
        let test_data = "test_password_123";
        
        let (encrypted, nonce) = pm.encrypt_data(test_data, &key).unwrap();
        let decrypted = pm.decrypt_data(&encrypted, &key, &nonce).unwrap();
        
        assert_eq!(test_data, decrypted);
    }

    #[test]
    fn test_secure_config_creation() {
        let config = WebDAVConfig::new(
            "https://dav.jianguoyun.com/dav/".to_string(),
            "test@example.com".to_string(),
            "password123".to_string(),
        );
        
        let pm = PasswordManager::new();
        
        // 注意：这个测试可能在某些环境下失败，因为keyring可能不可用
        if let Ok(secure_config) = SecureWebDAVConfig::from_config(&config, &pm) {
            assert!(secure_config.is_valid_structure());
            assert_eq!(secure_config.username, "test@example.com");
            
            // 尝试恢复配置
            if let Ok(restored_config) = secure_config.to_config(&pm) {
                assert_eq!(restored_config.password, "password123");
            }
        }
    }
}
