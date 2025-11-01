use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use reqwest;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

const CLIENT_ID: &str = "v";
const AUTH_BASE_URL: &str = "https://auth.augmentcode.com";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AugmentOAuthState {
    pub code_verifier: String,
    pub code_challenge: String,
    pub state: String,
    pub creation_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedCode {
    pub code: String,
    pub state: String,
    pub tenant_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AugmentTokenResponse {
    pub access_token: String,
    pub tenant_url: String,
    pub email: Option<String>,           // 从 get-models API 获取的邮箱
    pub credits_balance: Option<i32>,    // 从 get-credit-info 获取的余额
    pub expiry_date: Option<String>,     // 从 get-credit-info 获取的过期时间
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountStatus {
    pub is_banned: bool,
    pub status: String,
    pub error_message: Option<String>,
    pub response_code: Option<u16>,
    // 调试信息
    pub debug_info: DebugInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugInfo {
    pub request_url: String,
    pub request_headers: std::collections::HashMap<String, String>,
    pub request_body: String,
    pub response_headers: std::collections::HashMap<String, String>,
    pub response_body: String,
    pub response_status_text: String,
}

// 批量检测相关结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub access_token: String,
    pub tenant_url: String,
    pub id: Option<String>, // 用于前端识别是哪个token
    pub portal_url: Option<String>, // Portal URL用于获取使用次数信息
    pub auth_session: Option<String>, // Auth session用于自动刷新token
    pub email_note: Option<String>, // 邮箱备注,用于判断是否需要获取邮箱
}

// Portal信息结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct PortalInfo {
    pub credits_balance: i32,
    pub expiry_date: Option<String>,
}

// get-credit-info API 响应结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditInfoResponse {
    pub usage_units_remaining: f64,  // API 返回浮点数
    pub usage_units_total_current_billing_cycle: f64,
    pub usage_units_total_additional: f64,
    pub is_credit_balance_low: bool,
    pub display_info: Option<serde_json::Value>,
    pub refreshed_at: String,
    pub included_usage_units_per_billing_cycle: f64,
    pub current_billing_cycle_end_date_iso: String,
    pub credit_details: Option<Vec<serde_json::Value>>,
    pub usage_units_total: f64,
}

// get-models API 响应结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub user: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenStatusResult {
    pub token_id: Option<String>, // 对应输入的id
    pub access_token: String, // 保留token用于前端更新 (如果被刷新,这里是新token)
    pub tenant_url: String, // 保留tenant_url用于前端更新 (如果被刷新,这里是新url)
    pub portal_url: Option<String>, // Portal URL (如果被刷新,这里是新url)
    pub status_result: AccountStatus,
    pub portal_info: Option<PortalInfo>, // Portal信息（如果有）
    pub portal_error: Option<String>, // Portal获取错误（如果有）
    pub suspensions: Option<serde_json::Value>, // 封禁详情（如果有）
    pub email_note: Option<String>, // 邮箱备注（如果获取到）
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub response_text: String,
    pub request_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub chat_history: Vec<ChatMessage>,
    pub message: String,
    pub mode: String,
}

#[derive(Debug, Deserialize)]
struct TokenApiResponse {
    access_token: String,
}

/// Base64 URL encode without padding
fn base64_url_encode(data: &[u8]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(data)
}

/// Create SHA256 hash
fn sha256_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Generate random bytes
fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.r#gen()).collect()
}

/// Create OAuth state with code verifier, challenge and state
pub fn create_augment_oauth_state() -> AugmentOAuthState {
    let code_verifier_bytes = generate_random_bytes(32);
    let code_verifier = base64_url_encode(&code_verifier_bytes);
    
    let code_challenge_bytes = sha256_hash(code_verifier.as_bytes());
    let code_challenge = base64_url_encode(&code_challenge_bytes);
    
    let state_bytes = generate_random_bytes(8);
    let state = base64_url_encode(&state_bytes);
    
    let creation_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    
    AugmentOAuthState {
        code_verifier,
        code_challenge,
        state,
        creation_time,
    }
}

/// Generate OAuth authorization URL
pub fn generate_augment_authorize_url(oauth_state: &AugmentOAuthState) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = Url::parse(&format!("{}/authorize", AUTH_BASE_URL))?;
    
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("code_challenge", &oauth_state.code_challenge)
        .append_pair("client_id", CLIENT_ID)
        .append_pair("state", &oauth_state.state)
        .append_pair("prompt", "login");
    
    Ok(url.to_string())
}

/// Parse the authorization code response
pub fn parse_code(code: &str) -> Result<ParsedCode, Box<dyn std::error::Error>> {
    let parsed: ParsedCode = serde_json::from_str(code)?;
    Ok(parsed)
}

/// Get access token using authorization code
pub async fn get_augment_access_token(
    tenant_url: &str,
    code_verifier: &str,
    code: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    
    let mut data = HashMap::new();
    data.insert("grant_type", "authorization_code");
    data.insert("client_id", CLIENT_ID);
    data.insert("code_verifier", code_verifier);
    data.insert("redirect_uri", "");
    data.insert("code", code);
    
    let token_url = format!("{}token", tenant_url);
    let response = client
        .post(&token_url)
        .json(&data)
        .send()
        .await?;

    let token_response: TokenApiResponse = response.json().await?;
    Ok(token_response.access_token)
}

/// Complete OAuth flow and return token with tenant URL
pub async fn complete_augment_oauth_flow(
    oauth_state: &AugmentOAuthState,
    code_input: &str,
) -> Result<AugmentTokenResponse, Box<dyn std::error::Error>> {
    let parsed_code = parse_code(code_input)?;

    let token = get_augment_access_token(
        &parsed_code.tenant_url,
        &oauth_state.code_verifier,
        &parsed_code.code,
    ).await?;

    // 并行获取用户邮箱和 credit 信息
    let (email_result, credit_result) = tokio::join!(
        get_models(&token, &parsed_code.tenant_url),
        get_credit_info(&token, &parsed_code.tenant_url)
    );

    // 处理邮箱结果
    let email = match email_result {
        Ok(models_response) => Some(models_response.user.email),
        Err(err) => {
            println!("Failed to get user email: {}", err);
            None
        }
    };

    // 处理 credit 信息结果
    let (credits_balance, expiry_date) = match credit_result {
        Ok(credit_info) => (
            Some(credit_info.usage_units_remaining.floor() as i32),  // 转换为整数
            Some(credit_info.current_billing_cycle_end_date_iso),
        ),
        Err(err) => {
            println!("Failed to get credit info: {}", err);
            (None, None)
        }
    };

    Ok(AugmentTokenResponse {
        access_token: token,
        tenant_url: parsed_code.tenant_url,
        email,
        credits_balance,
        expiry_date,
    })
}

/// Check account ban status by testing find-missing API
pub async fn check_account_ban_status(
    token: &str,
    tenant_url: &str,
) -> Result<AccountStatus, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // Ensure tenant_url ends with a slash
    let base_url = if tenant_url.ends_with('/') {
        tenant_url.to_string()
    } else {
        format!("{}/", tenant_url)
    };

    let api_url = format!("{}find-missing", base_url);

    // Empty request body for find-missing endpoint
    let request_body = serde_json::json!({});

    // Prepare request headers for debugging
    let mut request_headers = HashMap::new();
    request_headers.insert("Content-Type".to_string(), "application/json".to_string());
    request_headers.insert("Authorization".to_string(), format!("Bearer {}", token));

    // Print debug info to console
    println!("=== API Request Debug Info ===");
    println!("URL: {}", api_url);
    println!("Method: POST");
    println!("Headers: {:#?}", request_headers);
    println!("Request Body: {}", request_body.to_string());
    println!("==============================");

    // Send request to find-missing API
    let response = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&request_body)
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let status_text = response.status().to_string();

    // Collect response headers
    let mut response_headers = HashMap::new();
    for (name, value) in response.headers() {
        response_headers.insert(
            name.to_string(),
            value.to_str().unwrap_or("<invalid utf8>").to_string(),
        );
    }

    // Print response debug info
    println!("=== API Response Debug Info ===");
    println!("Status Code: {} ({})", status_code, status_text);
    println!("Response Headers: {:#?}", response_headers);

    // Read response body
    let response_body = response.text().await?;
    println!("Response Body: {}", response_body);
    println!("===============================");

    // Create debug info
    let debug_info = DebugInfo {
        request_url: api_url.clone(),
        request_headers,
        request_body: request_body.to_string(),
        response_headers,
        response_body: response_body.clone(),
        response_status_text: status_text,
    };

    // Analyze response to determine ban status
    if (200..300).contains(&status_code) {
        // Check for various status indicators in response body
        if response_body.contains("suspended") {
            Ok(AccountStatus {
                is_banned: true,
                status: "SUSPENDED".to_string(),
                error_message: Some("Account is suspended based on response content".to_string()),
                response_code: Some(status_code),
                debug_info,
            })
        } else if response_body.to_lowercase().contains("invalid token") {
            Ok(AccountStatus {
                is_banned: false,
                status: "INVALID_TOKEN".to_string(),
                error_message: Some("Token is invalid".to_string()),
                response_code: Some(status_code),
                debug_info,
            })
        } else if response_body.contains("You are out of user messages")
                || response_body.contains("out of user messages")
                || response_body.contains("Please update your account")
                || (response_body.contains("continue using Augment") && !response_body.contains("Please upgrade to the latest version")) {
            Ok(AccountStatus {
                is_banned: true,
                status: "EXPIRED".to_string(),
                error_message: Some("Account has expired or run out of credits".to_string()),
                response_code: Some(status_code),
                debug_info,
            })
        } else {
            Ok(AccountStatus {
                is_banned: false,
                status: "ACTIVE".to_string(),
                error_message: None,
                response_code: Some(status_code),
                debug_info,
            })
        }
    } else {
        // Handle different error status codes
        let (is_banned, status, error_message) = match status_code {
            401 => {
                // 401 通常表示 Token 无效，需要进一步判断是封禁还是 Token 失效
                if response_body.contains("suspended") || response_body.contains("banned") {
                    (true, "SUSPENDED", "Account is suspended")
                } else {
                    (true, "INVALID_TOKEN", "Token is invalid or expired")
                }
            },
            403 => {
                // 403 可能是封禁或权限问题
                if response_body.contains("suspended") || response_body.contains("banned") {
                    (true, "SUSPENDED", "Account is suspended")
                } else {
                    (true, "FORBIDDEN", "Access forbidden - account may be banned")
                }
            },
            429 => (false, "RATE_LIMITED", "Rate limited - account is active but throttled"),
            500..=599 => (false, "SERVER_ERROR", "Server error - cannot determine ban status"),
            _ => (true, "UNKNOWN_ERROR", "Unknown error - possible ban"),
        };

        Ok(AccountStatus {
            is_banned,
            status: status.to_string(),
            error_message: Some(format!("{}: {}", error_message, response_body)),
            response_code: Some(status_code),
            debug_info,
        })
    }
}

/// 从 auth session 中提取 access token
pub async fn extract_token_from_session(session: &str) -> Result<AugmentTokenResponse, String> {
    use regex::Regex;

    // 生成 PKCE 参数
    let code_verifier = generate_random_string(32);
    let code_challenge = base64_url_encode(&sha256_hash(code_verifier.as_bytes()));
    let state = generate_random_string(42);
    let client_id = CLIENT_ID;

    // 使用 session 访问 terms-accept 获取 HTML
    let terms_url = format!(
        "{}/terms-accept?response_type=code&code_challenge={}&client_id={}&state={}&prompt=login",
        AUTH_BASE_URL, code_challenge, client_id, state
    );

    let client = reqwest::Client::new();
    let html_response = client
        .get(&terms_url)
        .header("Cookie", format!("session={}", session))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch terms page: {}", e))?;

    let html = html_response
        .text()
        .await
        .map_err(|e| format!("Failed to read HTML response: {}", e))?;

    // 使用正则表达式提取 code, state, tenant_url
    let code_regex = Regex::new(r#"code:\s*"([^"]+)""#).unwrap();
    let state_regex = Regex::new(r#"state:\s*"([^"]+)""#).unwrap();
    let tenant_url_regex = Regex::new(r#"tenant_url:\s*"([^"]+)""#).unwrap();

    let code = code_regex
        .captures(&html)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .ok_or("SESSION_ERROR_OR_ACCOUNT_BANNED")?;

    let parsed_state = state_regex
        .captures(&html)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .ok_or("SESSION_ERROR_OR_ACCOUNT_BANNED")?;

    let tenant_url = tenant_url_regex
        .captures(&html)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .ok_or("SESSION_ERROR_OR_ACCOUNT_BANNED")?;

    println!("Extracted - code: {}, state: {}, tenant_url: {}", code, parsed_state, tenant_url);

    // 用授权码换 Token
    let token_url = format!("{}token", tenant_url);
    let token_payload = serde_json::json!({
        "grant_type": "authorization_code",
        "client_id": client_id,
        "code_verifier": code_verifier,
        "redirect_uri": "",
        "code": code
    });

    let token_response = client
        .post(&token_url)
        .header("Content-Type", "application/json")
        .json(&token_payload)
        .send()
        .await
        .map_err(|e| format!("Failed to exchange token: {}", e))?;

    let token_data: TokenApiResponse = token_response
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    // 并行获取用户邮箱和 credit 信息
    let token = token_data.access_token.clone();
    let tenant_url_clone = tenant_url.to_string();

    let (email_result, credit_result) = tokio::join!(
        get_models(&token, &tenant_url_clone),
        get_credit_info(&token, &tenant_url_clone)
    );

    // 处理邮箱结果
    let email = match email_result {
        Ok(models_response) => Some(models_response.user.email),
        Err(err) => {
            println!("Failed to get user email from session: {}", err);
            None
        }
    };

    // 处理 credit 信息结果
    let (credits_balance, expiry_date) = match credit_result {
        Ok(credit_info) => {
            println!("✅ Successfully got credit info from session:");
            println!("   usage_units_remaining: {}", credit_info.usage_units_remaining);
            println!("   current_billing_cycle_end_date_iso: {}", credit_info.current_billing_cycle_end_date_iso);
            (
                Some(credit_info.usage_units_remaining.floor() as i32),  // 转换为整数
                Some(credit_info.current_billing_cycle_end_date_iso),
            )
        },
        Err(err) => {
            println!("❌ Failed to get credit info from session: {}", err);
            (None, None)
        }
    };

    println!("📦 Final AugmentTokenResponse:");
    println!("   credits_balance: {:?}", credits_balance);
    println!("   expiry_date: {:?}", expiry_date);

    Ok(AugmentTokenResponse {
        access_token: token_data.access_token,
        tenant_url: tenant_url.to_string(),
        email,
        credits_balance,
        expiry_date,
    })
}

/// 生成随机字符串
fn generate_random_string(length: usize) -> String {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut random_bytes = vec![0u8; length];
    rng.fill_bytes(&mut random_bytes);
    base64_url_encode(&random_bytes)
}

// 从Portal URL提取token
fn extract_token_from_portal_url(portal_url: &str) -> Option<String> {
    if let Ok(url) = url::Url::parse(portal_url) {
        url.query_pairs()
            .find(|(key, _)| key == "token")
            .map(|(_, value)| value.into_owned())
    } else {
        None
    }
}

// 获取Portal信息
async fn get_portal_info(portal_url: &str) -> Result<PortalInfo, String> {
    let token = extract_token_from_portal_url(portal_url)
        .ok_or("Failed to extract token from portal URL")?;

    // 获取customer信息
    let customer_url = format!("https://portal.withorb.com/api/v1/customer_from_link?token={}", token);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let customer_response = client
        .get(&customer_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
        .map_err(|e| format!("Failed to get customer info: {}", e))?;

    if !customer_response.status().is_success() {
        return Err(format!("Customer API request failed: {}", customer_response.status()));
    }

    let customer_text = customer_response.text().await
        .map_err(|e| format!("Failed to read customer response: {}", e))?;

    let customer_data: serde_json::Value = serde_json::from_str(&customer_text)
        .map_err(|e| format!("Failed to parse customer response: {}", e))?;

    // 提取customer_id和pricing_unit_id
    let customer_id = customer_data["customer"]["id"]
        .as_str()
        .ok_or("Customer ID not found")?;

    let pricing_unit_id = customer_data["customer"]["ledger_pricing_units"][0]["id"]
        .as_str()
        .ok_or("Pricing unit ID not found")?;

    // 获取ledger summary
    let ledger_url = format!(
        "https://portal.withorb.com/api/v1/customers/{}/ledger_summary?pricing_unit_id={}&token={}",
        customer_id, pricing_unit_id, token
    );

    let ledger_response = client
        .get(&ledger_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
        .map_err(|e| format!("Failed to get ledger info: {}", e))?;

    if !ledger_response.status().is_success() {
        return Err(format!("Ledger API request failed: {}", ledger_response.status()));
    }

    let ledger_text = ledger_response.text().await
        .map_err(|e| format!("Failed to read ledger response: {}", e))?;

    let ledger_data: serde_json::Value = serde_json::from_str(&ledger_text)
        .map_err(|e| format!("Failed to parse ledger response: {}", e))?;

    // 解析Portal信息
    let credits_balance: i32 = ledger_data["credits_balance"].as_str()
        .and_then(|s| s.parse::<f64>().ok())
        .map(|v| v.floor() as i32)
        .unwrap_or(0);

    let mut expiry_date = None;

    if let Some(credit_blocks) = ledger_data["credit_blocks"].as_array() {
        if let Some(first_block) = credit_blocks.first() {
            // 获取 effective_date 并加 1 个月
            if let Some(effective_date_str) = first_block["effective_date"].as_str() {
                // 尝试解析日期
                if let Ok(effective_date) = chrono::DateTime::parse_from_rfc3339(effective_date_str) {
                    // 加 1 个月（精确月份）
                    let expiry = effective_date.with_timezone(&chrono::Utc) + chrono::Months::new(1);
                    expiry_date = Some(expiry.to_rfc3339());
                }
            }
        }
    }

    Ok(PortalInfo {
        credits_balance,
        expiry_date,
    })
}

// 批量检测账号状态
pub async fn batch_check_account_status(
    tokens: Vec<TokenInfo>,
) -> Result<Vec<TokenStatusResult>, String> {

    // 创建并发任务并立即spawn
    let mut handles = Vec::new();

    for token_info in tokens {
        let mut token = token_info.access_token.clone();
        let mut tenant_url = token_info.tenant_url.clone();
        let token_id = token_info.id.clone();
        let portal_url = token_info.portal_url.clone();
        let auth_session = token_info.auth_session.clone();

        let handle = tokio::spawn(async move {
            println!("Checking status for token: {:?}", token_id);

            // 1. 先检测账号封禁状态
            let status_result = check_account_ban_status(&token, &tenant_url).await;

            // 处理账号状态检测结果
            let mut status_result = match status_result {
                Ok(status) => status,
                Err(err) => {
                    // 如果出错，创建一个错误状态并直接返回
                    let error_status = AccountStatus {
                        is_banned: false,
                        status: "ERROR".to_string(),
                        error_message: Some(format!("Failed to check status: {}", err)),
                        response_code: None,
                        debug_info: DebugInfo {
                            request_url: format!("{}find-missing", tenant_url),
                            request_headers: HashMap::new(),
                            request_body: "{}".to_string(),
                            response_headers: HashMap::new(),
                            response_body: format!("Error: {}", err),
                            response_status_text: "Error".to_string(),
                        },
                    };

                    return TokenStatusResult {
                        token_id,
                        access_token: token,
                        tenant_url,
                        portal_url,
                        status_result: error_status,
                        portal_info: None,
                        portal_error: Some(format!("Status check failed: {}", err)),
                        suspensions: None,
                        email_note: None,
                    };
                }
            };

            // 2. 如果检测到 INVALID_TOKEN 且有 auth_session，尝试自动刷新
            if status_result.status == "INVALID_TOKEN" {
                if let Some(ref session) = auth_session {
                    println!("Detected INVALID_TOKEN for {:?}, attempting auto-refresh with auth_session", token_id);

                    match extract_token_from_session(session).await {
                        Ok(new_token_response) => {
                            println!("Successfully refreshed token for {:?}", token_id);
                            // 更新 token 和 tenant_url
                            token = new_token_response.access_token;
                            tenant_url = new_token_response.tenant_url;

                            // Session 导入不再获取 portal_url，保持原值
                            // portal_url 保持不变

                            // 重新检测状态
                            match check_account_ban_status(&token, &tenant_url).await {
                                Ok(new_status) => {
                                    status_result = new_status;
                                    status_result.error_message = Some(format!(
                                        "Token was invalid but successfully auto-refreshed. New status: {}",
                                        status_result.status
                                    ));
                                }
                                Err(err) => {
                                    println!("Failed to check status after refresh: {}", err);
                                    status_result.error_message = Some(format!(
                                        "Token refreshed but status check failed: {}",
                                        err
                                    ));
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
                                    "Token is invalid. Auto-refresh failed: {}",
                                    err
                                ));
                            }
                        }
                    }
                } else {
                    println!("Token {:?} is invalid but no auth_session available for refresh", token_id);
                    status_result.error_message = Some(
                        "Token is invalid. No auth_session available for auto-refresh".to_string()
                    );
                }
            }

            // 3. 如果账号被封禁，尝试获取详细的用户信息
            let mut suspensions_info = None;
            if status_result.is_banned {
                // 如果有 auth_session,获取详细的封禁信息
                if let Some(ref session) = auth_session {
                    println!("Account banned for {:?}, fetching detailed user info", token_id);
                    match crate::augment_user_info::get_user_info(session).await {
                        Ok(user_info) => {
                            println!("Successfully fetched user info for banned account {:?}", token_id);
                            // 保存 suspensions 信息
                            if let Some(suspensions) = user_info.suspensions {
                                suspensions_info = Some(suspensions.clone());
                                status_result.error_message = Some(format!(
                                    "Account banned. Suspensions: {}",
                                    serde_json::to_string(&suspensions).unwrap_or_else(|_| "N/A".to_string())
                                ));
                            }
                        }
                        Err(err) => {
                            println!("Failed to fetch user info for banned account {:?}: {}", token_id, err);
                            // 不影响主流程,只记录错误
                        }
                    }
                }

                return TokenStatusResult {
                    token_id,
                    access_token: token,
                    tenant_url,
                    portal_url,
                    status_result,
                    portal_info: None,
                    portal_error: None,
                    suspensions: suspensions_info,
                    email_note: None,  // 封禁账号不获取邮箱
                };
            }

            // 4. 获取余额和过期时间信息
            // 使用 get-credit-info API
            let (portal_info, portal_error) = {
                println!("Using get-credit-info API to fetch credits and expiry for {:?}", token_id);
                match get_credit_info(&token, &tenant_url).await {
                    Ok(credit_info) => {
                        let portal_info = PortalInfo {
                            credits_balance: credit_info.usage_units_remaining.floor() as i32,  // 转换为整数
                            expiry_date: Some(credit_info.current_billing_cycle_end_date_iso),
                        };
                        (Some(portal_info), None)
                    }
                    Err(err) => {
                        println!("Failed to get credit info: {}", err);
                        (None, Some(err))
                    }
                }
            };

            // 5. 如果没有邮箱备注,尝试获取邮箱
            let email_note = if token_info.email_note.is_none() {
                match get_models(&token, &tenant_url).await {
                    Ok(models_response) => {
                        println!("Successfully got email for token {:?}: {}", token_id, models_response.user.email);
                        Some(models_response.user.email)
                    }
                    Err(err) => {
                        println!("Failed to get email for token {:?}: {}", token_id, err);
                        None
                    }
                }
            } else {
                token_info.email_note.clone()
            };

            TokenStatusResult {
                token_id,
                access_token: token,
                tenant_url,
                portal_url,
                status_result,
                portal_info,
                portal_error,
                suspensions: None,  // 正常情况下不需要 suspensions
                email_note,
            }
        });

        handles.push(handle);
    }

    let mut results = Vec::new();
    for (index, handle) in handles.into_iter().enumerate() {
        match handle.await {
            Ok(result) => results.push(result),
            Err(err) => {
                eprintln!("Task {} failed: {}", index, err);
                // 创建一个错误状态的结果
                results.push(TokenStatusResult {
                    token_id: Some(format!("task_{}", index)),
                    access_token: "".to_string(),
                    tenant_url: "".to_string(),
                    portal_url: None,
                    status_result: AccountStatus {
                        is_banned: false,
                        status: "ERROR".to_string(),
                        error_message: Some(format!("Task execution failed: {}", err)),
                        response_code: None,
                        debug_info: DebugInfo {
                            request_url: "".to_string(),
                            request_headers: HashMap::new(),
                            request_body: "{}".to_string(),
                            response_headers: HashMap::new(),
                            response_body: format!("Task Error: {}", err),
                            response_status_text: "Error".to_string(),
                        },
                    },
                    portal_info: None,
                    portal_error: Some(format!("Task failed: {}", err)),
                    suspensions: None,
                    email_note: None,
                });
            }
        }
    }

    Ok(results)
}

/// 获取 Credit 信息 (get-credit-info API)
pub async fn get_credit_info(
    token: &str,
    tenant_url: &str,
) -> Result<CreditInfoResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    // Ensure tenant_url ends with a slash
    let base_url = if tenant_url.ends_with('/') {
        tenant_url.to_string()
    } else {
        format!("{}/", tenant_url)
    };

    let api_url = format!("{}get-credit-info", base_url);

    println!("=== get-credit-info API Request ===");
    println!("URL: {}", api_url);

    // Send request to get-credit-info API
    let response = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&serde_json::json!({}))
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let status_code = response.status().as_u16();
    println!("Response Status: {}", status_code);

    if status_code != 200 {
        return Err(format!("get-credit-info API returned status {}", status_code));
    }

    let credit_info: CreditInfoResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse credit info response: {}", e))?;

    println!("Credit Info: balance={}, expiry={}",
        credit_info.usage_units_remaining,
        credit_info.current_billing_cycle_end_date_iso);

    Ok(credit_info)
}

/// 获取用户模型信息 (get-models API)
pub async fn get_models(
    token: &str,
    tenant_url: &str,
) -> Result<ModelsResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    // Ensure tenant_url ends with a slash
    let base_url = if tenant_url.ends_with('/') {
        tenant_url.to_string()
    } else {
        format!("{}/", tenant_url)
    };

    let api_url = format!("{}get-models", base_url);

    println!("=== get-models API Request ===");
    println!("URL: {}", api_url);

    // Send request to get-models API
    let response = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&serde_json::json!({}))
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let status_code = response.status().as_u16();
    println!("Response Status: {}", status_code);

    if status_code != 200 {
        return Err(format!("get-models API returned status {}", status_code));
    }

    let models_response: ModelsResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse models response: {}", e))?;

    println!("User Email: {}", models_response.user.email);

    Ok(models_response)
}