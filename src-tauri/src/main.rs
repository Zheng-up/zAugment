// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod augment_oauth;
mod augment_user_info;
mod bookmarks;
mod http_server;
mod outlook_manager;
mod storage;
mod thresholds;
mod webdav;

use augment_oauth::{create_augment_oauth_state, generate_augment_authorize_url, complete_augment_oauth_flow, check_account_ban_status, extract_token_from_session, batch_check_account_status, AugmentOAuthState, AugmentTokenResponse, TokenInfo, TokenStatusResult};
use augment_user_info::{get_user_info, exchange_auth_session_for_app_session};
use bookmarks::{BookmarkManager, Bookmark};
use http_server::HttpServer;
use outlook_manager::{OutlookManager, OutlookCredentials, EmailListResponse, EmailDetailsResponse, AccountStatus as OutlookAccountStatus};
use storage::{LocalFileStorage, TokenStorage};
use thresholds::StatusThresholds;
use webdav::{WebDAVConfig, CloudSync, SecureWebDAVConfig, PasswordManager};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::collections::HashMap;
use std::time::SystemTime;
use tauri::{State, Manager, WebviewWindowBuilder, WebviewUrl, Emitter, Listener};
use tauri_plugin_deep_link::DeepLinkExt;
use chrono;
use std::fs;
use uuid::Uuid;
use sysinfo::{System, SystemExt, ProcessExt};
use rusqlite::Connection;

// Session 导入响应结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenFromSessionResponse {
    pub access_token: String,
    pub tenant_url: String,
    pub user_info: augment_user_info::CompleteUserInfo,
}

// 用户数据同步包结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDataPackage {
    pub version: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub tokens: Option<serde_json::Value>,           // tokens.json 内容
    pub unified_config: Option<UnifiedAppConfig>,    // 统一配置（包含WebDAV配置）
    pub bookmarks: Option<serde_json::Value>,        // 书签数据
    pub status_thresholds: Option<StatusThresholds>, // 账号状态阈值配置
}

impl Default for UserDataPackage {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            timestamp: chrono::Utc::now(),
            tokens: None,
            unified_config: None,
            bookmarks: None,
            status_thresholds: None,
        }
    }
}

impl UserDataPackage {
    /// 收集所有用户数据
    pub async fn collect_from_local(app: &tauri::AppHandle, state: &State<'_, AppState>) -> Result<Self, String> {
        let mut package = UserDataPackage::default();
        
        // 获取数据目录
        let data_dir = get_effective_data_dir(app, state)?;
        
        // 1. 收集 tokens.json
        let tokens_path = data_dir.join("tokens.json");
        if tokens_path.exists() {
            let tokens_content = fs::read_to_string(&tokens_path)
                .map_err(|e| format!("读取tokens.json失败: {}", e))?;
            package.tokens = serde_json::from_str(&tokens_content)
                .map_err(|e| format!("解析tokens.json失败: {}", e))?;
        }
        
        // 2. 收集统一配置（从用户指定目录的配置文件）
        let unified_config = load_unified_config_with_state(app, state);
        package.unified_config = Some(unified_config);
        
        // 4. 收集书签数据
        let bookmarks_path = data_dir.join("bookmarks.json");
        if bookmarks_path.exists() {
            let bookmarks_content = fs::read_to_string(&bookmarks_path)
                .map_err(|e| format!("读取bookmarks.json失败: {}", e))?;
            package.bookmarks = serde_json::from_str(&bookmarks_content)
                .map_err(|e| format!("解析bookmarks.json失败: {}", e))?;
        }

        // 5. 收集阈值配置（从 unified_config 中读取）
        if let Some(ref config) = package.unified_config {
            package.status_thresholds = config.status_thresholds.clone();
        }

        // 更新时间戳
        package.timestamp = chrono::Utc::now();

        Ok(package)
    }
    
    /// 恢复用户数据到本地
    pub async fn restore_to_local(&self, app: &tauri::AppHandle, state: &State<'_, AppState>) -> Result<(), String> {
        let data_dir = get_effective_data_dir(app, state)?;
        
        // 确保数据目录存在
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("创建数据目录失败: {}", e))?;
        
        // 1. 恢复 tokens.json
        if let Some(ref tokens) = self.tokens {
            let tokens_path = data_dir.join("tokens.json");
            let tokens_content = serde_json::to_string_pretty(tokens)
                .map_err(|e| format!("序列化tokens失败: {}", e))?;
            fs::write(&tokens_path, tokens_content)
                .map_err(|e| format!("写入tokens.json失败: {}", e))?;
        }
        
        // 2. 恢复统一配置
        if let Some(ref unified_config) = self.unified_config {
            save_unified_config_with_state(app, unified_config, state)
                .map_err(|e| format!("保存统一配置失败: {}", e))?;
            
            // 同时更新内存中的状态（将WebDAVConfig转换为SecureWebDAVConfig用于内存存储）
            if let Some(ref webdav_config) = unified_config.webdav_config {
                // 先清除旧密码（如果有）
                let _ = state.password_manager.delete_password(&webdav_config.username);
                // 存储新密码到keyring
                let _ = state.password_manager.store_password(&webdav_config.username, &webdav_config.password);
                
                // 创建SecureWebDAVConfig用于内存
                if let Ok(secure_config) = SecureWebDAVConfig::from_config(webdav_config, &state.password_manager) {
                    let mut config_guard = state.webdav_config.lock().unwrap();
                    *config_guard = Some(secure_config);
                }
            }
            
            if let Some(ref custom_dir) = unified_config.custom_data_dir {
                let mut custom_dir_guard = state.custom_data_dir.lock().unwrap();
                *custom_dir_guard = Some(PathBuf::from(custom_dir));
            }
        }
        
        // 4. 恢复书签数据
        if let Some(ref bookmarks) = self.bookmarks {
            let bookmarks_path = data_dir.join("bookmarks.json");
            let bookmarks_content = serde_json::to_string_pretty(bookmarks)
                .map_err(|e| format!("序列化bookmarks失败: {}", e))?;
            fs::write(&bookmarks_path, bookmarks_content)
                .map_err(|e| format!("写入bookmarks.json失败: {}", e))?;
        }

        // 5. 恢复阈值配置（已经包含在 unified_config 中，无需单独处理）
        // 阈值配置会随着 unified_config 一起恢复

        Ok(())
    }
    
    /// 转换为JSON字节数组用于上传
    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec_pretty(self)
            .map_err(|e| format!("序列化用户数据包失败: {}", e))
    }
    
    /// 从JSON字节数组解析
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        serde_json::from_slice(data)
            .map_err(|e| format!("解析用户数据包失败: {}", e))
    }
}
use std::env;

// App Session 缓存结构 (公开以便其他模块使用)
#[derive(Clone)]
pub struct AppSessionCache {
    pub app_session: String,
    pub created_at: SystemTime,
}

// ============ Credit Consumption 相关结构体 ============

/// Credit 消费数据点
#[derive(Debug, Serialize, Deserialize)]
struct CreditDataPoint {
    #[serde(rename(serialize = "group_key", deserialize = "groupKey"))]
    group_key: Option<String>, // 模型名称
    #[serde(rename(serialize = "date_range", deserialize = "dateRange"))]
    date_range: Option<DateRange>,
    #[serde(rename(serialize = "credits_consumed", deserialize = "creditsConsumed"), default = "default_credits_consumed")]
    credits_consumed: String,
}

/// 默认值函数：当 creditsConsumed 字段缺失时返回 "0"
fn default_credits_consumed() -> String {
    "0".to_string()
}

/// 日期范围
#[derive(Debug, Serialize, Deserialize)]
struct DateRange {
    #[serde(rename(serialize = "start_date_iso", deserialize = "startDateIso"))]
    start_date_iso: String,
    #[serde(rename(serialize = "end_date_iso", deserialize = "endDateIso"))]
    end_date_iso: String,
}

/// Credit 消费响应
#[derive(Debug, Serialize, Deserialize)]
struct CreditConsumptionResponse {
    #[serde(rename(serialize = "data_points", deserialize = "dataPoints"), default)]
    data_points: Vec<CreditDataPoint>,
}

/// 批量获取 Credit 消费数据的响应
#[derive(Debug, Serialize, Deserialize)]
struct BatchCreditConsumptionResponse {
    stats_data: CreditConsumptionResponse,
    chart_data: CreditConsumptionResponse,
}

// Global state to store OAuth state and storage managers
struct AppState {
    augment_oauth_state: Mutex<Option<AugmentOAuthState>>,
    http_server: Mutex<Option<HttpServer>>,
    outlook_manager: Mutex<OutlookManager>,
    storage_manager: Arc<Mutex<Option<Arc<LocalFileStorage>>>>,
    custom_data_dir: Arc<Mutex<Option<PathBuf>>>,
    webdav_config: Arc<Mutex<Option<SecureWebDAVConfig>>>,
    cloud_sync: Arc<Mutex<Option<CloudSync>>>,
    password_manager: Arc<PasswordManager>,
    // App session 缓存: key 为 auth_session, value 为缓存的 app_session
    app_session_cache: Arc<Mutex<HashMap<String, AppSessionCache>>>,
}

#[tauri::command]
async fn generate_auth_url(state: State<'_, AppState>) -> Result<String, String> {
    let augment_oauth_state = create_augment_oauth_state();
    let auth_url = generate_augment_authorize_url(&augment_oauth_state)
        .map_err(|e| format!("Failed to generate auth URL: {}", e))?;
    
    // Store the Augment OAuth state
    *state.augment_oauth_state.lock().unwrap() = Some(augment_oauth_state);
    
    Ok(auth_url)
}

#[tauri::command]
async fn generate_augment_auth_url(state: State<'_, AppState>) -> Result<String, String> {
    let augment_oauth_state = create_augment_oauth_state();
    let auth_url = generate_augment_authorize_url(&augment_oauth_state)
        .map_err(|e| format!("Failed to generate Augment auth URL: {}", e))?;
    
    // Store the Augment OAuth state
    *state.augment_oauth_state.lock().unwrap() = Some(augment_oauth_state);
    
    Ok(auth_url)
}



#[tauri::command]
async fn get_token(code: String, state: State<'_, AppState>) -> Result<AugmentTokenResponse, String> {
    let augment_oauth_state = {
        let guard = state.augment_oauth_state.lock().unwrap();
        guard.clone()
            .ok_or("No Augment OAuth state found. Please generate auth URL first.")?
    };

    complete_augment_oauth_flow(&augment_oauth_state, &code)
        .await
        .map_err(|e| format!("Failed to complete OAuth flow: {}", e))
}

#[tauri::command]
async fn get_augment_token(code: String, state: State<'_, AppState>) -> Result<AugmentTokenResponse, String> {
    let augment_oauth_state = {
        let guard = state.augment_oauth_state.lock().unwrap();
        guard.clone()
            .ok_or("No Augment OAuth state found. Please generate auth URL first.")?
    };

    complete_augment_oauth_flow(&augment_oauth_state, &code)
        .await
        .map_err(|e| format!("Failed to complete Augment OAuth flow: {}", e))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAccountStatusResult {
    pub is_banned: bool,
    pub status: String,
    pub error_message: Option<String>,
    pub response_code: Option<u16>,
    pub access_token: String,
    pub tenant_url: String,
}

#[tauri::command]
async fn check_account_status(
    token: String,
    tenant_url: String,
    auth_session: Option<String>,
    token_id: Option<String>
) -> Result<CheckAccountStatusResult, String> {
    let mut current_token = token;
    let mut current_tenant_url = tenant_url;

    // 1. 检测账号状态
    let mut status_result = check_account_ban_status(&current_token, &current_tenant_url)
        .await
        .map_err(|e| format!("Failed to check account status: {}", e))?;

    // 2. 如果检测到 INVALID_TOKEN 且有 auth_session，尝试自动刷新
    if status_result.status == "INVALID_TOKEN" {
        if let Some(ref session) = auth_session {
            println!("Detected INVALID_TOKEN for {:?}, attempting auto-refresh", token_id);

            match extract_token_from_session(session).await {
                Ok(new_token_response) => {
                    println!("Successfully refreshed token for {:?}", token_id);
                    // 更新 token 和 tenant_url
                    current_token = new_token_response.access_token;
                    current_tenant_url = new_token_response.tenant_url;

                    // 重新检测状态
                    match check_account_ban_status(&current_token, &current_tenant_url).await {
                        Ok(new_status) => {
                            status_result = new_status;
                            status_result.error_message = Some(format!(
                                "Token was invalid but successfully auto-refreshed. New status: {}",
                                status_result.status
                            ));
                        }
                        Err(e) => {
                            println!("Failed to check status after refresh: {}", e);
                        }
                    }
                }
                Err(err) => {
                    println!("Failed to refresh token for {:?}: {}", token_id, err);
                    // 如果刷新失败原因是 SESSION_ERROR_OR_ACCOUNT_BANNED，视为账号封禁
                    if err.contains("SESSION_ERROR_OR_ACCOUNT_BANNED") {
                        status_result.status = "SUSPENDED".to_string();
                        status_result.is_banned = true;
                        status_result.error_message = Some(
                            "Account is suspended (detected during token refresh)".to_string()
                        );
                    } else {
                        status_result.error_message = Some(format!(
                            "Token is invalid. Failed to auto-refresh: {}",
                            err
                        ));
                    }
                }
            }
        } else {
            println!("Token {:?} is invalid but no auth_session available", token_id);
            status_result.error_message = Some(
                "Token is invalid. No auth_session available for auto-refresh".to_string()
            );
        }
    }

    // 3. 返回结果（包含可能已更新的 token 和 tenant_url）
    Ok(CheckAccountStatusResult {
        is_banned: status_result.is_banned,
        status: status_result.status,
        error_message: status_result.error_message,
        response_code: status_result.response_code,
        access_token: current_token,
        tenant_url: current_tenant_url,
    })
}

#[tauri::command]
async fn batch_check_tokens_status(
    tokens: Vec<TokenInfo>,
) -> Result<Vec<TokenStatusResult>, String> {
    batch_check_account_status(tokens)
        .await
        .map_err(|e| format!("Failed to batch check tokens status: {}", e))
}

/// 批量获取 Credit 消费数据(stats 和 chart),使用缓存的 app_session
#[tauri::command]
async fn fetch_batch_credit_consumption(
    auth_session: String,
    state: State<'_, AppState>,
) -> Result<BatchCreditConsumptionResponse, String> {
    // 1. 检查缓存中是否有有效的 app_session
    let cached_app_session = {
        let cache = state.app_session_cache.lock().unwrap();
        cache.get(&auth_session).map(|c| c.app_session.clone())
    };

    // 2. 获取或刷新 app_session
    let app_session = if let Some(app_session) = cached_app_session {
        println!("[Credit] Using cached app_session");
        println!("[Credit] App session (first 20 chars): {}...", &app_session.chars().take(20).collect::<String>());
        app_session
    } else {
        // 使用 auth_session 交换新的 app_session
        println!("[Credit] Exchanging auth_session for new app_session");
        println!("[Credit] Auth session (first 20 chars): {}...", &auth_session.chars().take(20).collect::<String>());

        let new_app_session = exchange_auth_session_for_app_session(&auth_session).await?;

        println!("[Credit] Successfully exchanged app_session");
        println!("[Credit] New app session (first 20 chars): {}...", &new_app_session.chars().take(20).collect::<String>());

        // 缓存新的 app_session
        {
            let mut cache = state.app_session_cache.lock().unwrap();
            cache.insert(
                auth_session.clone(),
                AppSessionCache {
                    app_session: new_app_session.clone(),
                    created_at: SystemTime::now(),
                },
            );
            println!("[Credit] App session cached for future use");
        }

        new_app_session
    };

    // 3. 创建 HTTP 客户端
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    // 4. 并行获取两个数据
    let stats_url = "https://app.augmentcode.com/api/credit-consumption?groupBy=NONE&granularity=DAY&billingCycle=CURRENT_BILLING_CYCLE";
    let chart_url = "https://app.augmentcode.com/api/credit-consumption?groupBy=MODEL_NAME&granularity=TOTAL&billingCycle=CURRENT_BILLING_CYCLE";

    println!("[Credit] Fetching stats from: {}", stats_url);
    println!("[Credit] Fetching chart from: {}", chart_url);
    println!("[Credit] Cookie header: _session={}...", &app_session.chars().take(20).collect::<String>());

    let (stats_result, chart_result) = tokio::join!(
        async {
            let response = client
                .get(stats_url)
                .header("Cookie", format!("_session={}", urlencoding::encode(&app_session)))
                .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .header("Accept", "application/json")
                .send()
                .await
                .map_err(|e| format!("Failed to fetch stats data: {}", e))?;

            let status = response.status();
            println!("[Credit] Stats API response status: {}", status);

            if !status.is_success() {
                let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                println!("[Credit] Stats API error body: {}", error_body);
                return Err(format!("Stats API returned status {}: {}", status, error_body));
            }

            let response_text = response.text().await
                .map_err(|e| format!("Failed to read stats response body: {}", e))?;

            println!("[Credit] Stats response (first 200 chars): {}...",
                &response_text.chars().take(200).collect::<String>());

            serde_json::from_str::<CreditConsumptionResponse>(&response_text)
                .map_err(|e| format!("Failed to parse stats response: {}. Response body: {}", e, response_text))
        },
        async {
            let response = client
                .get(chart_url)
                .header("Cookie", format!("_session={}", urlencoding::encode(&app_session)))
                .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .header("Accept", "application/json")
                .send()
                .await
                .map_err(|e| format!("Failed to fetch chart data: {}", e))?;

            let status = response.status();
            println!("[Credit] Chart API response status: {}", status);

            if !status.is_success() {
                let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                println!("[Credit] Chart API error body: {}", error_body);
                return Err(format!("Chart API returned status {}: {}", status, error_body));
            }

            let response_text = response.text().await
                .map_err(|e| format!("Failed to read chart response body: {}", e))?;

            println!("[Credit] Chart response (first 200 chars): {}...",
                &response_text.chars().take(200).collect::<String>());

            serde_json::from_str::<CreditConsumptionResponse>(&response_text)
                .map_err(|e| format!("Failed to parse chart response: {}. Response body: {}", e, response_text))
        }
    );

    let stats_data = stats_result?;
    let chart_data = chart_result?;

    Ok(BatchCreditConsumptionResponse {
        stats_data,
        chart_data,
    })
}

#[tauri::command]
async fn add_token_from_session(session: String, app: tauri::AppHandle) -> Result<TokenFromSessionResponse, String> {
    // 1. 从 session 提取 token
    let _ = app.emit("session-import-progress", "sessionImportExtractingToken");
    let token_response = extract_token_from_session(&session).await?;

    // 2. 获取用户信息
    let _ = app.emit("session-import-progress", "sessionImportGettingUserInfo");
    let user_info = get_user_info(&session).await?;

    let _ = app.emit("session-import-progress", "sessionImportComplete");

    Ok(TokenFromSessionResponse {
        access_token: token_response.access_token,
        tenant_url: token_response.tenant_url,
        user_info,
    })
}

#[tauri::command]
async fn open_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener().open_url(url, None::<&str>)
        .map_err(|e| format!("Failed to open URL: {}", e))
}

#[tauri::command]
async fn save_tokens_json(json_string: String, app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    use std::fs;
    use std::io::Write;

    // 获取有效的数据目录（优先使用自定义目录）
    let data_dir = get_effective_data_dir(&app, &state)?;

    // 确保目录存在并且可写
    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create data directory: {}", e))?;

    let storage_path = data_dir.join("tokens.json");

    // 使用同一目录下的临时文件，确保在同一文件系统上
    let temp_path = data_dir.join(format!("tokens.{}.tmp", uuid::Uuid::new_v4()));

    // 基本的 JSON 格式验证
    serde_json::from_str::<serde_json::Value>(&json_string)
        .map_err(|e| format!("Invalid JSON format: {}", e))?;

    // 原子性写入：先写临时文件，再重命名
    let write_result = (|| -> Result<(), String> {
        let mut temp_file = fs::File::create(&temp_path)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        temp_file.write_all(json_string.as_bytes())
            .map_err(|e| format!("Failed to write temp file: {}", e))?;

        temp_file.sync_all()
            .map_err(|e| format!("Failed to sync temp file: {}", e))?;

        // 显式关闭文件
        drop(temp_file);

        Ok(())
    })();

    // 如果写入失败，清理临时文件
    if let Err(e) = write_result {
        let _ = fs::remove_file(&temp_path);
        return Err(e);
    }

    // 尝试原子性重命名，如果失败则使用复制+删除的方式
    let rename_result = fs::rename(&temp_path, &storage_path);

    if let Err(rename_err) = rename_result {
        // 如果 rename 失败（可能是跨文件系统），尝试复制+删除
        println!("Rename failed, trying copy+delete: {}", rename_err);

        match fs::copy(&temp_path, &storage_path) {
            Ok(_) => {
                // 复制成功，删除临时文件
                let _ = fs::remove_file(&temp_path);
                Ok(())
            }
            Err(copy_err) => {
                // 复制也失败，清理临时文件并返回错误
                let _ = fs::remove_file(&temp_path);
                Err(format!("Failed to save file (rename failed: {}, copy failed: {})", rename_err, copy_err))
            }
        }
    } else {
        Ok(())
    }
}


#[tauri::command]
async fn load_tokens_json(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    use std::fs;

    // 获取有效的数据目录（优先使用自定义目录）
    let effective_data_dir = get_effective_data_dir(&app, &state)?;
    let effective_storage_path = effective_data_dir.join("tokens.json");

    // 如果有效目录中存在文件，直接读取
    if effective_storage_path.exists() {
        let content = fs::read_to_string(&effective_storage_path)
            .map_err(|e| format!("Failed to read tokens file: {}", e))?;

        println!("从有效目录读取到的文件内容: {}", content);

        // 如果文件为空，返回空数组的 JSON
        if content.trim().is_empty() {
            return Ok("[]".to_string());
        }

        return process_token_content(content);
    }

    // 如果有效目录中没有文件，尝试从默认目录读取（用于迁移）
    let default_app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let default_storage_path = default_app_data_dir.join("tokens.json");

    println!("尝试读取默认文件路径: {:?}", default_storage_path);

    // 尝试从默认目录读取（用于数据迁移）
    if default_storage_path.exists() {
        let content = fs::read_to_string(&default_storage_path)
            .map_err(|e| format!("Failed to read tokens file: {}", e))?;

        println!("从默认目录读取到的文件内容: {}", content);

        // 如果文件为空，返回空数组的 JSON
        if content.trim().is_empty() {
            return Ok("[]".to_string());
        }

        // 如果用户设置了自定义目录，迁移数据
        if effective_data_dir != default_app_data_dir {
            // 创建自定义目录（如果不存在）
            fs::create_dir_all(&effective_data_dir)
                .map_err(|e| format!("Failed to create custom data directory: {}", e))?;

            // 将文件迁移到自定义目录
            fs::copy(&default_storage_path, &effective_storage_path)
                .map_err(|e| format!("Failed to migrate tokens file: {}", e))?;

            println!("文件已迁移到自定义目录: {:?}", effective_storage_path);
        }

        return process_token_content(content);
    }

    // 如果默认目录没有文件，尝试从旧目录读取
    println!("默认目录中没有文件，尝试从旧目录读取...");

    // 构造旧的应用数据目录路径
    let old_app_data_dir = get_old_app_data_dir()?;
    let old_storage_path = old_app_data_dir.join("tokens.json");

    println!("尝试读取旧文件路径: {:?}", old_storage_path);

    if old_storage_path.exists() {
        let content = fs::read_to_string(&old_storage_path)
            .map_err(|e| format!("Failed to read old tokens file: {}", e))?;

        println!("从旧目录读取到的文件内容: {}", content);

        // 如果文件为空，返回空数组的 JSON
        if content.trim().is_empty() {
            return Ok("[]".to_string());
        }

        // 创建有效目录（如果不存在）
        fs::create_dir_all(&effective_data_dir)
            .map_err(|e| format!("Failed to create effective data directory: {}", e))?;

        // 将文件迁移到有效目录
        fs::copy(&old_storage_path, &effective_storage_path)
            .map_err(|e| format!("Failed to migrate tokens file: {}", e))?;

        println!("文件已迁移到有效目录: {:?}", effective_storage_path);

        return process_token_content(content);
    }

    // 所有目录都没有文件
    println!("所有目录都没有找到 tokens.json 文件");
    Ok("[]".to_string())
}

// 获取旧的应用数据目录
fn get_old_app_data_dir() -> Result<PathBuf, String> {
    use std::env;
    use std::path::PathBuf;

    let home_dir = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .map_err(|_| "Failed to get home directory".to_string())?;

    // 旧的 identifier: com.capslockCube.augment-token-manager
    let old_path = if cfg!(target_os = "windows") {
        // Windows: %APPDATA%\com.capslockCube.augment-token-manager
        PathBuf::from(home_dir)
            .join("AppData")
            .join("Roaming")
            .join("com.capslockCube.augment-token-manager")
    } else if cfg!(target_os = "macos") {
        // macOS: ~/Library/Application Support/com.capslockCube.augment-token-manager
        PathBuf::from(home_dir)
            .join("Library")
            .join("Application Support")
            .join("com.capslockCube.augment-token-manager")
    } else {
        // Linux: ~/.config/com.capslockCube.augment-token-manager
        PathBuf::from(home_dir)
            .join(".config")
            .join("com.capslockCube.augment-token-manager")
    };

    Ok(old_path)
}

// 处理 token 内容的通用函数
fn process_token_content(content: String) -> Result<String, String> {
    // 尝试解析 JSON 内容
    match serde_json::from_str::<serde_json::Value>(&content) {
        Ok(value) => {
            // 如果解析成功，检查是否需要转换格式
            match value {
                serde_json::Value::Array(_) => {
                    // 如果已经是数组格式，直接返回原内容
                    Ok(content)
                }
                serde_json::Value::Object(ref obj) => {
                    // 检查是否是旧格式 {tokens: [...]}
                    if let Some(tokens_array) = obj.get("tokens") {
                        if tokens_array.is_array() {
                            // 旧格式，提取 tokens 数组
                            Ok(serde_json::to_string_pretty(tokens_array)
                                .map_err(|e| format!("Failed to serialize tokens: {}", e))?)
                        } else {
                            Ok("[]".to_string())
                        }
                    } else {
                        // 如果是单个对象格式，包装成数组
                        let array = serde_json::Value::Array(vec![value]);
                        Ok(serde_json::to_string_pretty(&array)
                            .map_err(|e| format!("Failed to serialize tokens: {}", e))?)
                    }
                }
                _ => {
                    // 其他格式，返回空数组
                    Ok("[]".to_string())
                }
            }
        }
        Err(_) => {
            // 如果 JSON 解析失败，可能是其他格式的旧数据，返回空数组
            Ok("[]".to_string())
        }
    }
}



// Bookmark management commands
#[tauri::command]
async fn add_bookmark(
    name: String,
    url: String,
    description: Option<String>,
    category: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let bookmark_manager = BookmarkManager::new(&app)
        .map_err(|e| format!("Failed to initialize bookmark manager: {}", e))?;

    bookmark_manager.add_bookmark(name, url, description, category)
        .map_err(|e| format!("Failed to add bookmark: {}", e))
}

#[tauri::command]
async fn update_bookmark(
    id: String,
    name: String,
    url: String,
    description: Option<String>,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    let bookmark_manager = BookmarkManager::new(&app)
        .map_err(|e| format!("Failed to initialize bookmark manager: {}", e))?;

    bookmark_manager.update_bookmark(&id, name, url, description)
        .map_err(|e| format!("Failed to update bookmark: {}", e))
}

#[tauri::command]
async fn delete_bookmark(
    id: String,
    app: tauri::AppHandle,
) -> Result<bool, String> {
    let bookmark_manager = BookmarkManager::new(&app)
        .map_err(|e| format!("Failed to initialize bookmark manager: {}", e))?;

    bookmark_manager.remove_bookmark(&id)
        .map_err(|e| format!("Failed to delete bookmark: {}", e))
}

#[tauri::command]
async fn get_bookmarks(
    category: String,
    app: tauri::AppHandle,
) -> Result<Vec<Bookmark>, String> {
    let bookmark_manager = BookmarkManager::new(&app)
        .map_err(|e| format!("Failed to initialize bookmark manager: {}", e))?;

    bookmark_manager.get_bookmarks_by_category(&category)
        .map_err(|e| format!("Failed to get bookmarks: {}", e))
}

#[tauri::command]
async fn get_all_bookmarks(
    app: tauri::AppHandle,
) -> Result<Vec<Bookmark>, String> {
    let bookmark_manager = BookmarkManager::new(&app)
        .map_err(|e| format!("Failed to initialize bookmark manager: {}", e))?;

    bookmark_manager.get_all_bookmarks()
        .map_err(|e| format!("Failed to get all bookmarks: {}", e))
}







#[tauri::command]
async fn open_internal_browser(
    app: tauri::AppHandle,
    url: String,
    title: Option<String>
) -> Result<String, String> {
    let window_label = format!("browser_{}", chrono::Utc::now().timestamp());

    let _window = WebviewWindowBuilder::new(
        &app,
        &window_label,
        WebviewUrl::External(url.parse().map_err(|e| format!("Invalid URL: {}", e))?)
    )
    .title(&title.unwrap_or_else(|| "内置浏览器".to_string()))
    .inner_size(1000.0, 700.0)
    .center()
    .resizable(true)
    .build()
    .map_err(|e| format!("Failed to create browser window: {}", e))?;

    Ok(window_label)
}

#[tauri::command]
async fn close_window(app: tauri::AppHandle, window_label: String) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(&window_label) {
        window.close().map_err(|e| format!("Failed to close window: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn open_with_chrome(url: String) -> Result<(), String> {
    use std::process::Command;
    use std::env;
    use std::path::Path;

    // Chrome 的常见安装路径
    let user_profile_chrome = format!("{}\\AppData\\Local\\Google\\Chrome\\Application\\chrome.exe",
                                     env::var("USERPROFILE").unwrap_or_default());
    let chrome_paths = vec![
        // Windows 路径
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        // 用户目录路径
        &user_profile_chrome,
    ];

    // 查找 Chrome
    let mut chrome_path = None;
    for path in chrome_paths {
        if Path::new(path).exists() {
            chrome_path = Some(path);
            break;
        }
    }

    match chrome_path {
        Some(path) => {
            match Command::new(path)
                .arg(&url)
                .spawn() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to launch Chrome: {}", e))
            }
        }
        None => Err("Chrome not found in default locations".to_string())
    }
}

#[tauri::command]
async fn open_with_custom_browser(url: String, browser_path: String) -> Result<(), String> {
    use std::process::Command;
    use std::path::Path;

    if !Path::new(&browser_path).exists() {
        return Err("Browser path does not exist".to_string());
    }

    match Command::new(&browser_path)
        .arg(&url)
        .spawn() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to launch browser: {}", e))
    }
}

#[derive(serde::Deserialize)]
struct FileFilter {
    name: String,
    extensions: Vec<String>,
}

// 窗口控制命令
#[tauri::command]
async fn minimize_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.minimize().map_err(|e| format!("Failed to minimize window: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn maximize_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.maximize().map_err(|e| format!("Failed to maximize window: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn unmaximize_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.unmaximize().map_err(|e| format!("Failed to unmaximize window: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn close_app_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.close().map_err(|e| format!("Failed to close window: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn start_drag(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.start_dragging().map_err(|e| format!("Failed to start dragging: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn set_window_size(app: tauri::AppHandle, width: u32, height: u32) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        use tauri::Size;
        let size = Size::Physical(tauri::PhysicalSize { width, height });
        window.set_size(size).map_err(|e| format!("Failed to set window size: {}", e))?;
        window.center().map_err(|e| format!("Failed to center window: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn toggle_fullscreen(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let is_fullscreen = window.is_fullscreen().map_err(|e| format!("Failed to get fullscreen state: {}", e))?;
        window.set_fullscreen(!is_fullscreen).map_err(|e| format!("Failed to toggle fullscreen: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn set_fullscreen(app: tauri::AppHandle, fullscreen: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.set_fullscreen(fullscreen).map_err(|e| format!("Failed to set fullscreen: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn get_window_state(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    if let Some(window) = app.get_webview_window("main") {
        let is_maximized = window.is_maximized().map_err(|e| format!("Failed to get maximized state: {}", e))?;
        let is_fullscreen = window.is_fullscreen().map_err(|e| format!("Failed to get fullscreen state: {}", e))?;

        Ok(serde_json::json!({
            "isMaximized": is_maximized,
            "isFullscreen": is_fullscreen
        }))
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
async fn get_window_size(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    if let Some(window) = app.get_webview_window("main") {
        let size = window.inner_size().map_err(|e| format!("Failed to get window size: {}", e))?;

        Ok(serde_json::json!({
            "width": size.width,
            "height": size.height
        }))
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
async fn save_file_dialog(default_filename: String) -> Result<Option<String>, String> {
    use std::process::Command;

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                &format!(r#"
                Add-Type -AssemblyName System.Windows.Forms
                $saveFileDialog = New-Object System.Windows.Forms.SaveFileDialog
                $saveFileDialog.Filter = "JSON文件 (*.json)|*.json|所有文件 (*.*)|*.*"
                $saveFileDialog.Title = "导出账号数据"
                $saveFileDialog.FileName = "{}"
                $saveFileDialog.DefaultExt = "json"
                $result = $saveFileDialog.ShowDialog()
                if ($result -eq [System.Windows.Forms.DialogResult]::OK) {{
                    Write-Output $saveFileDialog.FileName
                }}
                "#, default_filename)
            ])
            .output();

        match output {
            Ok(output) => {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if path_str.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(path_str))
                }
            }
            Err(e) => Err(format!("打开文件保存对话框失败: {}", e))
        }
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("osascript")
            .args(&[
                "-e",
                &format!(r#"choose file name with prompt "导出账号数据" default name "{}""#, default_filename)
            ])
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let path = stdout.trim();
                if path.is_empty() {
                    Ok(None)
                } else {
                    let path = path.replace("alias ", "").replace(":", "/");
                    let path = if path.starts_with("Macintosh HD") {
                        path.replace("Macintosh HD", "")
                    } else {
                        path
                    };
                    Ok(Some(path))
                }
            }
            Err(e) => Err(format!("打开文件保存对话框失败: {}", e))
        }
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("zenity")
            .args(&[
                "--file-selection",
                "--save",
                "--filename",
                &default_filename,
                "--title=导出账号数据"
            ])
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let path = stdout.trim();
                if path.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(path.to_string()))
                }
            }
            Err(e) => Err(format!("打开文件保存对话框失败: {}", e))
        }
    }
}

#[tauri::command]
async fn write_file_content(file_path: String, content: String) -> Result<(), String> {
    use std::fs;
    use std::path::Path;

    let target_path = Path::new(&file_path);

    // 确保父目录存在
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }

    // 使用同一目录下的唯一临时文件名，确保在同一文件系统上
    let parent_dir = target_path.parent()
        .ok_or("无法获取父目录")?;
    let file_name = target_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file");
    let temp_path = parent_dir.join(format!("{}.{}.tmp", file_name, uuid::Uuid::new_v4()));

    // 写入临时文件
    fs::write(&temp_path, &content)
        .map_err(|e| format!("写入临时文件失败: {}", e))?;

    // 尝试原子性重命名，如果失败则使用复制+删除的方式
    match fs::rename(&temp_path, target_path) {
        Ok(_) => Ok(()),
        Err(rename_err) => {
            // 如果 rename 失败（可能是跨文件系统），尝试复制+删除
            match fs::copy(&temp_path, target_path) {
                Ok(_) => {
                    let _ = fs::remove_file(&temp_path);
                    Ok(())
                }
                Err(copy_err) => {
                    let _ = fs::remove_file(&temp_path);
                    Err(format!("写入文件失败 (rename: {}, copy: {})", rename_err, copy_err))
                }
            }
        }
    }
}

#[tauri::command]
async fn select_file(_app: tauri::AppHandle, _filters: Vec<FileFilter>) -> Result<Option<String>, String> {
    use std::process::Command;

    // 使用Windows的文件选择对话框
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            r#"
            Add-Type -AssemblyName System.Windows.Forms
            $openFileDialog = New-Object System.Windows.Forms.OpenFileDialog
            $openFileDialog.Filter = "可执行文件 (*.exe)|*.exe|所有文件 (*.*)|*.*"
            $openFileDialog.Title = "选择浏览器可执行文件"
            $openFileDialog.InitialDirectory = "C:\Program Files"
            $result = $openFileDialog.ShowDialog()
            if ($result -eq [System.Windows.Forms.DialogResult]::OK) {
                Write-Output $openFileDialog.FileName
            }
            "#
        ])
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let path = stdout.trim();
            if path.is_empty() {
                Ok(None)
            } else {
                Ok(Some(path.to_string()))
            }
        }
        Err(e) => {
            // 如果PowerShell失败，返回常见浏览器路径
            let common_browsers = vec![
                r"C:\Program Files\Mozilla Firefox\firefox.exe",
                r"C:\Program Files (x86)\Mozilla Firefox\firefox.exe",
                r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
                r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
            ];

            for browser_path in common_browsers {
                if std::path::Path::new(browser_path).exists() {
                    return Ok(Some(browser_path.to_string()));
                }
            }

            Err(format!("Failed to open file dialog: {}", e))
        }
    }
}

#[tauri::command]
async fn get_customer_info(token: String) -> Result<String, String> {
    let url = format!("https://portal.withorb.com/api/v1/customer_from_link?token={}", token);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Accept-Charset", "utf-8")
        .header("Connection", "keep-alive")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .send()
        .await
        .map_err(|e| format!("Failed to make API request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read response bytes: {}", e))?;

        let response_text = String::from_utf8_lossy(&bytes).to_string();

        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json_value) => {
                match serde_json::to_string_pretty(&json_value) {
                    Ok(formatted) => Ok(formatted),
                    Err(_) => Ok(response_text),
                }
            }
            Err(_) => Ok(response_text),
        }
    } else {
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        Err(format!("API request failed with status {}: {}", status, response_text))
    }
}

#[tauri::command]
async fn get_subscriptions_from_link(token: String) -> Result<String, String> {
    let url = format!("https://portal.withorb.com/api/v1/subscriptions_from_link?token={}", token);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Accept-Charset", "utf-8")
        .header("Connection", "keep-alive")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .send()
        .await
        .map_err(|e| format!("Failed to make API request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read response bytes: {}", e))?;

        let response_text = String::from_utf8_lossy(&bytes).to_string();

        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json_value) => {
                match serde_json::to_string_pretty(&json_value) {
                    Ok(formatted) => Ok(formatted),
                    Err(_) => Ok(response_text),
                }
            }
            Err(_) => Ok(response_text),
        }
    } else {
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        Err(format!("API request failed with status {}: {}", status, response_text))
    }
}



#[tauri::command]
async fn get_ledger_summary(customer_id: String, pricing_unit_id: String, token: String) -> Result<String, String> {
    let url = format!("https://portal.withorb.com/api/v1/customers/{}/ledger_summary?pricing_unit_id={}&token={}",
                     customer_id, pricing_unit_id, token);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Accept-Charset", "utf-8")
        .header("Connection", "keep-alive")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .send()
        .await
        .map_err(|e| format!("Failed to make API request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read response bytes: {}", e))?;

        let response_text = String::from_utf8_lossy(&bytes).to_string();

        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json_value) => {
                match serde_json::to_string_pretty(&json_value) {
                    Ok(formatted) => Ok(formatted),
                    Err(_) => Ok(response_text),
                }
            }
            Err(_) => Ok(response_text),
        }
    } else {
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        Err(format!("API request failed with status {}: {}", status, response_text))
    }
}

#[tauri::command]
async fn test_api_call() -> Result<String, String> {
    let url = "https://portal.withorb.com/api/v1/customer_from_link?token=ImRhUHFhU3ZtelpKdEJrUVci.1konHDs_4UqVUJWcxaZpKV4nQik";

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Accept-Charset", "utf-8")
        .header("Connection", "keep-alive")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-origin")
        .send()
        .await
        .map_err(|e| format!("Failed to make API request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        // 尝试获取JSON并格式化
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read response bytes: {}", e))?;

        // 确保使用UTF-8解码
        let response_text = String::from_utf8_lossy(&bytes).to_string();

        // 尝试解析并格式化JSON
        match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(json_value) => {
                // 格式化JSON输出
                match serde_json::to_string_pretty(&json_value) {
                    Ok(formatted) => Ok(formatted),
                    Err(_) => Ok(response_text), // 如果格式化失败，返回原始文本
                }
            }
            Err(_) => Ok(response_text), // 如果不是有效JSON，返回原始文本
        }
    } else {
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        Err(format!("API request failed with status {}: {}", status, response_text))
    }
}

// 版本检查相关结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct AppVersionInfo {
    pub current_version: String,
    pub commit_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub published_at: String,
    pub assets: Vec<GitHubAsset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCheckResult {
    pub has_update: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_notes: String,
    pub download_url: String,
    pub asset_name: String,
}

// 获取当前应用版本信息
#[tauri::command]
async fn get_app_version() -> Result<AppVersionInfo, String> {
    Ok(AppVersionInfo {
        current_version: env!("CARGO_PKG_VERSION").to_string(),
        commit_hash: option_env!("GIT_HASH").map(|s| s.to_string()),
    })
}

// 检查更新
#[tauri::command]
async fn check_for_updates() -> Result<UpdateCheckResult, String> {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("检查更新: 当前版本 {}", current_version);

    // 获取最新 Release 信息，带重试机制
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("ZAugment-UpdateChecker/1.0")
        .build()
        .map_err(|e| {
            let error_msg = format!("Failed to create HTTP client: {}", e);
            println!("错误: {}", error_msg);
            error_msg
        })?;

    // GitHub token - 从环境变量读取，如果没有则使用空字符串
    let github_token = std::env::var("GITHUB_TOKEN").unwrap_or_default();

    // 重试机制：最多重试3次，每次间隔递增
    let mut last_error = String::new();
    for attempt in 1..=3 {
        println!("正在请求 GitHub API... (尝试 {}/3)", attempt);

        let mut request = client
            .get("https://api.github.com/repos/Zheng-up/zAugment/releases/latest")
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "ZAugment/1.0");

        // 添加认证头以避免速率限制（如果有 token）
        if !github_token.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", github_token));
        }

        let response = request.send().await;

        match response {
            Ok(resp) => {
                println!("GitHub API 响应状态: {}", resp.status());

                if resp.status().is_success() {
                    // 成功响应，继续处理
                    let release: GitHubRelease = resp
                        .json()
                        .await
                        .map_err(|e| {
                            let error_msg = format!("Failed to parse release JSON: {}", e);
                            println!("JSON 解析错误: {}", error_msg);
                            error_msg
                        })?;

                    let latest_version = release.tag_name.trim_start_matches('v');
                    println!("最新版本: {}, 当前版本: {}", latest_version, current_version);

                    let has_update = is_newer_version(latest_version, current_version);
                    println!("是否有更新: {}", has_update);

                    // 获取适合当前平台的下载链接
                    let (download_url, asset_name) = get_platform_download_url(&release.assets);
                    println!("下载链接: {}, 文件名: {}", download_url, asset_name);

                    let result = UpdateCheckResult {
                        has_update,
                        current_version: current_version.to_string(),
                        latest_version: latest_version.to_string(),
                        release_notes: parse_release_notes(&release.body),
                        download_url,
                        asset_name,
                    };

                    println!("检查更新完成: {:?}", result);
                    return Ok(result);
                } else if resp.status() == 403 {
                    // API 速率限制
                    last_error = format!("GitHub API 速率限制，请稍后重试 (状态码: {})", resp.status());
                    println!("API 速率限制: {}", last_error);
                } else {
                    // 其他错误
                    last_error = format!("GitHub API 请求失败，状态码: {}", resp.status());
                    println!("API 请求失败: {}", last_error);
                }
            }
            Err(e) => {
                last_error = format!("网络请求失败: {}", e);
                println!("网络请求错误: {}", last_error);
            }
        }

        // 如果不是最后一次尝试，等待后重试
        if attempt < 3 {
            let delay = std::time::Duration::from_secs(attempt * 2); // 2s, 4s
            println!("等待 {}s 后重试...", delay.as_secs());
            tokio::time::sleep(delay).await;
        }
    }

    // 所有重试都失败了
    Err(format!("检查更新失败: {}", last_error))
}

// 版本比较函数
fn is_newer_version(latest: &str, current: &str) -> bool {
    let latest_parts: Vec<u32> = latest.split('.').filter_map(|s| s.parse().ok()).collect();
    let current_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();

    let max_len = latest_parts.len().max(current_parts.len());

    for i in 0..max_len {
        let latest_part = latest_parts.get(i).unwrap_or(&0);
        let current_part = current_parts.get(i).unwrap_or(&0);

        if latest_part > current_part {
            return true;
        } else if latest_part < current_part {
            return false;
        }
    }

    false
}

// 获取平台特定的下载链接
fn get_platform_download_url(assets: &[GitHubAsset]) -> (String, String) {
    let platform = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    // 根据平台和架构选择合适的资源
    let preferred_asset = match platform {
        "windows" => assets.iter().find(|asset|
            asset.name.contains("windows") && asset.name.ends_with(".exe")
        ),
        "macos" => {
            if arch == "aarch64" {
                assets.iter().find(|asset|
                    asset.name.contains("aarch64") && asset.name.ends_with(".dmg")
                )
            } else {
                assets.iter().find(|asset|
                    asset.name.contains("x64") && asset.name.ends_with(".dmg")
                )
            }
        },
        "linux" => assets.iter().find(|asset|
            asset.name.contains("amd64") && asset.name.ends_with(".deb")
        ).or_else(|| assets.iter().find(|asset|
            asset.name.contains("x86_64") && asset.name.ends_with(".rpm")
        )),
        _ => None,
    };

    if let Some(asset) = preferred_asset {
        (asset.browser_download_url.clone(), asset.name.clone())
    } else if let Some(first_asset) = assets.first() {
        (first_asset.browser_download_url.clone(), first_asset.name.clone())
    } else {
        (String::new(), String::new())
    }
}

// 解析 Release Notes
fn parse_release_notes(body: &str) -> String {
    if body.is_empty() {
        return "暂无更新说明".to_string();
    }

    println!("原始 release notes: {}", body);

    // 只提取更新内容部分，不包含发布标题
    let lines: Vec<&str> = body.lines().collect();
    let mut result = Vec::new();
    let mut in_update_content = false;

    for line in lines {
        let trimmed = line.trim();

        // 检测更新内容部分开始（支持带 emoji 和不带 emoji 的标题）
        if trimmed.starts_with("### 更新内容") ||
           trimmed.starts_with("###更新内容") ||
           trimmed.contains("更新内容") && trimmed.starts_with("###") {
            in_update_content = true;
            continue; // 不包含标题行
        }

        // 如果遇到其他三级标题，停止收集
        if trimmed.starts_with("### ") && in_update_content {
            break;
        }

        // 在更新内容部分内，收集列表项和空行
        if in_update_content {
            if trimmed.starts_with("- ") || trimmed.starts_with("* ") ||
               trimmed.chars().next().map_or(false, |c| c.is_ascii_digit()) ||
               trimmed.is_empty() {
                result.push(line);
            }
        }
    }

    let final_result = result.join("\n");
    println!("解析后的 release notes: {}", final_result);

    if result.is_empty() {
        return "暂无更新说明".to_string();
    }

    final_result
}

#[tauri::command]
async fn open_data_folder(
    app: tauri::AppHandle,
) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    // Open folder using system default file manager
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&app_data_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&app_data_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&app_data_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
async fn create_jetbrains_token_file(
    editor_type: String,
    token_data: String,
) -> Result<String, String> {
    use std::fs;
    use std::env;
    use std::path::PathBuf;

    // 获取用户主目录
    let home_dir = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .map_err(|_| "Failed to get home directory".to_string())?;

    let augment_dir = PathBuf::from(&home_dir).join(".augment");

    // 确保 .augment 目录存在
    fs::create_dir_all(&augment_dir)
        .map_err(|e| format!("Failed to create .augment directory: {}", e))?;

    // 创建文件路径
    let file_name = format!("{}_token.json", editor_type);
    let file_path = augment_dir.join(&file_name);

    // 写入文件
    fs::write(&file_path, token_data)
        .map_err(|e| format!("Failed to write token file: {}", e))?;

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn open_editor_with_protocol(
    app: tauri::AppHandle,
    protocol_url: String,
) -> Result<(), String> {
    println!("Opening editor with protocol URL: {}", protocol_url);

    use tauri_plugin_opener::OpenerExt;
    app.opener().open_url(protocol_url, None::<&str>)
        .map_err(|e| format!("Failed to open editor with protocol: {}", e))
}

// Outlook 邮箱管理命令
#[tauri::command]
async fn outlook_save_credentials(
    email: String,
    refresh_token: String,
    client_id: String,
    state: State<'_, AppState>,
    _app: tauri::AppHandle,
) -> Result<(), String> {
    let credentials = OutlookCredentials {
        email,
        refresh_token,
        client_id,
        status: Some("unknown".to_string()),
        last_checked: None,
    };

    // 仅内存保存：避免跨 await 持有锁，改为同步保存
    let result = {
        let mut manager = state.outlook_manager.lock().unwrap();
        manager.save_credentials(credentials)
    };
    result?;
    Ok(())
}

#[tauri::command]
async fn outlook_get_all_accounts(
    state: State<'_, AppState>,
    _app: tauri::AppHandle,
) -> Result<Vec<String>, String> {
    let manager = state.outlook_manager.lock().unwrap();
    manager.get_all_accounts()
}

#[tauri::command]
async fn outlook_delete_account(
    email: String,
    state: State<'_, AppState>,
    _app: tauri::AppHandle,
) -> Result<bool, String> {
    let result = {
        let mut manager = state.outlook_manager.lock().unwrap();
        manager.delete_account(&email)
    };
    result
}

#[tauri::command]
async fn outlook_check_account_status(
    email: String,
    state: State<'_, AppState>,
    _app: tauri::AppHandle,
) -> Result<OutlookAccountStatus, String> {
    let credentials = {
        let manager = state.outlook_manager.lock().unwrap();
        manager.get_credentials(&email)?
    };
    let check_manager = OutlookManager::new();
    check_manager.check_account_status_with_credentials(&credentials).await
}

#[tauri::command]
async fn outlook_get_emails(
    email: String,
    folder: String,
    page: i32,
    page_size: i32,
    state: State<'_, AppState>,
    _app: tauri::AppHandle,
) -> Result<EmailListResponse, String> {
    let credentials = {
        let manager = state.outlook_manager.lock().unwrap();
        manager.get_credentials(&email)?
    };
    let fetch_manager = OutlookManager::new();
    fetch_manager.get_emails_with_credentials(&credentials, &folder, page, page_size).await
}

#[tauri::command]
async fn outlook_get_email_details(
    email: String,
    message_id: String,
    state: State<'_, AppState>,
    _app: tauri::AppHandle,
) -> Result<EmailDetailsResponse, String> {
    let credentials = {
        let manager = state.outlook_manager.lock().unwrap();
        manager.get_credentials(&email)?
    };
    let details_manager = OutlookManager::new();
    details_manager.get_email_details_with_credentials(&credentials, &message_id).await
}

// 移除了获取账户状态功能 - 不再本地保存状态

// 移除了账户状态更新功能 - 不再本地保存状态







#[tauri::command]
async fn delete_token(
    token_id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let storage_manager = {
        let guard = state.storage_manager.lock().unwrap();
        guard.clone().ok_or("Storage manager not initialized")?
    };

    storage_manager.delete_token(&token_id).await
        .map_err(|e| format!("Delete failed: {}", e))
}

// 配置文件管理函数
fn get_config_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    Ok(app_data_dir.join("config.json"))
}

fn load_custom_data_dir(app: &tauri::AppHandle) -> Option<PathBuf> {
    let config = load_unified_config(app);
    config.custom_data_dir.map(PathBuf::from)
}

fn save_custom_data_dir(app: &tauri::AppHandle, path: &str) -> Result<(), String> {
    let mut config = load_unified_config(app);
    config.custom_data_dir = Some(path.to_string());
    config.last_updated = chrono::Utc::now();
    
    save_unified_config(app, &config)?;
    Ok(())
}

fn get_effective_data_dir(app: &tauri::AppHandle, state: &State<'_, AppState>) -> Result<PathBuf, String> {
    // 首先检查内存中的自定义目录
    {
        let custom_dir_guard = state.custom_data_dir.lock().unwrap();
        if let Some(custom_dir) = custom_dir_guard.as_ref() {
            return Ok(custom_dir.clone());
        }
    }
    
    // 如果内存中没有，尝试从配置文件加载
    if let Some(custom_dir) = load_custom_data_dir(app) {
        // 验证目录是否存在且可访问
        if custom_dir.exists() {
            // 保存到内存中以便下次使用
            *state.custom_data_dir.lock().unwrap() = Some(custom_dir.clone());
            return Ok(custom_dir);
        }
    }
    
    // 简单安全：直接使用AppData目录
    let default_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    // 确保目录存在
    fs::create_dir_all(&default_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    
    println!("使用AppData目录: {}", default_dir.display());
    Ok(default_dir)
}

// 设置页面相关命令
#[tauri::command]
async fn select_data_directory(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    use std::process::Command;

    #[cfg(target_os = "windows")]
    {
        // 使用改进的 PowerShell 命令，处理中文路径
        let script = r#"
        chcp 65001 > $null
        Add-Type -AssemblyName System.Windows.Forms
        $folderBrowser = New-Object System.Windows.Forms.FolderBrowserDialog
        $folderBrowser.Description = "选择数据存储目录"
        $folderBrowser.ShowNewFolderButton = $true
        $documentsPath = [Environment]::GetFolderPath([Environment+SpecialFolder]::MyDocuments)
        $folderBrowser.SelectedPath = $documentsPath
        $result = $folderBrowser.ShowDialog()
        if ($result -eq [System.Windows.Forms.DialogResult]::OK) {
            $utf8 = [System.Text.Encoding]::UTF8
            $bytes = $utf8.GetBytes($folderBrowser.SelectedPath)
            [System.Console]::WriteLine([System.Text.Encoding]::UTF8.GetString($bytes))
        }
        "#;

        let output = Command::new("powershell")
            .args(&["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", script])
            .output();

        match output {
            Ok(output) => {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                
                if path_str.is_empty() {
                    return Err("用户取消了目录选择".to_string());
                }

                // 验证路径是否包含中文字符
                if path_str.chars().any(|c| {
                    // 检查是否包含中文字符（Unicode范围）
                    ('\u{4e00}'..='\u{9fff}').contains(&c) || // 中日韩统一表意文字
                    ('\u{3400}'..='\u{4dbf}').contains(&c) || // 中日韩扩展A
                    ('\u{20000}'..='\u{2a6df}').contains(&c) || // 中日韩扩展B
                    ('\u{f900}'..='\u{faff}').contains(&c) || // 中日韩兼容表意文字
                    ('\u{2f800}'..='\u{2fa1f}').contains(&c) // 中日韩兼容表意文字补充
                }) {
                    return Err("为避免编码问题，请选择不包含中文字符的文件夹路径。建议使用英文路径，如：D:\\TokenData".to_string());
                }

                // 验证路径是否存在
                let path_buf = PathBuf::from(&path_str);
                if !path_buf.exists() {
                    return Err(format!("选择的路径不存在: {}", path_str));
                }

                if !path_buf.is_dir() {
                    return Err("选择的路径不是一个目录".to_string());
                }

                // 保存选择的目录到配置文件
                save_custom_data_dir(&app, &path_str)?;
                
                // 更新内存中的配置
                *state.custom_data_dir.lock().unwrap() = Some(path_buf);
                
                Ok(path_str)
            }
            Err(e) => {
                Err(format!("打开文件夹选择对话框失败: {}", e))
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("osascript")
            .args(&[
                "-e",
                r#"choose folder with prompt "选择数据存储目录""#
            ])
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let path = stdout.trim();
                if path.is_empty() {
                    Err("用户取消了目录选择".to_string())
                } else {
                    let path = path.replace("alias ", "").replace(":", "/");
                    let path = if path.starts_with("Macintosh HD") {
                        path.replace("Macintosh HD", "")
                    } else {
                        path
                    };
                    
                    // 验证路径是否包含中文字符
                    if path.chars().any(|c| {
                        ('\u{4e00}'..='\u{9fff}').contains(&c) || 
                        ('\u{3400}'..='\u{4dbf}').contains(&c) || 
                        ('\u{20000}'..='\u{2a6df}').contains(&c) || 
                        ('\u{f900}'..='\u{faff}').contains(&c) || 
                        ('\u{2f800}'..='\u{2fa1f}').contains(&c)
                    }) {
                        return Err("为避免编码问题，请选择不包含中文字符的文件夹路径。建议使用英文路径。".to_string());
                    }
                    
                    save_custom_data_dir(&app, &path)?;
                    *state.custom_data_dir.lock().unwrap() = Some(PathBuf::from(&path));
                    
                    Ok(path)
                }
            }
            Err(e) => {
                Err(format!("打开文件夹选择对话框失败: {}", e))
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("zenity")
            .args(&[
                "--file-selection",
                "--directory",
                "--title=选择数据存储目录"
            ])
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let path = stdout.trim();
                if path.is_empty() {
                    Err("用户取消了目录选择".to_string())
                } else {
                    // 验证路径是否包含中文字符
                    if path.chars().any(|c| {
                        ('\u{4e00}'..='\u{9fff}').contains(&c) || 
                        ('\u{3400}'..='\u{4dbf}').contains(&c) || 
                        ('\u{20000}'..='\u{2a6df}').contains(&c) || 
                        ('\u{f900}'..='\u{faff}').contains(&c) || 
                        ('\u{2f800}'..='\u{2fa1f}').contains(&c)
                    }) {
                        return Err("为避免编码问题，请选择不包含中文字符的文件夹路径。建议使用英文路径。".to_string());
                    }
                    
                    save_custom_data_dir(&app, path)?;
                    *state.custom_data_dir.lock().unwrap() = Some(PathBuf::from(path));
                    Ok(path.to_string())
                }
            }
            Err(_) => {
                let home_dir = env::var("HOME")
                    .map_err(|_| "无法获取用户主目录".to_string())?;
                let documents_dir = format!("{}/Documents", home_dir);
                
                save_custom_data_dir(&app, &documents_dir)?;
                *state.custom_data_dir.lock().unwrap() = Some(PathBuf::from(&documents_dir));
                
                Ok(documents_dir)
            }
        }
    }
}

#[tauri::command]
async fn get_current_data_path(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    // 获取有效的数据目录（优先使用自定义目录）
    let data_dir = get_effective_data_dir(&app, &state)?;
    let tokens_path = data_dir.join("tokens.json");
    Ok(tokens_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn get_data_directory(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {
    // 获取有效的数据目录（优先使用自定义目录）
    let data_dir = get_effective_data_dir(&app, &state)?;
    Ok(data_dir.to_string_lossy().to_string())
}

#[tauri::command]
async fn reset_data_directory(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    // 删除配置文件中的自定义目录设置
    let config_path = get_config_path(&app)?;
    if config_path.exists() {
        // 读取现有配置
        let content = fs::read_to_string(&config_path).unwrap_or_default();
        if let Ok(mut config) = serde_json::from_str::<serde_json::Value>(&content) {
            // 移除自定义数据目录设置
            if let Some(obj) = config.as_object_mut() {
                obj.remove("custom_data_dir");
                
                // 保存更新后的配置
                fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap())
                    .map_err(|e| format!("Failed to update config: {}", e))?;
            }
        }
    }
    
    // 清除内存中的自定义目录设置
    *state.custom_data_dir.lock().unwrap() = None;
    
    // 获取默认应用数据目录
    let default_app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get default app data directory: {}", e))?;

    // 确保默认目录存在
    fs::create_dir_all(&default_app_data_dir)
        .map_err(|e| format!("Failed to create default app local data directory: {}", e))?;

    Ok(())
}

#[tauri::command]
async fn open_data_directory(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    // 获取有效的数据目录（优先使用自定义目录）
    let data_dir = get_effective_data_dir(&app, &state)?;

    // 创建目录如果不存在
    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create data directory: {}", e))?;

    // 使用系统默认文件管理器打开目录
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&data_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&data_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&data_dir)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    Ok(())
}







// 辅助函数：初始化存储管理器
async fn initialize_storage_manager(
    app: &tauri::AppHandle,
    state: &State<'_, AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 创建本地存储
    let local_storage = Arc::new(LocalFileStorage::new(app)?);

    // 更新应用状态
    *state.storage_manager.lock().unwrap() = Some(local_storage);

    Ok(())
}

// WebDAV云同步相关命令
#[tauri::command]
async fn configure_webdav(
    server_url: String,
    username: String,
    password: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let mut config = WebDAVConfig::new(server_url, username, password);
    config.remote_path = "ZAugment/tokens.json".to_string();
    
    // 测试连接
    let client = webdav::WebDAVClient::new(config.clone())
        .map_err(|e| format!("创建WebDAV客户端失败: {}", e))?;
    
    client.test_connection().await
        .map_err(|e| format!("WebDAV连接测试失败: {}", e))?;
    
    // 创建安全配置
    let secure_config = SecureWebDAVConfig::from_config(&config, &state.password_manager)
        .map_err(|e| format!("创建安全配置失败: {}", e))?;
    
    // 保存安全配置到内存
    *state.webdav_config.lock().unwrap() = Some(secure_config.clone());
    
    // 保存配置到统一配置文件（使用原始WebDAVConfig包含明文密码）
    let mut unified_config = load_unified_config_with_state(&app, &state);
    unified_config.webdav_config = Some(config.clone());
    unified_config.last_updated = chrono::Utc::now();
    save_unified_config_with_state(&app, &unified_config, &state)?;
    
    Ok("WebDAV配置成功，密码已安全存储".to_string())
}

#[tauri::command]
async fn test_webdav_connection(state: State<'_, AppState>) -> Result<bool, String> {
    let config = {
        let config_guard = state.webdav_config.lock().unwrap();
        let secure_config = config_guard.as_ref()
            .ok_or("WebDAV未配置")?;
        secure_config.to_config(&state.password_manager)
            .map_err(|e| format!("获取配置失败: {}", e))?
    };
    
    let client = webdav::WebDAVClient::new(config)
        .map_err(|e| format!("创建WebDAV客户端失败: {}", e))?;
    
    client.test_connection().await
        .map_err(|e| format!("连接测试失败: {}", e))
}

#[tauri::command]
async fn sync_to_cloud(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<String, String> {
    let config = {
        let config_guard = state.webdav_config.lock().unwrap();
        let secure_config = config_guard.as_ref()
            .ok_or("WebDAV未配置")?;
        secure_config.to_config(&state.password_manager)
            .map_err(|e| format!("获取配置失败: {}", e))?
    };
    
    // 收集所有用户数据
    let user_data = UserDataPackage::collect_from_local(&app, &state).await?;
    
    // 创建临时文件来存储用户数据包
    let data_dir = get_effective_data_dir(&app, &state)?;
    let temp_path = data_dir.join("user_data_temp.json");
    
    // 将用户数据包写入临时文件
    let data_bytes = user_data.to_bytes()?;
    fs::write(&temp_path, &data_bytes)
        .map_err(|e| format!("写入临时文件失败: {}", e))?;
    
    // 创建同步实例（使用新的文件名）
    let mut sync_config = config.clone();
    // 将远程路径改为用户数据包文件
    // 确保远程路径正确：将 tokens.json 替换为 user_data.json
    if sync_config.remote_path.contains("tokens.json") {
        sync_config.remote_path = sync_config.remote_path.replace("tokens.json", "user_data.json");
    } else {
        // 如果路径中没有 tokens.json，使用默认路径
        sync_config.remote_path = "/ZAugment/user_data.json".to_string();
    }
    
    let mut sync = CloudSync::new(sync_config, temp_path.clone())
        .map_err(|e| format!("创建同步实例失败: {}", e))?;
    
    // 执行同步
    let result = sync.sync().await
        .map_err(|e| format!("同步失败: {}", e))?;
    
    // 清理临时文件
    let _ = fs::remove_file(&temp_path);
    
    // 更新状态中的同步实例
    {
        let mut sync_guard = state.cloud_sync.lock().unwrap();
        *sync_guard = Some(sync);
    }
    
    Ok(format!("用户数据同步完成: {} (传输 {} 字节)", result.message, result.bytes_transferred))
}

#[tauri::command]
async fn force_upload_to_cloud(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<String, String> {
    let config = {
        let config_guard = state.webdav_config.lock().unwrap();
        let secure_config = config_guard.as_ref()
            .ok_or("WebDAV未配置")?;
        secure_config.to_config(&state.password_manager)
            .map_err(|e| format!("获取配置失败: {}", e))?
    };
    
    // 收集所有用户数据
    let user_data = UserDataPackage::collect_from_local(&app, &state).await?;
    
    // 创建临时文件来存储用户数据包
    let data_dir = get_effective_data_dir(&app, &state)?;
    let temp_path = data_dir.join("user_data_temp.json");
    
    // 将用户数据包写入临时文件
    let data_bytes = user_data.to_bytes()?;
    fs::write(&temp_path, &data_bytes)
        .map_err(|e| format!("写入临时文件失败: {}", e))?;
    
    // 创建同步实例（使用新的文件名）
    let mut sync_config = config.clone();
    // 确保远程路径正确：将 tokens.json 替换为 user_data.json
    if sync_config.remote_path.contains("tokens.json") {
        sync_config.remote_path = sync_config.remote_path.replace("tokens.json", "user_data.json");
    } else {
        // 如果路径中没有 tokens.json，使用默认路径
        sync_config.remote_path = "/ZAugment/user_data.json".to_string();
    }
    
    // 调试信息：显示上传路径（在移动之前）
    println!("上传路径: {}", sync_config.remote_path);
    println!("上传URL: {}", sync_config.get_remote_file_url().unwrap_or_else(|_| "URL构建失败".to_string()));
    
    let mut sync = CloudSync::new(sync_config, temp_path.clone())
        .map_err(|e| format!("创建同步实例失败: {}", e))?;
    
    let result = sync.force_upload().await
        .map_err(|e| format!("强制上传失败: {}", e))?;
    
    // 清理临时文件
    let _ = fs::remove_file(&temp_path);
    
    {
        let mut sync_guard = state.cloud_sync.lock().unwrap();
        *sync_guard = Some(sync);
    }
    
    Ok(format!("强制上传用户数据完成: {} (传输 {} 字节)", result.message, result.bytes_transferred))
}

#[tauri::command]
async fn force_download_from_cloud(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<String, String> {
    let config = {
        let config_guard = state.webdav_config.lock().unwrap();
        let secure_config = config_guard.as_ref()
            .ok_or("WebDAV未配置")?;
        secure_config.to_config(&state.password_manager)
            .map_err(|e| format!("获取配置失败: {}", e))?
    };
    
    // 创建临时文件来存储下载的用户数据包
    let data_dir = get_effective_data_dir(&app, &state)?;
    let temp_path = data_dir.join("user_data_temp.json");
    
    // 创建同步实例（使用新的文件名）
    let mut sync_config = config.clone();
    // 确保远程路径正确：将 tokens.json 替换为 user_data.json
    if sync_config.remote_path.contains("tokens.json") {
        sync_config.remote_path = sync_config.remote_path.replace("tokens.json", "user_data.json");
    } else {
        // 如果路径中没有 tokens.json，使用默认路径
        sync_config.remote_path = "/ZAugment/user_data.json".to_string();
    }
    
    // 调试信息：显示使用的路径（在移动之前）
    println!("下载路径: {}", sync_config.remote_path);
    println!("完整URL: {}", sync_config.get_remote_file_url().unwrap_or_else(|_| "URL构建失败".to_string()));
    
    let mut sync = CloudSync::new(sync_config, temp_path.clone())
        .map_err(|e| format!("创建同步实例失败: {}", e))?;
    
    // 强制下载到临时文件
    let result = sync.force_download().await
        .map_err(|e| format!("强制下载失败: {}", e))?;
    
    // 检查下载结果
    if !result.success {
        return Err(format!("下载未成功: {}", result.message));
    }
    
    // 验证临时文件是否存在
    if !temp_path.exists() {
        return Err(format!("临时文件不存在: {:?}", temp_path));
    }
    
    // 读取下载的数据并解析为用户数据包
    let data_bytes = fs::read(&temp_path)
        .map_err(|e| format!("读取下载文件失败 (路径: {:?}): {}", temp_path, e))?;
    
    let user_data = UserDataPackage::from_bytes(&data_bytes)?;
    
    // 恢复用户数据到本地
    user_data.restore_to_local(&app, &state).await?;
    
    // 清理临时文件
    let _ = fs::remove_file(&temp_path);
    
    {
        let mut sync_guard = state.cloud_sync.lock().unwrap();
        *sync_guard = Some(sync);
    }
    
    Ok(format!("强制下载用户数据完成: {} (传输 {} 字节)", result.message, result.bytes_transferred))
}

#[tauri::command]
async fn get_webdav_config(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<Option<WebDAVConfig>, String> {
    // 从统一配置文件获取WebDAV配置（包含明文密码）
    let unified_config = load_unified_config_with_state(&app, &state);
    Ok(unified_config.webdav_config)
}

#[tauri::command]
async fn check_sync_conflicts(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<Option<webdav::ConflictInfo>, String> {
    let config = {
        let config_guard = state.webdav_config.lock().unwrap();
        let secure_config = config_guard.as_ref()
            .ok_or("WebDAV未配置")?;
        secure_config.to_config(&state.password_manager)
            .map_err(|e| format!("获取配置失败: {}", e))?
    };
    
    let data_dir = get_effective_data_dir(&app, &state)?;
    let tokens_file = data_dir.join("tokens.json");
    
    let cloud_sync = CloudSync::new(config, tokens_file)
        .map_err(|e| format!("创建CloudSync失败: {}", e))?;
    
    cloud_sync.get_conflict_info().await
        .map_err(|e| format!("检查冲突失败: {}", e))
}

#[tauri::command]
async fn resolve_sync_conflict(
    resolution: String,
    state: State<'_, AppState>, 
    app: tauri::AppHandle
) -> Result<String, String> {
    let conflict_resolution = match resolution.as_str() {
        "keep_local" => webdav::ConflictResolution::KeepLocal,
        "keep_remote" => webdav::ConflictResolution::KeepRemote,
        "keep_both" => webdav::ConflictResolution::KeepBoth,
        "merge" => webdav::ConflictResolution::Merge,
        _ => return Err("无效的冲突解决方案".to_string()),
    };
    
    let config = {
        let config_guard = state.webdav_config.lock().unwrap();
        let secure_config = config_guard.as_ref()
            .ok_or("WebDAV未配置")?;
        secure_config.to_config(&state.password_manager)
            .map_err(|e| format!("获取配置失败: {}", e))?
    };
    
    let data_dir = get_effective_data_dir(&app, &state)?;
    let tokens_file = data_dir.join("tokens.json");
    
    let mut cloud_sync = CloudSync::new(config, tokens_file)
        .map_err(|e| format!("创建CloudSync失败: {}", e))?;
    
    let result = cloud_sync.resolve_conflict(conflict_resolution).await
        .map_err(|e| format!("解决冲突失败: {}", e))?;
    
    Ok(format!("冲突解决完成: {} (传输 {} 字节)", result.message, result.bytes_transferred))
}

// 保存安全WebDAV配置到文件（不包含密码）
fn save_secure_webdav_config(app: &tauri::AppHandle, config: &SecureWebDAVConfig) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    
    let config_path = app_data_dir.join("webdav_config.json");
    let config_json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    
    fs::write(&config_path, config_json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;
    
    Ok(())
}

// 从文件加载安全WebDAV配置
fn load_secure_webdav_config(app: &tauri::AppHandle) -> Option<SecureWebDAVConfig> {
    let app_data_dir = app.path().app_data_dir().ok()?;
    let config_path = app_data_dir.join("webdav_config.json");
    
    if !config_path.exists() {
        return None;
    }
    
    let content = fs::read_to_string(&config_path).ok()?;
    serde_json::from_str(&content).ok()
}

// 兼容性函数：保存WebDAV配置到文件
fn save_webdav_config(app: &tauri::AppHandle, config: &WebDAVConfig) -> Result<(), String> {
    let password_manager = PasswordManager::new();
    let secure_config = SecureWebDAVConfig::from_config(config, &password_manager)
        .map_err(|e| format!("Failed to create secure config: {}", e))?;
    save_secure_webdav_config(app, &secure_config)
}

// 兼容性函数：从文件加载WebDAV配置
fn load_webdav_config(app: &tauri::AppHandle) -> Option<WebDAVConfig> {
    let secure_config = load_secure_webdav_config(app)?;
    let password_manager = PasswordManager::new();
    secure_config.to_config(&password_manager).ok()
}

// 统一的应用配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAppConfig {
    pub version: String,
    pub last_updated: chrono::DateTime<chrono::Utc>,

    // 应用基础设置
    pub app_settings: AppSettings,

    // 数据目录设置
    pub custom_data_dir: Option<String>,

    // WebDAV配置（直接存储明文密码）
    pub webdav_config: Option<WebDAVConfig>,

    // UI设置
    pub ui_settings: UiSettings,

    // 账号状态阈值配置
    pub status_thresholds: Option<StatusThresholds>,
}

// 应用基础设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub current_view: String,
    pub auto_sync_enabled: bool,
    pub last_sync_time: Option<chrono::DateTime<chrono::Utc>>,
}

// UI设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub theme: Option<String>,
    pub window_size: Option<(u32, u32)>,
    pub window_position: Option<(i32, i32)>,
    pub language: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            current_view: "token-generator".to_string(),
            auto_sync_enabled: false,
            last_sync_time: None,
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: None,
            window_size: None,
            window_position: None,
            language: Some("zh-CN".to_string()),
        }
    }
}

impl Default for UnifiedAppConfig {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            last_updated: chrono::Utc::now(),
            app_settings: AppSettings::default(),
            custom_data_dir: None,
            webdav_config: None,
            ui_settings: UiSettings::default(),
            status_thresholds: Some(StatusThresholds::default()),
        }
    }
}

// 保存统一配置到文件（注意：需要AppState来获取有效数据目录）
fn save_unified_config_with_state(app: &tauri::AppHandle, config: &UnifiedAppConfig, state: &State<'_, AppState>) -> Result<(), String> {
    let data_dir = get_effective_data_dir(app, state)?;

    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create data directory: {}", e))?;

    let config_path = data_dir.join("config.json");
    let config_json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize unified config: {}", e))?;

    // 使用同一目录下的唯一临时文件名，确保在同一文件系统上
    let temp_path = data_dir.join(format!("config.{}.tmp", uuid::Uuid::new_v4()));

    // 写入临时文件
    fs::write(&temp_path, &config_json)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;

    // 尝试原子性重命名，如果失败则使用复制+删除的方式
    match fs::rename(&temp_path, &config_path) {
        Ok(_) => Ok(()),
        Err(rename_err) => {
            // 如果 rename 失败（可能是跨文件系统），尝试复制+删除
            match fs::copy(&temp_path, &config_path) {
                Ok(_) => {
                    let _ = fs::remove_file(&temp_path);
                    Ok(())
                }
                Err(copy_err) => {
                    let _ = fs::remove_file(&temp_path);
                    Err(format!("Failed to save config (rename: {}, copy: {})", rename_err, copy_err))
                }
            }
        }
    }
}

// 保存统一配置到默认应用目录（用于初始化或没有state时）
fn save_unified_config(app: &tauri::AppHandle, config: &UnifiedAppConfig) -> Result<(), String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    let config_path = data_dir.join("config.json");
    let config_json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize unified config: {}", e))?;

    // 使用同一目录下的唯一临时文件名，确保在同一文件系统上
    let temp_path = data_dir.join(format!("config.{}.tmp", uuid::Uuid::new_v4()));

    // 写入临时文件
    fs::write(&temp_path, &config_json)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;

    // 尝试原子性重命名，如果失败则使用复制+删除的方式
    match fs::rename(&temp_path, &config_path) {
        Ok(_) => Ok(()),
        Err(rename_err) => {
            // 如果 rename 失败（可能是跨文件系统），尝试复制+删除
            match fs::copy(&temp_path, &config_path) {
                Ok(_) => {
                    let _ = fs::remove_file(&temp_path);
                    Ok(())
                }
                Err(copy_err) => {
                    let _ = fs::remove_file(&temp_path);
                    Err(format!("Failed to save config (rename: {}, copy: {})", rename_err, copy_err))
                }
            }
        }
    }
}

// 从文件加载统一配置（支持迁移旧配置）
fn load_unified_config(app: &tauri::AppHandle) -> UnifiedAppConfig {
    // 先尝试从应用默认目录加载配置
    let app_data_dir = match app.path().app_data_dir() {
        Ok(dir) => {
            // 确保目录存在
            if let Err(e) = fs::create_dir_all(&dir) {
                println!("警告：无法创建应用数据目录 {}: {}", dir.display(), e);
                return UnifiedAppConfig::default();
            }
            dir
        },
        Err(e) => {
            println!("警告：无法获取应用数据目录: {}", e);
            return UnifiedAppConfig::default();
        },
    };
    
    let config_path = app_data_dir.join("config.json");
    
    // 如果统一配置文件存在，直接加载
    if config_path.exists() {
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                if let Ok(config) = serde_json::from_str::<UnifiedAppConfig>(&content) {
                    return config;
                }
                // 如果解析失败，尝试从旧格式迁移
                if let Ok(old_config) = serde_json::from_str::<serde_json::Value>(&content) {
                    return migrate_from_old_config(app, old_config);
                }
                println!("警告：配置文件格式错误，使用默认配置");
            },
            Err(e) => {
                println!("警告：无法读取配置文件 {}: {}", config_path.display(), e);
            }
        }
    }
    
    // 如果配置文件不存在，尝试从旧文件迁移
    migrate_from_old_files(app)
}

// 从有效数据目录加载统一配置
fn load_unified_config_with_state(app: &tauri::AppHandle, state: &State<'_, AppState>) -> UnifiedAppConfig {
    let data_dir = match get_effective_data_dir(app, state) {
        Ok(dir) => {
            // 确保目录存在
            if let Err(e) = fs::create_dir_all(&dir) {
                println!("警告：无法创建有效数据目录 {}: {}", dir.display(), e);
                return load_unified_config(app); // 回退到默认加载方式
            }
            dir
        },
        Err(_) => return load_unified_config(app), // 回退到默认加载方式
    };
    
    let config_path = data_dir.join("config.json");
    
    // 如果统一配置文件存在，直接加载
    if config_path.exists() {
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                if let Ok(config) = serde_json::from_str::<UnifiedAppConfig>(&content) {
                    return config;
                }
                println!("警告：自定义目录配置文件格式错误");
            },
            Err(e) => {
                println!("警告：无法读取自定义目录配置文件 {}: {}", config_path.display(), e);
            }
        }
    }
    
    // 避免递归调用，直接返回默认配置
    println!("自定义目录中没有配置文件，使用默认配置");
    UnifiedAppConfig::default()
}

// 从旧文件迁移配置
fn migrate_from_old_files(app: &tauri::AppHandle) -> UnifiedAppConfig {
    let mut config = UnifiedAppConfig::default();
    let data_dir = match app.path().app_data_dir() {
        Ok(dir) => dir,
        Err(_) => return config,
    };
    
    // 注释掉递归调用，避免栈溢出
    // if let Some(custom_dir) = load_custom_data_dir(app) {
    //     config.custom_data_dir = Some(custom_dir.to_string_lossy().to_string());
    // }
    
    // 迁移WebDAV配置（从SecureWebDAVConfig转换为WebDAV Config）
    if let Some(secure_webdav_config) = load_secure_webdav_config(app) {
        // 尝试从keyring获取密码
        let password_manager = PasswordManager::new();
        if let Ok(webdav_config) = secure_webdav_config.to_config(&password_manager) {
            config.webdav_config = Some(webdav_config);
        }
    }
    
    // 迁移旧的应用设置（如果存在）
    let old_app_settings_path = data_dir.join("app_settings.json");
    if old_app_settings_path.exists() {
        if let Ok(content) = fs::read_to_string(&old_app_settings_path) {
            if let Ok(old_settings) = serde_json::from_str::<AppSettings>(&content) {
                config.app_settings = old_settings;
            }
        }
    }
    
    // 保存迁移后的配置（如果有自定义数据目录，也要保存到那里）
    let _ = save_unified_config(app, &config);
    
    // 如果存在自定义数据目录，也要保存到那里
    if let Some(ref custom_dir_str) = config.custom_data_dir {
        let custom_dir = PathBuf::from(custom_dir_str);
        if custom_dir.exists() {
            let custom_config_path = custom_dir.join("config.json");
            if let Ok(config_json) = serde_json::to_string_pretty(&config) {
                let _ = fs::write(custom_config_path, config_json);
                println!("已将迁移的配置保存到自定义目录: {}", custom_dir_str);
            }
        }
    }
    
    // 清理旧文件
    let _ = fs::remove_file(old_app_settings_path);
    let _ = fs::remove_file(data_dir.join("webdav_config.json"));
    
    config
}

// 从旧配置格式迁移
fn migrate_from_old_config(app: &tauri::AppHandle, old_config: serde_json::Value) -> UnifiedAppConfig {
    let mut config = UnifiedAppConfig::default();
    
    // 迁移自定义数据目录
    if let Some(custom_dir) = old_config.get("custom_data_dir").and_then(|v| v.as_str()) {
        config.custom_data_dir = Some(custom_dir.to_string());
    }
    
    // 保存新的配置格式
    let _ = save_unified_config(app, &config);
    
    config
}

// Tauri命令：保存应用设置
#[tauri::command]
async fn save_app_settings_cmd(
    current_view: String,
    auto_sync_enabled: bool,
    state: State<'_, AppState>,
    app: tauri::AppHandle
) -> Result<(), String> {
    let mut config = load_unified_config_with_state(&app, &state);
    config.app_settings.current_view = current_view;
    config.app_settings.auto_sync_enabled = auto_sync_enabled;
    config.app_settings.last_sync_time = Some(chrono::Utc::now());
    config.last_updated = chrono::Utc::now();
    
    save_unified_config_with_state(&app, &config, &state)?;
    Ok(())
}

// Tauri命令：获取应用设置
#[tauri::command]
async fn get_app_settings_cmd(app: tauri::AppHandle) -> Result<AppSettings, String> {
    let config = load_unified_config(&app);
    Ok(config.app_settings)
}

// Tauri命令：获取完整统一配置
#[tauri::command]
async fn get_unified_config_cmd(app: tauri::AppHandle) -> Result<UnifiedAppConfig, String> {
    Ok(load_unified_config(&app))
}

// Tauri命令：保存UI设置
#[tauri::command]
async fn save_ui_settings_cmd(
    theme: Option<String>,
    window_size: Option<(u32, u32)>,
    language: Option<String>,
    state: State<'_, AppState>,
    app: tauri::AppHandle
) -> Result<(), String> {
    let mut config = load_unified_config_with_state(&app, &state);
    config.ui_settings.theme = theme;
    config.ui_settings.window_size = window_size;
    config.ui_settings.language = language;
    config.last_updated = chrono::Utc::now();

    save_unified_config_with_state(&app, &config, &state)?;
    Ok(())
}

// ================================
// 阈值配置管理命令
// ================================

// Tauri命令：保存阈值配置
#[tauri::command]
async fn save_status_thresholds(
    thresholds: StatusThresholds,
    state: State<'_, AppState>,
    app: tauri::AppHandle
) -> Result<String, String> {
    // 加载现有配置
    let mut config = load_unified_config_with_state(&app, &state);

    // 更新阈值配置
    config.status_thresholds = Some(thresholds);
    config.last_updated = chrono::Utc::now();

    // 保存到 config.json
    save_unified_config_with_state(&app, &config, &state)?;

    Ok("阈值配置已保存".to_string())
}

// Tauri命令：加载阈值配置
#[tauri::command]
async fn load_status_thresholds(
    state: State<'_, AppState>,
    app: tauri::AppHandle
) -> Result<StatusThresholds, String> {
    // 从 config.json 加载配置
    let config = load_unified_config_with_state(&app, &state);

    // 返回阈值配置，如果不存在则返回默认值
    Ok(config.status_thresholds.unwrap_or_default())
}

// ================================
// 编辑器重置功能相关命令
// ================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorInfo {
    pub editor_type: String,
    pub name: String,
    pub process_names: Vec<String>,
    pub config_paths: Vec<String>,
}

/// 获取编辑器配置路径
fn get_editor_paths(editor_type: &str) -> Result<(Option<PathBuf>, Option<PathBuf>), String> {
    let home_dir = dirs::home_dir().ok_or("无法获取用户主目录")?;

    match editor_type {
        "vscode" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                let base_dir = PathBuf::from(appdata).join("Code").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
            #[cfg(target_os = "macos")]
            {
                let base_dir = home_dir.join("Library").join("Application Support").join("Code").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
            #[cfg(target_os = "linux")]
            {
                let base_dir = home_dir.join(".config").join("Code").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
        },
        "cursor" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                let base_dir = PathBuf::from(appdata).join("Cursor").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                let base_dir = home_dir.join(".cursor").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
        },
        "windsurf" => {
            // Windsurf 可能有多种路径，这里简化处理
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                let base_dir = PathBuf::from(appdata).join("Windsurf").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                let base_dir = home_dir.join(".config").join("Windsurf").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
        },
        // 其他 VSCode 系列编辑器
        "kiro" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                let base_dir = PathBuf::from(appdata).join("Kiro").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                let base_dir = home_dir.join(".config").join("Kiro").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
        },
        "trae" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                let base_dir = PathBuf::from(appdata).join("Trae").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                let base_dir = home_dir.join(".config").join("Trae").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
        },
        "qoder" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                let base_dir = PathBuf::from(appdata).join("Qoder").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                let base_dir = home_dir.join(".config").join("Qoder").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
        },
        "vscodium" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                let base_dir = PathBuf::from(appdata).join("VSCodium").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                let base_dir = home_dir.join(".config").join("VSCodium").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
        },
        "codebuddy" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                let base_dir = PathBuf::from(appdata).join("CodeBuddy").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                let base_dir = home_dir.join(".config").join("CodeBuddy").join("User").join("globalStorage");
                Ok((
                    Some(base_dir.join("state.vscdb")),
                    Some(base_dir.join("storage.json"))
                ))
            }
        },

        // JetBrains 系列 - 所有编辑器都不使用 SQLite 数据库
        "idea" | "pycharm" | "goland" | "rustrover" | "webstorm" | "phpstorm" |
        "clion" | "datagrip" | "rider" | "rubymine" | "aqua" | "androidstudio" => {
            Ok((None, None))
        },

        _ => Err(format!("不支持的编辑器类型: {}", editor_type))
    }
}

/// 获取编辑器进程名称列表
fn get_editor_process_names(editor_type: &str) -> Vec<String> {
    match editor_type {
        // VSCode 系列
        "vscode" => vec![
            "Code.exe".to_string(),
            "Code - Insiders.exe".to_string(),
            "Code - OSS.exe".to_string(),
            "code".to_string(),
        ],
        "cursor" => vec![
            "Cursor.exe".to_string(),
            "cursor.exe".to_string(),
            "cursor".to_string(),
        ],
        "kiro" => vec![
            "Kiro.exe".to_string(),
            "kiro.exe".to_string(),
            "kiro".to_string(),
        ],
        "trae" => vec![
            "Trae.exe".to_string(),
            "trae.exe".to_string(),
            "trae".to_string(),
        ],
        "windsurf" => vec![
            "Windsurf.exe".to_string(),
            "windsurf.exe".to_string(),
            "windsurf".to_string(),
        ],
        "qoder" => vec![
            "Qoder.exe".to_string(),
            "qoder.exe".to_string(),
            "qoder".to_string(),
        ],
        "vscodium" => vec![
            "VSCodium.exe".to_string(),
            "vscodium.exe".to_string(),
            "vscodium".to_string(),
            "codium".to_string(),
        ],
        "codebuddy" => vec![
            "CodeBuddy.exe".to_string(),
            "codebuddy.exe".to_string(),
            "codebuddy".to_string(),
        ],

        // JetBrains 系列 - 单独处理每个编辑器
        "idea" => vec![
            "idea64.exe".to_string(), "idea.exe".to_string(), "idea".to_string(),
        ],
        "pycharm" => vec![
            "pycharm64.exe".to_string(), "pycharm.exe".to_string(), "pycharm".to_string(),
        ],
        "goland" => vec![
            "goland64.exe".to_string(), "goland.exe".to_string(), "goland".to_string(),
        ],
        "rustrover" => vec![
            "rustrover64.exe".to_string(), "rustrover.exe".to_string(), "rustrover".to_string(),
        ],
        "webstorm" => vec![
            "webstorm64.exe".to_string(), "webstorm.exe".to_string(), "webstorm".to_string(),
        ],
        "phpstorm" => vec![
            "phpstorm64.exe".to_string(), "phpstorm.exe".to_string(), "phpstorm".to_string(),
        ],
        "clion" => vec![
            "clion64.exe".to_string(), "clion.exe".to_string(), "clion".to_string(),
        ],
        "datagrip" => vec![
            "datagrip64.exe".to_string(), "datagrip.exe".to_string(), "datagrip".to_string(),
        ],
        "rider" => vec![
            "rider64.exe".to_string(), "rider.exe".to_string(), "rider".to_string(),
        ],
        "rubymine" => vec![
            "rubymine64.exe".to_string(), "rubymine.exe".to_string(), "rubymine".to_string(),
        ],
        "aqua" => vec![
            "aqua64.exe".to_string(), "aqua.exe".to_string(), "aqua".to_string(),
        ],
        "androidstudio" => vec![
            "studio64.exe".to_string(), "studio.exe".to_string(), "studio".to_string(),
        ],

        _ => vec![]
    }
}

#[tauri::command]
async fn check_editor_processes(editor_type: String) -> Result<Vec<String>, String> {
    let process_names = get_editor_process_names(&editor_type);
    let mut system = System::new_all();
    system.refresh_all();

    let mut running_processes = Vec::new();

    // 收集所有匹配的进程
    for process in system.processes().values() {
        let process_name = process.name();
        if process_names.iter().any(|name| process_name.contains(name)) {
            running_processes.push(format!("{} (PID: {})", process_name, process.pid()));
        }
    }

    Ok(running_processes)
}

#[tauri::command]
async fn close_editor_processes(editor_type: String) -> Result<String, String> {
    let process_names = get_editor_process_names(&editor_type);
    let mut system = System::new_all();
    system.refresh_all();

    let mut closed_count = 0;
    let mut failed_processes = Vec::new();
    let mut process_pids = Vec::new();

    // 首先收集所有匹配的进程
    for process in system.processes().values() {
        let process_name = process.name();
        if process_names.iter().any(|name| process_name.contains(name)) {
            process_pids.push((process.pid(), process_name.to_string()));
        }
    }

    // 获取编辑器显示名称
    let editor_display_name = match editor_type.as_str() {
        "vscode" => "VS Code",
        "cursor" => "Cursor",
        "kiro" => "Kiro",
        "trae" => "Trae",
        "windsurf" => "Windsurf",
        "qoder" => "Qoder",
        "vscodium" => "VSCodium",
        "codebuddy" => "CodeBuddy",
        "idea" => "IntelliJ IDEA",
        "pycharm" => "PyCharm",
        "goland" => "GoLand",
        "rustrover" => "RustRover",
        "webstorm" => "WebStorm",
        "phpstorm" => "PhpStorm",
        "clion" => "CLion",
        "datagrip" => "DataGrip",
        "rider" => "Rider",
        "rubymine" => "RubyMine",
        "aqua" => "Aqua",
        "androidstudio" => "Android Studio",
        _ => &editor_type,
    };

    if process_pids.is_empty() {
        return Ok(format!("关闭进程: 未发现运行中的 {} 进程", editor_display_name));
    }

    // 使用Windows API温和地关闭进程
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        for (pid, process_name) in &process_pids {
            println!("尝试关闭进程: {} (PID: {})", process_name, pid);

            // 第一步：使用温和的方式关闭进程
            let output = Command::new("taskkill")
                .args(&["/PID", &pid.to_string(), "/T"]) // /T 关闭进程树
                .output();

            let mut process_closed = false;

            match output {
                Ok(result) => {
                    if result.status.success() {
                        closed_count += 1;
                        process_closed = true;
                        println!("温和关闭成功: {} (PID: {})", process_name, pid);
                    } else {
                        println!("温和关闭失败，尝试强制关闭: {} (PID: {})", process_name, pid);
                    }
                }
                Err(e) => {
                    println!("温和关闭命令执行失败: {}, 尝试强制关闭", e);
                }
            }

            // 如果温和关闭失败，尝试强制关闭
            if !process_closed {
                let force_output = Command::new("taskkill")
                    .args(&["/PID", &pid.to_string(), "/F", "/T"])
                    .output();

                match force_output {
                    Ok(force_result) => {
                        if force_result.status.success() {
                            closed_count += 1;
                            process_closed = true;
                            println!("强制关闭成功: {} (PID: {})", process_name, pid);
                        } else {
                            let stderr = String::from_utf8_lossy(&force_result.stderr);
                            println!("强制关闭失败: {} (PID: {}), 错误: {}", process_name, pid, stderr);
                        }
                    }
                    Err(e) => {
                        println!("强制关闭命令执行失败: {} (PID: {}), 错误: {}", process_name, pid, e);
                    }
                }
            }

            // 如果进程仍然没有关闭，记录到失败列表
            if !process_closed {
                failed_processes.push(format!("{} (PID: {})", process_name, pid));
            }
        }
    }

    // 非Windows系统使用SIGTERM和SIGKILL的组合方式
    #[cfg(not(target_os = "windows"))]
    {
        use std::process::Command;

        for (pid, process_name) in &process_pids {
            println!("尝试关闭进程: {} (PID: {})", process_name, pid);
            let mut process_closed = false;

            // 第一步：使用SIGTERM温和关闭
            let term_output = Command::new("kill")
                .args(&["-TERM", &pid.to_string()])
                .output();

            match term_output {
                Ok(result) => {
                    if result.status.success() {
                        println!("发送SIGTERM信号成功: {} (PID: {})", process_name, pid);
                        // 等待进程响应SIGTERM
                        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

                        // 检查进程是否还存在
                        let check_output = Command::new("kill")
                            .args(&["-0", &pid.to_string()])
                            .output();

                        if let Ok(check_result) = check_output {
                            if !check_result.status.success() {
                                // 进程已经不存在了
                                closed_count += 1;
                                process_closed = true;
                                println!("SIGTERM关闭成功: {} (PID: {})", process_name, pid);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("发送SIGTERM失败: {} (PID: {}), 错误: {}", process_name, pid, e);
                }
            }

            // 如果SIGTERM失败，使用SIGKILL强制关闭
            if !process_closed {
                println!("SIGTERM无效，尝试SIGKILL强制关闭: {} (PID: {})", process_name, pid);
                let kill_output = Command::new("kill")
                    .args(&["-KILL", &pid.to_string()])
                    .output();

                match kill_output {
                    Ok(result) => {
                        if result.status.success() {
                            closed_count += 1;
                            process_closed = true;
                            println!("SIGKILL关闭成功: {} (PID: {})", process_name, pid);
                        } else {
                            let stderr = String::from_utf8_lossy(&result.stderr);
                            println!("SIGKILL失败: {} (PID: {}), 错误: {}", process_name, pid, stderr);
                        }
                    }
                    Err(e) => {
                        println!("SIGKILL命令执行失败: {} (PID: {}), 错误: {}", process_name, pid, e);
                    }
                }
            }

            // 如果进程仍然没有关闭，记录到失败列表
            if !process_closed {
                failed_processes.push(format!("{} (PID: {})", process_name, pid));
            }
        }
    }

    // 等待所有进程完全关闭
    if closed_count > 0 {
        println!("等待进程完全关闭...");
        tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;

        // 再次检查是否还有残留进程
        system.refresh_all();
        let mut remaining_processes = Vec::new();
        for process in system.processes().values() {
            let process_name = process.name();
            if process_names.iter().any(|name| process_name.contains(name)) {
                remaining_processes.push(format!("{} (PID: {})", process_name, process.pid()));
            }
        }

        if !remaining_processes.is_empty() {
            println!("警告: 发现残留进程: {:?}", remaining_processes);
            // 将残留进程添加到失败列表，但不阻止继续执行
            for remaining in remaining_processes {
                if !failed_processes.contains(&remaining) {
                    failed_processes.push(remaining);
                }
            }
        }
    }

    // 构建结果消息
    let mut result_message = if closed_count > 0 {
        format!("关闭进程: 成功关闭 {} 个 {} 进程", closed_count, editor_display_name)
    } else {
        format!("关闭进程: 未发现运行中的 {} 进程", editor_display_name)
    };

    // 如果有失败的进程，添加警告信息但不返回错误
    if !failed_processes.is_empty() {
        result_message.push_str(&format!(" (警告: {} 个进程可能未完全关闭)", failed_processes.len()));
        println!("部分进程关闭失败: {:?}", failed_processes);
    }

    Ok(result_message)
}

#[tauri::command]
async fn clean_editor_database(editor_type: String, keyword: String) -> Result<String, String> {
    // 获取编辑器显示名称
    let editor_display_name = match editor_type.as_str() {
        "vscode" => "VS Code",
        "cursor" => "Cursor",
        "kiro" => "Kiro",
        "trae" => "Trae",
        "windsurf" => "Windsurf",
        "qoder" => "Qoder",
        "vscodium" => "VSCodium",
        "codebuddy" => "CodeBuddy",
        "idea" => "IntelliJ IDEA",
        "pycharm" => "PyCharm",
        "goland" => "GoLand",
        "rustrover" => "RustRover",
        "webstorm" => "WebStorm",
        "phpstorm" => "PhpStorm",
        "clion" => "CLion",
        "datagrip" => "DataGrip",
        "rider" => "Rider",
        "rubymine" => "RubyMine",
        "aqua" => "Aqua",
        "androidstudio" => "Android Studio",
        _ => &editor_type,
    };

    // 检查是否为 JetBrains 系列编辑器
    let jetbrains_editors = ["idea", "pycharm", "goland", "rustrover", "webstorm", "phpstorm",
                            "clion", "datagrip", "rider", "rubymine", "aqua", "androidstudio"];
    if jetbrains_editors.contains(&editor_type.as_str()) {
        return Ok(format!("清理数据: {} 使用XML配置，无需数据库清理", editor_display_name));
    }

    let (db_path, _) = get_editor_paths(&editor_type)?;
    let db_path = db_path.ok_or("无法获取数据库路径")?;

    if !db_path.exists() {
        return Ok(format!("清理数据: {} 数据库文件不存在，跳过清理", editor_display_name));
    }

    // 创建备份，如果失败则跳过
    let backup_path = db_path.with_extension("vscdb.backup");
    if let Err(e) = std::fs::copy(&db_path, &backup_path) {
        println!("警告: 创建备份失败: {}, 继续执行...", e);
    }

    println!("开始连接数据库: {}", db_path.display());

    // 连接数据库并清理，添加重试机制
    let mut conn_result = Connection::open(&db_path);
    let mut retry_count = 0;

    while conn_result.is_err() && retry_count < 3 {
        retry_count += 1;
        println!("数据库连接失败，重试第 {} 次...", retry_count);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        conn_result = Connection::open(&db_path);
    }

    let conn = conn_result.map_err(|e| format!("打开数据库失败 (重试{}次后): {}", retry_count, e))?;
    println!("数据库连接成功");

    println!("开始查询包含关键字 '{}' 的条目", keyword);

    // 查询匹配的条目
    let mut stmt = conn.prepare("SELECT key FROM ItemTable WHERE LOWER(key) LIKE LOWER(?)")
        .map_err(|e| format!("准备查询失败: {}", e))?;

    let keyword_pattern = format!("%{}%", keyword);
    println!("查询模式: {}", keyword_pattern);

    let rows = stmt.query_map([&keyword_pattern], |row| {
        Ok(row.get::<_, String>(0)?)
    }).map_err(|e| format!("查询失败: {}", e))?;

    println!("查询执行成功，开始收集结果");

    let mut matching_keys = Vec::new();
    for row in rows {
        let key = row.map_err(|e| format!("读取行失败: {}", e))?;
        println!("找到匹配的键: {}", key);
        matching_keys.push(key);
    }

    println!("总共找到 {} 个匹配的键", matching_keys.len());

    if matching_keys.is_empty() {
        return Ok(format!("清理数据: {} 数据库中未找到需要清理的条目", editor_display_name));
    }

    println!("开始删除匹配的条目");
    // 删除匹配的条目
    let deleted_count = conn.execute(
        "DELETE FROM ItemTable WHERE LOWER(key) LIKE LOWER(?)",
        [&keyword_pattern]
    ).map_err(|e| format!("删除失败: {}", e))?;

    println!("删除操作完成，删除了 {} 个条目", deleted_count);

    Ok(format!("清理数据: {} 成功删除 {} 个包含关键字的条目", editor_display_name, deleted_count))
}

#[tauri::command]
async fn reset_editor_telemetry(editor_type: String) -> Result<String, String> {
    // 获取编辑器显示名称
    let editor_display_name = match editor_type.as_str() {
        "vscode" => "VS Code",
        "cursor" => "Cursor",
        "kiro" => "Kiro",
        "trae" => "Trae",
        "windsurf" => "Windsurf",
        "qoder" => "Qoder",
        "vscodium" => "VSCodium",
        "codebuddy" => "CodeBuddy",
        "idea" => "IntelliJ IDEA",
        "pycharm" => "PyCharm",
        "goland" => "GoLand",
        "rustrover" => "RustRover",
        "webstorm" => "WebStorm",
        "phpstorm" => "PhpStorm",
        "clion" => "CLion",
        "datagrip" => "DataGrip",
        "rider" => "Rider",
        "rubymine" => "RubyMine",
        "aqua" => "Aqua",
        "androidstudio" => "Android Studio",
        _ => &editor_type,
    };

    // 检查是否为 JetBrains 系列编辑器
    let jetbrains_editors = ["idea", "pycharm", "goland", "rustrover", "webstorm", "phpstorm",
                            "clion", "datagrip", "rider", "rubymine", "aqua", "androidstudio"];
    if jetbrains_editors.contains(&editor_type.as_str()) {
        return reset_jetbrains_session_id().await;
    }

    let (_, storage_path) = get_editor_paths(&editor_type)?;
    let storage_path = storage_path.ok_or("无法获取存储文件路径")?;

    if !storage_path.exists() {
        return Ok(format!("重置遥测: {} 存储文件不存在，跳过重置", editor_display_name));
    }

    // 检查文件权限
    let metadata = std::fs::metadata(&storage_path)
        .map_err(|e| format!("无法读取文件元数据: {}", e))?;

    println!("文件权限检查:");
    println!("  - 文件大小: {} 字节", metadata.len());
    println!("  - 只读状态: {}", metadata.permissions().readonly());

    // 如果文件是只读的，尝试修改权限
    if metadata.permissions().readonly() {
        println!("文件是只读的，尝试修改权限...");
        let mut perms = metadata.permissions();
        perms.set_readonly(false);
        if let Err(e) = std::fs::set_permissions(&storage_path, perms) {
            println!("警告: 无法修改文件权限: {}, 继续尝试...", e);
        } else {
            println!("文件权限修改成功");
        }
    }

    // 创建备份，如果失败则跳过
    let backup_path = storage_path.with_extension("json.backup");
    if let Err(e) = std::fs::copy(&storage_path, &backup_path) {
        println!("警告: 创建备份失败: {}, 继续执行...", e);
    }

    println!("开始读取存储文件: {}", storage_path.display());

    // 读取并修改JSON文件，添加重试机制
    let mut content_result = std::fs::read_to_string(&storage_path);
    let mut retry_count = 0;

    while content_result.is_err() && retry_count < 3 {
        retry_count += 1;
        println!("读取存储文件失败，重试第 {} 次...", retry_count);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        content_result = std::fs::read_to_string(&storage_path);
    }

    let content = content_result.map_err(|e| format!("读取存储文件失败 (重试{}次后): {}", retry_count, e))?;
    println!("存储文件读取成功，文件大小: {} 字节", content.len());

    println!("开始解析JSON数据");
    let mut data: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("解析JSON失败: {}", e))?;
    println!("JSON解析成功");

    println!("开始生成新的UUID");
    // 生成新的ID
    let new_machine_id = Uuid::new_v4().to_string();
    let new_device_id = Uuid::new_v4().to_string();
    println!("新的machine_id: {}", new_machine_id);
    println!("新的device_id: {}", new_device_id);

    let mut modified = false;
    println!("开始修改JSON数据");

    println!("开始检查根级对象");
    // 修改根级 machineId
    if let Some(obj) = data.as_object_mut() {
        println!("根级对象存在，包含 {} 个键", obj.len());

        if obj.contains_key("machineId") {
            println!("找到根级 machineId，正在修改...");
            obj.insert("machineId".to_string(), serde_json::Value::String(new_machine_id.clone()));
            modified = true;
            println!("根级 machineId 修改完成");
        } else {
            println!("根级不包含 machineId");
        }

        println!("开始检查遥测对象");
        // 修改遥测相关ID
        if let Some(telemetry) = obj.get_mut("telemetry") {
            println!("找到遥测对象");
            if let Some(telemetry_obj) = telemetry.as_object_mut() {
                println!("遥测对象包含 {} 个键", telemetry_obj.len());

                if telemetry_obj.contains_key("machineId") {
                    println!("找到遥测 machineId，正在修改...");
                    telemetry_obj.insert("machineId".to_string(), serde_json::Value::String(new_machine_id.clone()));
                    modified = true;
                    println!("遥测 machineId 修改完成");
                }

                if telemetry_obj.contains_key("devDeviceId") {
                    println!("找到 devDeviceId，正在修改...");
                    telemetry_obj.insert("devDeviceId".to_string(), serde_json::Value::String(new_device_id.clone()));
                    modified = true;
                    println!("devDeviceId 修改完成");
                }
            } else {
                println!("遥测对象不是JSON对象类型");
            }
        } else {
            println!("未找到遥测对象");
        }
    } else {
        println!("根级不是JSON对象类型");
    }

    println!("JSON数据修改完成，modified = {}", modified);

    // 如果没有找到标准字段，尝试查找其他可能的遥测相关字段
    if !modified {
        println!("未找到标准字段，开始搜索其他可能的遥测字段...");

        if let Some(obj) = data.as_object_mut() {
            // 打印所有根级键，帮助调试
            println!("根级键列表:");
            for key in obj.keys() {
                println!("  - {}", key);
            }

            // 搜索包含machine、device、telemetry等关键字的字段
            let mut keys_to_modify = Vec::new();
            for key in obj.keys() {
                let key_lower = key.to_lowercase();
                if key_lower.contains("machine") ||
                   key_lower.contains("device") ||
                   key_lower.contains("telemetry") ||
                   key_lower.contains("session") ||
                   key_lower.contains("instance") {
                    keys_to_modify.push(key.clone());
                }
            }

            println!("找到可能的遥测相关字段: {:?}", keys_to_modify);

            // 修改找到的字段
            let mut modified_fields = Vec::new();
            for key in keys_to_modify {
                println!("正在修改字段: {}", key);
                obj.insert(key.clone(), serde_json::Value::String(new_machine_id.clone()));
                modified_fields.push(key);
                modified = true;
            }

            if !modified_fields.is_empty() {
                println!("成功修改了 {} 个遥测字段: {:?}", modified_fields.len(), modified_fields);
            }
        }

        if !modified {
            println!("仍未找到可修改的字段，将添加新的machineId字段");
            if let Some(obj) = data.as_object_mut() {
                obj.insert("machineId".to_string(), serde_json::Value::String(new_machine_id.clone()));
                obj.insert("deviceId".to_string(), serde_json::Value::String(new_device_id.clone()));
                modified = true;
                println!("已添加新的遥测字段");
            }
        }
    }

    if !modified {
        return Ok("无法修改遥测ID字段".to_string());
    }

    println!("开始序列化修改后的JSON数据");
    // 写入修改后的内容
    let new_content = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("序列化JSON失败: {}", e))?;
    println!("JSON序列化成功，准备写入文件");

    // 使用临时文件 + 原子性重命名的方式写入，避免权限问题
    println!("开始写入文件，内容大小: {} 字节", new_content.len());

    // 获取原文件的所有权信息（仅在 Unix 系统上）
    #[cfg(unix)]
    let original_ownership = {
        use std::os::unix::fs::MetadataExt;
        if let Ok(metadata) = std::fs::metadata(&storage_path) {
            Some((metadata.uid(), metadata.gid()))
        } else {
            None
        }
    };

    // 使用同一目录下的唯一临时文件名，确保在同一文件系统上
    let parent_dir = storage_path.parent()
        .ok_or("无法获取父目录")?;
    let temp_path = parent_dir.join(format!("storage.{}.tmp", uuid::Uuid::new_v4()));

    let mut retry_count = 0;
    let max_retries = 3;
    let mut last_error = None;

    while retry_count < max_retries {
        retry_count += 1;

        // 尝试写入临时文件
        let write_result = std::fs::write(&temp_path, &new_content);

        match write_result {
            Ok(_) => {
                println!("临时文件写入成功");

                // 在 Unix 系统上，尝试恢复原文件的所有权
                #[cfg(unix)]
                if let Some((uid, gid)) = original_ownership {
                    use std::os::unix::fs::PermissionsExt;

                    // 尝试使用 chown 命令恢复所有权（需要 root 权限）
                    let chown_result = std::process::Command::new("chown")
                        .arg(format!("{}:{}", uid, gid))
                        .arg(&temp_path)
                        .output();

                    if let Ok(output) = chown_result {
                        if output.status.success() {
                            println!("成功恢复临时文件的所有权: uid={}, gid={}", uid, gid);
                        } else {
                            println!("警告: 无法恢复临时文件所有权，但继续执行...");
                        }
                    }

                    // 设置文件权限为 644 (rw-r--r--)
                    if let Ok(metadata) = std::fs::metadata(&temp_path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o644);
                        let _ = std::fs::set_permissions(&temp_path, perms);
                    }
                }

                // 尝试原子性重命名，如果失败则使用复制+删除的方式
                let rename_result = std::fs::rename(&temp_path, &storage_path);

                match rename_result {
                    Ok(_) => {
                        println!("文件写入成功，遥测ID重置完成");
                        break;
                    }
                    Err(rename_err) => {
                        println!("重命名文件失败 (第{}次): {} (错误代码: {:?})", retry_count, rename_err, rename_err.kind());

                        // 如果 rename 失败，尝试复制+删除
                        match std::fs::copy(&temp_path, &storage_path) {
                            Ok(_) => {
                                println!("使用复制方式写入成功");
                                let _ = std::fs::remove_file(&temp_path);
                                break;
                            }
                            Err(copy_err) => {
                                println!("复制文件也失败: {} (错误代码: {:?})", copy_err, copy_err.kind());
                                last_error = Some(rename_err);

                                // 清理临时文件
                                let _ = std::fs::remove_file(&temp_path);

                                if retry_count < max_retries {
                                    std::thread::sleep(std::time::Duration::from_millis(1000));
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("写入临时文件失败 (第{}次): {} (错误代码: {:?})", retry_count, e, e.kind());
                last_error = Some(e);

                if retry_count < max_retries {
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            }
        }
    }

    // 如果所有重试都失败，返回错误
    if retry_count >= max_retries && last_error.is_some() {
        let error = last_error.unwrap();
        return Err(format!(
            "写入文件失败 (重试{}次后): {} (错误类型: {:?})\n提示: 请确保 VS Code 已完全关闭，或尝试手动修改文件权限",
            retry_count, error, error.kind()
        ));
    }

    // 统计修改的字段数量
    let modified_count = if let Some(obj) = data.as_object() {
        obj.keys().filter(|key| {
            let key_lower = key.to_lowercase();
            key_lower.contains("machine") ||
            key_lower.contains("device") ||
            key_lower.contains("telemetry") ||
            key_lower.contains("session") ||
            key_lower.contains("instance")
        }).count()
    } else {
        0
    };

    Ok(format!("重置遥测: {} 成功重置 {} 个遥测字段", editor_display_name, modified_count))
}

/// 清除 Augment 聊天记录
#[tauri::command]
async fn clear_augment_chat_history(editor_type: String) -> Result<String, String> {
    use walkdir::WalkDir;

    let home_dir = dirs::home_dir().ok_or("无法获取用户主目录")?;

    // 获取编辑器显示名称
    let editor_display_name = match editor_type.as_str() {
        "vscode" => "VS Code",
        "cursor" => "Cursor",
        "kiro" => "Kiro",
        "trae" => "Trae",
        "windsurf" => "Windsurf",
        "qoder" => "Qoder",
        "vscodium" => "VSCodium",
        "codebuddy" => "CodeBuddy",
        "idea" => "IntelliJ IDEA",
        "pycharm" => "PyCharm",
        "goland" => "GoLand",
        "rustrover" => "RustRover",
        "webstorm" => "WebStorm",
        "phpstorm" => "PhpStorm",
        "clion" => "CLion",
        "datagrip" => "DataGrip",
        "rider" => "Rider",
        "rubymine" => "RubyMine",
        "aqua" => "Aqua",
        "androidstudio" => "Android Studio",
        _ => &editor_type,
    };

    // 获取 workspaceStorage 路径
    let workspace_storage_path = match editor_type.as_str() {
        // VSCode 系列编辑器
        "vscode" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                PathBuf::from(appdata).join("Code").join("User").join("workspaceStorage")
            }
            #[cfg(target_os = "macos")]
            {
                home_dir.join("Library").join("Application Support").join("Code").join("User").join("workspaceStorage")
            }
            #[cfg(target_os = "linux")]
            {
                home_dir.join(".config").join("Code").join("User").join("workspaceStorage")
            }
        },
        "cursor" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                PathBuf::from(appdata).join("Cursor").join("User").join("workspaceStorage")
            }
            #[cfg(target_os = "macos")]
            {
                home_dir.join("Library").join("Application Support").join("Cursor").join("User").join("workspaceStorage")
            }
            #[cfg(target_os = "linux")]
            {
                home_dir.join(".config").join("Cursor").join("User").join("workspaceStorage")
            }
        },
        "kiro" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                PathBuf::from(appdata).join("Kiro").join("User").join("workspaceStorage")
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                home_dir.join(".config").join("Kiro").join("User").join("workspaceStorage")
            }
        },
        "trae" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                PathBuf::from(appdata).join("Trae").join("User").join("workspaceStorage")
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                home_dir.join(".config").join("Trae").join("User").join("workspaceStorage")
            }
        },
        "windsurf" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                PathBuf::from(appdata).join("Windsurf").join("User").join("workspaceStorage")
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                home_dir.join(".config").join("Windsurf").join("User").join("workspaceStorage")
            }
        },
        "qoder" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                PathBuf::from(appdata).join("Qoder").join("User").join("workspaceStorage")
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                home_dir.join(".config").join("Qoder").join("User").join("workspaceStorage")
            }
        },
        "vscodium" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                PathBuf::from(appdata).join("VSCodium").join("User").join("workspaceStorage")
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                home_dir.join(".config").join("VSCodium").join("User").join("workspaceStorage")
            }
        },
        "codebuddy" => {
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
                PathBuf::from(appdata).join("CodeBuddy").join("User").join("workspaceStorage")
            }
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            {
                home_dir.join(".config").join("CodeBuddy").join("User").join("workspaceStorage")
            }
        },

        // JetBrains 系列 - 返回空路径，后面会特殊处理
        "idea" | "pycharm" | "goland" | "rustrover" | "webstorm" | "phpstorm" |
        "clion" | "datagrip" | "rider" | "rubymine" | "aqua" | "androidstudio" => {
            return clear_jetbrains_chat_history(&editor_type, editor_display_name);
        },

        _ => return Err(format!("不支持的编辑器类型: {}", editor_type)),
    };

    if !workspace_storage_path.exists() {
        return Ok(format!("清除聊天记录: {} 的 workspaceStorage 目录不存在，跳过清除", editor_display_name));
    }

    println!("开始清除 {} 的聊天记录，路径: {:?}", editor_display_name, workspace_storage_path);

    let mut deleted_count = 0;
    let mut error_count = 0;

    // 遍历 workspaceStorage 下的所有工作区文件夹
    for entry in WalkDir::new(&workspace_storage_path)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // 查找 Augment 相关的聊天记录文件
        if file_name.contains("augment") &&
           (file_name.ends_with(".json") ||
            file_name.contains("chat") ||
            file_name.contains("conversation") ||
            file_name.contains("history")) {

            println!("找到 Augment 聊天记录文件: {:?}", path);

            match std::fs::remove_file(path) {
                Ok(_) => {
                    println!("成功删除: {:?}", path);
                    deleted_count += 1;
                },
                Err(e) => {
                    println!("删除失败: {:?}, 错误: {}", path, e);
                    error_count += 1;
                }
            }
        }

        // 同时检查目录名，如果是 augmentcode.augment-* 开头的目录，删除整个目录
        if path.is_dir() && file_name.starts_with("augmentcode.augment") {
            println!("找到 Augment 扩展目录: {:?}", path);

            match std::fs::remove_dir_all(path) {
                Ok(_) => {
                    println!("成功删除目录: {:?}", path);
                    deleted_count += 1;
                },
                Err(e) => {
                    println!("删除目录失败: {:?}, 错误: {}", path, e);
                    error_count += 1;
                }
            }
        }
    }

    if deleted_count == 0 && error_count == 0 {
        Ok(format!("清除聊天记录: {} 未找到 Augment 聊天记录", editor_display_name))
    } else if error_count > 0 {
        Ok(format!("清除聊天记录: {} 删除了 {} 个文件/目录，{} 个失败",
            editor_display_name, deleted_count, error_count))
    } else {
        Ok(format!("清除聊天记录: {} 成功删除 {} 个文件/目录",
            editor_display_name, deleted_count))
    }
}

/// 清除 JetBrains 系列编辑器的 Augment 聊天记录
fn clear_jetbrains_chat_history(editor_type: &str, editor_display_name: &str) -> Result<String, String> {
    use walkdir::WalkDir;

    let home_dir = dirs::home_dir().ok_or("无法获取用户主目录")?;

    // JetBrains 配置目录路径
    let config_base = if cfg!(target_os = "windows") {
        let appdata = std::env::var("APPDATA").map_err(|_| "无法获取APPDATA环境变量")?;
        PathBuf::from(appdata).join("JetBrains")
    } else if cfg!(target_os = "macos") {
        home_dir.join("Library").join("Application Support").join("JetBrains")
    } else {
        home_dir.join(".config").join("JetBrains")
    };

    // 根据编辑器类型确定配置目录前缀
    let dir_prefix = match editor_type {
        "idea" => "IntelliJIdea",
        "pycharm" => "PyCharm",
        "goland" => "GoLand",
        "rustrover" => "RustRover",
        "webstorm" => "WebStorm",
        "phpstorm" => "PhpStorm",
        "clion" => "CLion",
        "datagrip" => "DataGrip",
        "rider" => "Rider",
        "rubymine" => "RubyMine",
        "aqua" => "Aqua",
        "androidstudio" => "AndroidStudio",
        _ => return Err(format!("不支持的 JetBrains 编辑器: {}", editor_type)),
    };

    if !config_base.exists() {
        return Ok(format!("清除聊天记录: {} 配置目录不存在，跳过清除", editor_display_name));
    }

    println!("开始清除 {} 的聊天记录，配置目录: {:?}", editor_display_name, config_base);

    let mut deleted_count = 0;
    let mut error_count = 0;

    // 遍历 JetBrains 配置目录，查找对应编辑器的版本目录
    for entry in std::fs::read_dir(&config_base).map_err(|e| format!("无法读取目录: {}", e))? {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // 检查是否是目标编辑器的配置目录（例如：IntelliJIdea2024.1）
        if !dir_name.starts_with(dir_prefix) {
            continue;
        }

        println!("找到 {} 配置目录: {:?}", editor_display_name, path);

        // 在配置目录中查找 Augment 相关文件
        // JetBrains 插件数据通常存储在：
        // - options/ 目录下的 XML 文件
        // - workspace/ 目录
        let options_dir = path.join("options");
        let workspace_dir = path.join("workspace");

        // 清除 options 目录中的 Augment 配置
        if options_dir.exists() {
            for file_entry in WalkDir::new(&options_dir)
                .max_depth(2)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let file_path = file_entry.path();
                let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if file_name.to_lowercase().contains("augment") {
                    println!("找到 Augment 配置文件: {:?}", file_path);

                    match std::fs::remove_file(file_path) {
                        Ok(_) => {
                            println!("成功删除: {:?}", file_path);
                            deleted_count += 1;
                        },
                        Err(e) => {
                            println!("删除失败: {:?}, 错误: {}", file_path, e);
                            error_count += 1;
                        }
                    }
                }
            }
        }

        // 清除 workspace 目录中的 Augment 数据
        if workspace_dir.exists() {
            for file_entry in WalkDir::new(&workspace_dir)
                .max_depth(3)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let file_path = file_entry.path();
                let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if file_name.to_lowercase().contains("augment") {
                    println!("找到 Augment 工作区文件: {:?}", file_path);

                    if file_path.is_dir() {
                        match std::fs::remove_dir_all(file_path) {
                            Ok(_) => {
                                println!("成功删除目录: {:?}", file_path);
                                deleted_count += 1;
                            },
                            Err(e) => {
                                println!("删除目录失败: {:?}, 错误: {}", file_path, e);
                                error_count += 1;
                            }
                        }
                    } else {
                        match std::fs::remove_file(file_path) {
                            Ok(_) => {
                                println!("成功删除: {:?}", file_path);
                                deleted_count += 1;
                            },
                            Err(e) => {
                                println!("删除失败: {:?}, 错误: {}", file_path, e);
                                error_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    if deleted_count == 0 && error_count == 0 {
        Ok(format!("清除聊天记录: {} 未找到 Augment 聊天记录", editor_display_name))
    } else if error_count > 0 {
        Ok(format!("清除聊天记录: {} 删除了 {} 个文件/目录，{} 个失败",
            editor_display_name, deleted_count, error_count))
    } else {
        Ok(format!("清除聊天记录: {} 成功删除 {} 个文件/目录",
            editor_display_name, deleted_count))
    }
}

async fn reset_jetbrains_session_id() -> Result<String, String> {
    let home_dir = dirs::home_dir().ok_or("无法获取用户主目录")?;
    let new_session_id = Uuid::new_v4().to_string();

    // JetBrains 配置目录
    #[cfg(target_os = "windows")]
    let jetbrains_base = home_dir.join("AppData").join("Roaming").join("JetBrains");
    #[cfg(target_os = "macos")]
    let jetbrains_base = home_dir.join("Library").join("Application Support").join("JetBrains");
    #[cfg(target_os = "linux")]
    let jetbrains_base = home_dir.join(".config").join("JetBrains");

    if !jetbrains_base.exists() {
        return Err("未找到 JetBrains 配置目录".to_string());
    }

    let mut success_count = 0;
    let mut total_count = 0;

    // 遍历所有 JetBrains 产品目录
    for entry in std::fs::read_dir(&jetbrains_base).map_err(|e| format!("读取目录失败: {}", e))? {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            let options_dir = path.join("options");
            let ide_general_file = options_dir.join("ide.general.xml");

            // 检查是否有字体配置文件，如果有则跳过以保护用户设置
            let font_config_file = options_dir.join("font.options.xml");
            if font_config_file.exists() {
                continue;
            }

            total_count += 1;

            // 确保 options 目录存在
            std::fs::create_dir_all(&options_dir).ok();

            // 创建或修改 ide.general.xml
            let xml_content = if ide_general_file.exists() {
                std::fs::read_to_string(&ide_general_file).unwrap_or_default()
            } else {
                r#"<?xml version="1.0" encoding="UTF-8"?>
<application>
  <component name="GeneralSettings">
  </component>
</application>"#.to_string()
            };

            // 简单的XML处理 - 在实际项目中应该使用专门的XML库
            let updated_xml = if xml_content.contains("augment.session.id") {
                // 替换现有的session id
                regex::Regex::new(r#"<property name="augment\.session\.id" value="[^"]*" />"#)
                    .unwrap()
                    .replace(&xml_content, &format!(r#"<property name="augment.session.id" value="{}" />"#, new_session_id))
                    .to_string()
            } else {
                // 添加新的session id
                xml_content.replace(
                    r#"<component name="GeneralSettings">"#,
                    &format!(r#"<component name="GeneralSettings">
    <property name="augment.session.id" value="{}" />"#, new_session_id)
                )
            };

            if std::fs::write(&ide_general_file, updated_xml).is_ok() {
                success_count += 1;
            }
        }
    }

    if success_count > 0 {
        Ok(format!("重置遥测: 成功更新 {}/{} 个 JetBrains 产品的 SessionID", success_count, total_count))
    } else {
        Ok("重置遥测: JetBrains 未找到可更新的产品配置".to_string())
    }
}

fn main() {
    let mut builder = tauri::Builder::default();

    // 在桌面平台上添加 single-instance 插件
    // 这个插件必须是第一个注册的插件，用于防止多实例运行
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            // 聚焦主窗口
            if let Some(main_window) = app.get_webview_window("main") {
                let _ = main_window.set_focus();
                let _ = main_window.unminimize();
            }

            // 处理 deep-link URL（从 argv 中查找）
            for arg in argv {
                if arg.starts_with("zaugment://") {
                    // 解析 URL 获取 session 参数
                    if let Ok(parsed_url) = url::Url::parse(&arg) {
                        if let Some(session) = parsed_url.query_pairs().find(|(k, _)| k == "session").map(|(_, v)| v.to_string()) {
                            // 发送 session 到前端
                            let app_handle = app.app_handle().clone();
                            let session_clone = session.clone();

                            // 等待一下确保前端已经准备好
                            std::thread::spawn(move || {
                                std::thread::sleep(std::time::Duration::from_millis(1000));

                                let _ = app_handle.emit(
                                    "deep-link-session-received",
                                    serde_json::json!({
                                        "session": session_clone
                                    })
                                );
                            });
                        }
                    }

                    break;
                }
            }
        }));
    }

    builder
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            println!("应用启动中...");

            // 最小化状态管理
            let app_state = AppState {
                augment_oauth_state: Mutex::new(None),
                http_server: Mutex::new(None),
                outlook_manager: Mutex::new(OutlookManager::new()),
                storage_manager: Arc::new(Mutex::new(None)),
                custom_data_dir: Arc::new(Mutex::new(None)),
                webdav_config: Arc::new(Mutex::new(None)),
                cloud_sync: Arc::new(Mutex::new(None)),
                password_manager: Arc::new(PasswordManager::new()),
                app_session_cache: Arc::new(Mutex::new(HashMap::new())),
            };

            app.manage(app_state);

            println!("状态管理器初始化完成");

            // 注册 Deep-Link 协议处理
            // 在 Windows 和 Linux 上总是注册，macOS 通过 bundle 配置
            #[cfg(any(target_os = "linux", windows))]
            {
                let _ = app.deep_link().register_all();
            }

            // 处理 Deep-Link 事件（仅在首次启动时触发）
            let app_handle = app.app_handle().clone();
            app.listen("deep-link://new-deep-link", move |event| {
                // 从 URL 中提取 session 参数
                let payload = event.payload();

                if let Ok(url_str) = serde_json::from_str::<Vec<String>>(payload) {
                    if let Some(url) = url_str.first() {
                        // 解析 URL 获取 session 参数
                        if let Ok(parsed_url) = url::Url::parse(url) {
                            if let Some(session) = parsed_url.query_pairs().find(|(k, _)| k == "session").map(|(_, v)| v.to_string()) {
                                // 等待主窗口加载完成（最多等待 10 秒）
                                let app_handle_clone = app_handle.clone();
                                let session_clone = session.clone();

                                tokio::spawn(async move {
                                    let mut attempts = 0;
                                    let max_attempts = 100; // 100 * 100ms = 10 秒

                                    while attempts < max_attempts {
                                        if let Some(main_window) = app_handle_clone.get_webview_window("main") {
                                            // 窗口存在，再等待一小段时间确保前端事件监听器已注册
                                            tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;

                                            // 聚焦主窗口
                                            let _ = main_window.set_focus();
                                            let _ = main_window.unminimize();

                                            // 发送 session 到前端，由前端调用导入方法
                                            let _ = app_handle_clone.emit(
                                                "deep-link-session-received",
                                                serde_json::json!({
                                                    "session": session_clone
                                                })
                                            );
                                            break;
                                        }

                                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                        attempts += 1;
                                    }

                                    if attempts >= max_attempts {
                                        eprintln!("⚠️ Timeout waiting for main window to be ready");
                                    }
                                });
                            }
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_auth_url,
            generate_augment_auth_url,
            get_token,
            get_augment_token,
            check_account_status,
            batch_check_tokens_status,
            fetch_batch_credit_consumption,
            add_token_from_session,
            open_url,
            // 新的简化命令
            save_tokens_json,
            load_tokens_json,
            // 文件操作命令
            save_file_dialog,
            write_file_content,
            // 书签管理命令
            add_bookmark,
            update_bookmark,
            delete_bookmark,
            get_bookmarks,
            get_all_bookmarks,
            // API 调用命令
            get_customer_info,
            get_subscriptions_from_link,
            get_ledger_summary,
            test_api_call,
            open_data_folder,
            open_editor_with_protocol,
            create_jetbrains_token_file,
            // Outlook 邮箱管理命令
            outlook_save_credentials,
            outlook_get_all_accounts,
            outlook_delete_account,
            outlook_check_account_status,
            outlook_get_emails,
            outlook_get_email_details,
            // 删除命令
            delete_token,
            // 设置页面命令
            select_data_directory,
            get_current_data_path,
            get_data_directory,
            reset_data_directory,
            open_data_directory,
            
            // WebDAV云同步命令
            configure_webdav,
            test_webdav_connection,
            sync_to_cloud,
            force_upload_to_cloud,
            force_download_from_cloud,
            get_webdav_config,
            
            // 冲突检测和解决命令
            check_sync_conflicts,
            resolve_sync_conflict,
            
            // 统一配置管理命令
            save_app_settings_cmd,
            get_app_settings_cmd,
            get_unified_config_cmd,
            save_ui_settings_cmd,

            // 阈值配置管理命令
            save_status_thresholds,
            load_status_thresholds,

            // 版本检查命令
            get_app_version,
            check_for_updates,

            open_internal_browser,
            close_window,
            open_with_chrome,
            open_with_custom_browser,
            select_file,

            // 窗口控制命令
            minimize_window,
            maximize_window,
            unmaximize_window,
            close_app_window,
            start_drag,
            set_window_size,
            toggle_fullscreen,
            set_fullscreen,
            get_window_state,
            get_window_size,

            // 编辑器重置功能命令
            check_editor_processes,
            close_editor_processes,
            clean_editor_database,
            reset_editor_telemetry,
            clear_augment_chat_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
