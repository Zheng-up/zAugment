use super::client::WebDAVClient;
use super::config::WebDAVConfig;
use super::error::WebDAVError;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use tokio::fs;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub last_sync: Option<DateTime<Utc>>,
    pub last_local_modified: Option<DateTime<Utc>>,
    pub last_remote_modified: Option<DateTime<Utc>>,
    pub local_etag: Option<String>,
    pub remote_etag: Option<String>,
    pub local_checksum: Option<String>,
    pub remote_checksum: Option<String>,
    pub sync_count: u64,
    pub last_sync_size: u64,
}

#[derive(Debug, Clone)]
pub enum SyncAction {
    UploadToRemote,
    DownloadFromRemote,
    NoActionNeeded,
    ConflictDetected,
}

#[derive(Debug, Clone)]
pub enum ConflictResolution {
    KeepLocal,           // 保留本地文件
    KeepRemote,          // 保留远程文件
    KeepBoth,            // 保留两个文件（创建副本）
    Merge,               // 合并文件（暂不实现）
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub local_size: u64,
    pub remote_size: u64,
    pub local_modified: DateTime<Utc>,
    pub remote_modified: DateTime<Utc>,
    pub local_checksum: String,
    pub remote_checksum: String,
}

#[derive(Debug, Clone)]
pub struct SyncResult {
    pub action: SyncAction,
    pub success: bool,
    pub message: String,
    pub bytes_transferred: u64,
    pub local_checksum: Option<String>,
    pub remote_checksum: Option<String>,
    pub conflict_details: Option<String>,
}

pub struct CloudSync {
    client: WebDAVClient,
    local_file_path: PathBuf,
    remote_file_path: String,
    status: SyncStatus,
}

impl CloudSync {
    /// 创建新的云同步实例
    pub fn new(config: WebDAVConfig, local_file_path: PathBuf) -> Result<Self, WebDAVError> {
        let client = WebDAVClient::new(config.clone())?;
        let remote_file_path = config.remote_path.clone();
        
        Ok(Self {
            client,
            local_file_path,
            remote_file_path,
            status: SyncStatus {
                last_sync: None,
                last_local_modified: None,
                last_remote_modified: None,
                local_etag: None,
                remote_etag: None,
                local_checksum: None,
                remote_checksum: None,
                sync_count: 0,
                last_sync_size: 0,
            },
        })
    }

    /// 测试连接
    pub async fn test_connection(&self) -> Result<bool, WebDAVError> {
        self.client.test_connection().await
    }

    /// 执行同步
    pub async fn sync(&mut self) -> Result<SyncResult, WebDAVError> {
        // 1. 检查本地文件状态
        let local_info = self.get_local_file_info().await?;
        
        // 2. 检查远程文件状态
        let remote_info = self.client.get_file_info(&self.remote_file_path).await?;
        
        // 3. 决定同步动作
        let action = self.determine_sync_action(&local_info, &remote_info);
        
        // 4. 执行同步动作
        let result = self.execute_sync_action(action, &local_info, &remote_info).await?;
        
        // 5. 更新状态
        if result.success {
            self.update_sync_status(&local_info, &remote_info);
        }
        
        Ok(result)
    }

    /// 强制上传到远程
    pub async fn force_upload(&mut self) -> Result<SyncResult, WebDAVError> {
        let local_info = self.get_local_file_info().await?;
        if local_info.is_none() {
            return Ok(SyncResult {
                action: SyncAction::NoActionNeeded,
                success: false,
                message: "本地文件不存在".to_string(),
                bytes_transferred: 0,
                local_checksum: None,
                remote_checksum: None,
                conflict_details: None,
            });
        }

        let content = fs::read(&self.local_file_path).await.map_err(|e| {
            WebDAVError::from_io_error(e)
        })?;

        // 计算本地文件校验和
        let local_checksum = self.calculate_data_checksum(&content);

        self.client.upload_file(&self.remote_file_path, &content).await?;

        let result = SyncResult {
            action: SyncAction::UploadToRemote,
            success: true,
            message: "强制上传成功".to_string(),
            bytes_transferred: content.len() as u64,
            local_checksum: Some(local_checksum.clone()),
            remote_checksum: Some(local_checksum.clone()), // 上传后远程校验和应该与本地相同
            conflict_details: None,
        };

        // 更新状态
        let remote_info = self.client.get_file_info(&self.remote_file_path).await?;
        self.update_sync_status(&local_info, &remote_info);

        Ok(result)
    }

    /// 强制从远程下载
    pub async fn force_download(&mut self) -> Result<SyncResult, WebDAVError> {
        // 调试信息
        println!("正在检查远程文件: {}", self.remote_file_path);
        let remote_info = self.client.get_file_info(&self.remote_file_path).await?;
        
        if remote_info.is_none() {
            println!("远程文件不存在: {}", self.remote_file_path);
            return Ok(SyncResult {
                action: SyncAction::NoActionNeeded,
                success: false,
                message: format!("远程文件不存在: {}", self.remote_file_path),
                bytes_transferred: 0,
                local_checksum: None,
                remote_checksum: None,
                conflict_details: None,
            });
        }
        
        println!("远程文件存在，开始下载: {}", self.remote_file_path);

        let content = self.client.download_file(&self.remote_file_path).await?;
        
        // 确保本地目录存在
        if let Some(parent) = self.local_file_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                WebDAVError::from_io_error(e)
            })?;
        }
        
        fs::write(&self.local_file_path, &content).await.map_err(|e| {
            WebDAVError::from_io_error(e)
        })?;

        // 计算远程文件校验和
        let remote_checksum = self.calculate_data_checksum(&content);

        let result = SyncResult {
            action: SyncAction::DownloadFromRemote,
            success: true,
            message: "强制下载成功".to_string(),
            bytes_transferred: content.len() as u64,
            local_checksum: Some(remote_checksum.clone()), // 下载后本地校验和应该与远程相同
            remote_checksum: Some(remote_checksum),
            conflict_details: None,
        };

        // 更新状态
        let local_info = self.get_local_file_info().await?;
        self.update_sync_status(&local_info, &remote_info);

        Ok(result)
    }

    /// 获取本地文件信息
    async fn get_local_file_info(&self) -> Result<Option<LocalFileInfo>, WebDAVError> {
        if !self.local_file_path.exists() {
            return Ok(None);
        }

        let metadata = fs::metadata(&self.local_file_path).await.map_err(|e| {
            WebDAVError::from_io_error(e)
        })?;

        let modified = metadata.modified().map_err(|e| {
            WebDAVError::from_io_error(e)
        })?;

        Ok(Some(LocalFileInfo {
            size: metadata.len(),
            last_modified: DateTime::from(modified),
        }))
    }

    /// 决定同步动作
    fn determine_sync_action(
        &self,
        local_info: &Option<LocalFileInfo>,
        remote_info: &Option<super::client::FileInfo>,
    ) -> SyncAction {
        match (local_info, remote_info) {
            (None, None) => SyncAction::NoActionNeeded,
            (Some(_), None) => SyncAction::UploadToRemote,
            (None, Some(_)) => SyncAction::DownloadFromRemote,
            (Some(local), Some(remote)) => {
                // 比较修改时间
                if local.last_modified > remote.last_modified {
                    SyncAction::UploadToRemote
                } else if remote.last_modified > local.last_modified {
                    SyncAction::DownloadFromRemote
                } else {
                    // 时间相同，比较ETag或大小
                    if let Some(remote_etag) = &remote.etag {
                        if let Some(local_etag) = &self.status.local_etag {
                            if remote_etag != local_etag {
                                SyncAction::ConflictDetected
                            } else {
                                SyncAction::NoActionNeeded
                            }
                        } else {
                            SyncAction::NoActionNeeded
                        }
                    } else if local.size != remote.size {
                        SyncAction::ConflictDetected
                    } else {
                        SyncAction::NoActionNeeded
                    }
                }
            }
        }
    }

    /// 执行同步动作
    async fn execute_sync_action(
        &self,
        action: SyncAction,
        _local_info: &Option<LocalFileInfo>,
        _remote_info: &Option<super::client::FileInfo>,
    ) -> Result<SyncResult, WebDAVError> {
        match action {
            SyncAction::NoActionNeeded => Ok(SyncResult {
                action,
                success: true,
                message: "无需同步".to_string(),
                bytes_transferred: 0,
                local_checksum: None,
                remote_checksum: None,
                conflict_details: None,
            }),
            
            SyncAction::UploadToRemote => {
                let content = fs::read(&self.local_file_path).await.map_err(|e| {
                    WebDAVError::from_io_error(e)
                })?;

                // 计算本地文件校验和
                let local_checksum = self.calculate_data_checksum(&content);
                
                self.client.upload_file(&self.remote_file_path, &content).await?;

                Ok(SyncResult {
                    action,
                    success: true,
                    message: "上传到远程成功".to_string(),
                    bytes_transferred: content.len() as u64,
                    local_checksum: Some(local_checksum.clone()),
                    remote_checksum: Some(local_checksum), // 上传后远程校验和应该与本地相同
                    conflict_details: None,
                })
            }
            
            SyncAction::DownloadFromRemote => {
                let content = self.client.download_file(&self.remote_file_path).await?;
                
                // 确保本地目录存在
                if let Some(parent) = self.local_file_path.parent() {
                    fs::create_dir_all(parent).await.map_err(|e| {
                        WebDAVError::from_io_error(e)
                    })?;
                }
                
                fs::write(&self.local_file_path, &content).await.map_err(|e| {
                    WebDAVError::from_io_error(e)
                })?;

                // 计算远程文件校验和
                let remote_checksum = self.calculate_data_checksum(&content);

                Ok(SyncResult {
                    action,
                    success: true,
                    message: "从远程下载成功".to_string(),
                    bytes_transferred: content.len() as u64,
                    local_checksum: Some(remote_checksum.clone()), // 下载后本地校验和应该与远程相同
                    remote_checksum: Some(remote_checksum),
                    conflict_details: None,
                })
            }
            
            SyncAction::ConflictDetected => {
                // 尝试获取本地和远程文件的校验和来提供冲突详情
                let local_checksum = match fs::read(&self.local_file_path).await {
                    Ok(content) => Some(self.calculate_data_checksum(&content)),
                    Err(_) => None,
                };
                
                let remote_checksum = match self.client.download_file(&self.remote_file_path).await {
                    Ok(content) => Some(self.calculate_data_checksum(&content)),
                    Err(_) => None,
                };

                let conflict_details = format!(
                    "本地和远程文件都已修改。本地校验和: {:?}, 远程校验和: {:?}",
                    local_checksum, remote_checksum
                );

                Ok(SyncResult {
                    action,
                    success: false,
                    message: "检测到冲突：本地和远程文件都已修改，请手动处理".to_string(),
                    bytes_transferred: 0,
                    local_checksum,
                    remote_checksum,
                    conflict_details: Some(conflict_details),
                })
            },
        }
    }

    /// 更新同步状态
    fn update_sync_status(
        &mut self,
        local_info: &Option<LocalFileInfo>,
        remote_info: &Option<super::client::FileInfo>,
    ) {
        self.status.last_sync = Some(Utc::now());
        self.status.sync_count += 1;
        
        if let Some(local) = local_info {
            self.status.last_local_modified = Some(local.last_modified);
        }
        
        if let Some(remote) = remote_info {
            self.status.last_remote_modified = Some(remote.last_modified);
            self.status.remote_etag = remote.etag.clone();
        }
    }

    /// 获取同步状态
    pub fn get_status(&self) -> &SyncStatus {
        &self.status
    }

    /// 计算文件校验和（SHA256）
    async fn calculate_file_checksum(&self, file_path: &PathBuf) -> Result<String, WebDAVError> {
        let content = fs::read(file_path).await.map_err(|e| {
            WebDAVError::from_io_error(e)
        })?;
        Ok(self.calculate_data_checksum(&content))
    }

    /// 计算数据校验和（SHA256）
    fn calculate_data_checksum(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// 比较文件是否相同（通过校验和）
    async fn files_are_identical(&self, local_checksum: &str, remote_data: &[u8]) -> bool {
        let remote_checksum = self.calculate_data_checksum(remote_data);
        local_checksum == remote_checksum
    }

    /// 增强的文件比较逻辑
    async fn compare_files_detailed(
        &self,
        local_info: &Option<LocalFileInfo>,
        remote_info: &Option<super::client::FileInfo>,
    ) -> Result<SyncAction, WebDAVError> {
        match (local_info, remote_info) {
            (None, None) => Ok(SyncAction::NoActionNeeded),
            (Some(_), None) => Ok(SyncAction::UploadToRemote),
            (None, Some(_)) => Ok(SyncAction::DownloadFromRemote),
            (Some(local), Some(remote)) => {
                // 首先比较修改时间
                let time_diff = (local.last_modified.timestamp() - remote.last_modified.timestamp()).abs();
                
                // 如果时间差小于2秒，认为是同一时间（考虑到文件系统时间精度）
                if time_diff <= 2 {
                    // 时间相近，检查文件大小
                    if local.size == remote.size {
                        // 大小相同，进一步检查校验和
                        let local_checksum = self.calculate_file_checksum(&self.local_file_path).await?;
                        
                        // 下载远程文件内容计算校验和
                        match self.client.download_file(&self.remote_file_path).await {
                            Ok(remote_content) => {
                                if self.files_are_identical(&local_checksum, &remote_content).await {
                                    Ok(SyncAction::NoActionNeeded)
                                } else {
                                    // 文件内容不同，但修改时间相近，这是冲突
                                    Ok(SyncAction::ConflictDetected)
                                }
                            }
                            Err(_) => {
                                // 无法下载远程文件，按本地文件更新
                                Ok(SyncAction::UploadToRemote)
                            }
                        }
                    } else {
                        // 大小不同，检查哪个更新
                        if local.last_modified > remote.last_modified {
                            Ok(SyncAction::UploadToRemote)
                        } else {
                            Ok(SyncAction::DownloadFromRemote)
                        }
                    }
                } else {
                    // 时间差较大，按修改时间决定
                    if local.last_modified > remote.last_modified {
                        Ok(SyncAction::UploadToRemote)
                    } else {
                        Ok(SyncAction::DownloadFromRemote)
                    }
                }
            }
        }
    }

    /// 解决冲突
    pub async fn resolve_conflict(
        &mut self,
        resolution: ConflictResolution,
    ) -> Result<SyncResult, WebDAVError> {
        let local_info = self.get_local_file_info().await?;
        let remote_info = self.client.get_file_info(&self.remote_file_path).await?;
        
        match (local_info, remote_info) {
            (Some(local), Some(remote)) => {
                match resolution {
                    ConflictResolution::KeepLocal => {
                        // 强制上传本地文件
                        self.force_upload().await
                    }
                    ConflictResolution::KeepRemote => {
                        // 强制下载远程文件
                        self.force_download().await
                    }
                    ConflictResolution::KeepBoth => {
                        // 保留两个文件，创建备份
                        self.keep_both_files(local, remote).await
                    }
                    ConflictResolution::Merge => {
                        Err(WebDAVError::InvalidConfig("合并功能暂未实现".to_string()))
                    }
                }
            }
            _ => Err(WebDAVError::InvalidConfig("无法解决冲突：文件状态异常".to_string())),
        }
    }

    /// 保留两个文件的实现
    async fn keep_both_files(
        &mut self,
        local: LocalFileInfo,
        remote: super::client::FileInfo,
    ) -> Result<SyncResult, WebDAVError> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        
        // 创建本地备份文件名
        let local_backup_path = self.create_backup_path(&format!("local_{}", timestamp))?;
        
        // 复制当前本地文件到备份位置
        fs::copy(&self.local_file_path, &local_backup_path).await.map_err(|e| {
            WebDAVError::from_io_error(e)
        })?;
        
        // 下载远程文件
        let remote_content = self.client.download_file(&self.remote_file_path).await?;
        fs::write(&self.local_file_path, &remote_content).await.map_err(|e| {
            WebDAVError::from_io_error(e)
        })?;
        
        // 上传本地备份到远程备份位置  
        let remote_backup_path = format!("{}.local_{}", self.remote_file_path, timestamp);
        let backup_content = fs::read(&local_backup_path).await.map_err(|e| {
            WebDAVError::from_io_error(e)
        })?;
        
        self.client.upload_file(&remote_backup_path, &backup_content).await?;
        
        let local_checksum = self.calculate_data_checksum(&backup_content);
        let remote_checksum = self.calculate_data_checksum(&remote_content);
        
        Ok(SyncResult {
            action: SyncAction::NoActionNeeded,
            success: true,
            message: format!("冲突已解决：保留了两个文件版本。本地备份: {:?}, 远程备份: {}", 
                           local_backup_path, remote_backup_path),
            bytes_transferred: remote_content.len() as u64,
            local_checksum: Some(remote_checksum.clone()),
            remote_checksum: Some(remote_checksum),
            conflict_details: Some(format!(
                "本地版本({}): {}, 远程版本({}): {}",
                local.last_modified.format("%Y-%m-%d %H:%M:%S"),
                local_checksum,
                remote.last_modified.format("%Y-%m-%d %H:%M:%S"),
                self.calculate_data_checksum(&remote_content)
            )),
        })
    }

    /// 创建备份文件路径
    fn create_backup_path(&self, suffix: &str) -> Result<PathBuf, WebDAVError> {
        let _file_name = self.local_file_path
            .file_name()
            .ok_or_else(|| WebDAVError::InvalidConfig("无效的文件名".to_string()))?
            .to_string_lossy();
            
        let file_stem = self.local_file_path
            .file_stem()
            .ok_or_else(|| WebDAVError::InvalidConfig("无效的文件名".to_string()))?
            .to_string_lossy();
            
        let extension = self.local_file_path
            .extension()
            .map(|ext| format!(".{}", ext.to_string_lossy()))
            .unwrap_or_default();
        
        let backup_name = format!("{}_{}{}", file_stem, suffix, extension);
        
        Ok(self.local_file_path.with_file_name(backup_name))
    }

    /// 获取冲突信息
    pub async fn get_conflict_info(&self) -> Result<Option<ConflictInfo>, WebDAVError> {
        let local_info = self.get_local_file_info().await?;
        let remote_info = self.client.get_file_info(&self.remote_file_path).await?;
        
        match (local_info, remote_info) {
            (Some(local), Some(remote)) => {
                // 检查是否真的有冲突
                let action = self.compare_files_detailed(&Some(local.clone()), &Some(remote.clone())).await?;
                
                if matches!(action, SyncAction::ConflictDetected) {
                    let local_content = fs::read(&self.local_file_path).await.map_err(|e| {
                        WebDAVError::from_io_error(e)
                    })?;
                    let remote_content = self.client.download_file(&self.remote_file_path).await?;
                    
                    Ok(Some(ConflictInfo {
                        local_size: local.size,
                        remote_size: remote.size,
                        local_modified: local.last_modified,
                        remote_modified: remote.last_modified,
                        local_checksum: self.calculate_data_checksum(&local_content),
                        remote_checksum: self.calculate_data_checksum(&remote_content),
                    }))
                } else {
                    Ok(None) // 没有冲突
                }
            }
            _ => Ok(None), // 文件状态不符合冲突条件
        }
    }
}

#[derive(Debug, Clone)]
struct LocalFileInfo {
    size: u64,
    last_modified: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_local_file_info() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        
        // 测试不存在的文件
        let config = WebDAVConfig::default();
        let sync = CloudSync::new(config, file_path.clone()).unwrap();
        let info = sync.get_local_file_info().await.unwrap();
        assert!(info.is_none());
        
        // 创建文件并测试
        tokio::fs::write(&file_path, "test content").await.unwrap();
        let info = sync.get_local_file_info().await.unwrap();
        assert!(info.is_some());
        assert_eq!(info.unwrap().size, 12);
    }
}
