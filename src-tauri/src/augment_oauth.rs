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

    Ok(AugmentTokenResponse {
        access_token: token,
        tenant_url: parsed_code.tenant_url,
    })
}

/// Check account ban status by testing chat-stream API
pub async fn check_account_ban_status(
    token: &str,
    tenant_url: &str,
) -> Result<AccountStatus, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // Prepare test chat request
    let chat_request = ChatRequest {
        chat_history: vec![ChatMessage {
            response_text: "你好 Cube! 我是 Augment，很高兴为你提供帮助。".to_string(),
            request_message: "你好，我是Cube".to_string(),
        }],
        message: "我叫什么名字".to_string(),
        mode: "CHAT".to_string(),
    };

    // Ensure tenant_url ends with a slash
    let base_url = if tenant_url.ends_with('/') {
        tenant_url.to_string()
    } else {
        format!("{}/", tenant_url)
    };

    let chat_url = format!("{}chat-stream", base_url);

    // Serialize request body for debugging
    let request_body = serde_json::to_string_pretty(&chat_request)?;

    // Prepare request headers for debugging
    let mut request_headers = HashMap::new();
    request_headers.insert("Content-Type".to_string(), "application/json".to_string());
    request_headers.insert("Authorization".to_string(), format!("Bearer {}", token));

    // Print debug info to console
    println!("=== API Request Debug Info ===");
    println!("URL: {}", chat_url);
    println!("Method: POST");
    println!("Headers: {:#?}", request_headers);
    println!("Request Body: {}", request_body);
    println!("==============================");

    // Send request to chat-stream API
    let response = client
        .post(&chat_url)
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&chat_request)
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
        request_url: chat_url,
        request_headers,
        request_body,
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

    Ok(AugmentTokenResponse {
        access_token: token_data.access_token,
        tenant_url: tenant_url.to_string(),
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