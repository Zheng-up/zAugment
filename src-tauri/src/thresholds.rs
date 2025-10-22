use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdValue {
    pub warning: i32,
    pub safe: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusThresholds {
    pub time: ThresholdValue,
    pub balance: ThresholdValue,
    #[serde(rename = "timeMax")]
    pub time_max: i32,
    #[serde(rename = "balanceMax")]
    pub balance_max: i32,
}

impl Default for StatusThresholds {
    fn default() -> Self {
        Self {
            time: ThresholdValue {
                warning: 3,
                safe: 5,
            },
            balance: ThresholdValue {
                warning: 10,
                safe: 30,
            },
            time_max: 365,
            balance_max: 100000,
        }
    }
}

pub struct ThresholdsManager {
    local_file_path: PathBuf,
    cloud_file_path: PathBuf,
}

impl ThresholdsManager {
    pub fn new(data_dir: PathBuf) -> Self {
        let local_file_path = data_dir.join("status_thresholds.json");
        let cloud_file_path = data_dir.join("cloud_status_thresholds.json");
        
        Self {
            local_file_path,
            cloud_file_path,
        }
    }

    /// 保存阈值配置到本地文件
    pub fn save_to_local(&self, thresholds: &StatusThresholds) -> Result<(), String> {
        let json = serde_json::to_string_pretty(thresholds)
            .map_err(|e| format!("序列化阈值配置失败: {}", e))?;
        
        fs::write(&self.local_file_path, json)
            .map_err(|e| format!("写入阈值配置文件失败: {}", e))?;
        
        Ok(())
    }

    /// 从本地文件加载阈值配置
    pub fn load_from_local(&self) -> Result<StatusThresholds, String> {
        if !self.local_file_path.exists() {
            return Ok(StatusThresholds::default());
        }

        let content = fs::read_to_string(&self.local_file_path)
            .map_err(|e| format!("读取阈值配置文件失败: {}", e))?;
        
        let thresholds: StatusThresholds = serde_json::from_str(&content)
            .map_err(|e| format!("解析阈值配置失败: {}", e))?;
        
        Ok(thresholds)
    }

    /// 保存阈值配置到云端文件（用于WebDAV同步）
    pub fn save_to_cloud(&self, thresholds: &StatusThresholds) -> Result<(), String> {
        let json = serde_json::to_string_pretty(thresholds)
            .map_err(|e| format!("序列化阈值配置失败: {}", e))?;
        
        fs::write(&self.cloud_file_path, json)
            .map_err(|e| format!("写入云端阈值配置文件失败: {}", e))?;
        
        Ok(())
    }

    /// 从云端文件加载阈值配置
    pub fn load_from_cloud(&self) -> Result<StatusThresholds, String> {
        if !self.cloud_file_path.exists() {
            return Ok(StatusThresholds::default());
        }

        let content = fs::read_to_string(&self.cloud_file_path)
            .map_err(|e| format!("读取云端阈值配置文件失败: {}", e))?;
        
        let thresholds: StatusThresholds = serde_json::from_str(&content)
            .map_err(|e| format!("解析云端阈值配置失败: {}", e))?;
        
        Ok(thresholds)
    }

    /// 获取本地文件路径
    pub fn get_local_file_path(&self) -> &PathBuf {
        &self.local_file_path
    }

    /// 获取云端文件路径
    pub fn get_cloud_file_path(&self) -> &PathBuf {
        &self.cloud_file_path
    }
}

