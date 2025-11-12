use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use urlencoding;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: Option<String>,
    pub suspensions: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionInfo {
    #[serde(rename = "portalUrl")]
    pub portal_url: Option<String>,
    #[serde(rename = "billingPeriodEnd")]
    pub billing_period_end: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditsInfo {
    #[serde(rename = "usageUnitsAvailable")]
    pub usage_units_available: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteUserInfo {
    pub suspensions: Option<Value>,
    pub ban_status: String,
}

/// 通过 auth session 交换 app session
pub async fn exchange_auth_session_for_app_session(auth_session: &str) -> Result<String, String> {
    use reqwest::cookie::Jar;
    use std::sync::Arc;

    // 创建 cookie jar
    let jar = Arc::new(Jar::default());

    // 设置 auth session cookie 到 auth.augmentcode.com 域
    let auth_url = "https://auth.augmentcode.com/".parse::<reqwest::Url>()
        .map_err(|e| format!("Failed to parse auth URL: {}", e))?;
    jar.add_cookie_str(
        &format!("session={}", auth_session),
        &auth_url
    );

    // 创建带 cookie store 的客户端
    let client = reqwest::Client::builder()
        .cookie_provider(jar.clone())
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    // HEAD 敲门
    let _ = client
        .head("https://app.augmentcode.com/login")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await;

    // GET 触发授权流
    let _ = client
        .get("https://app.augmentcode.com/login")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await
        .map_err(|e| format!("Failed to exchange session: {}", e))?;

    // 发送一个简单的请求来获取 app session
    let final_response = client
        .get("https://app.augmentcode.com/api/user")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await;

    // 从响应中提取 cookies
    if let Ok(resp) = final_response {
        for cookie in resp.cookies() {
            if cookie.name() == "_session" {
                return Ok(urlencoding::decode(cookie.value())
                    .unwrap_or_else(|_| cookie.value().into())
                    .to_string());
            }
        }
    }

    Err("Failed to extract app session cookie".to_string())
}

/// 获取用户信息
pub async fn fetch_app_user(app_session: &str) -> Result<UserInfo, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get("https://app.augmentcode.com/api/user")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Cookie", format!("_session={}", urlencoding::encode(app_session)))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch user info: {}", e))?;

    response
        .json::<UserInfo>()
        .await
        .map_err(|e| format!("Failed to parse user info: {}", e))
}

/// 获取订阅信息
pub async fn fetch_app_subscription(app_session: &str) -> Result<SubscriptionInfo, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get("https://app.augmentcode.com/api/subscription")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Cookie", format!("_session={}", urlencoding::encode(app_session)))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch subscription info: {}", e))?;

    response
        .json::<SubscriptionInfo>()
        .await
        .map_err(|e| format!("Failed to parse subscription info: {}", e))
}

/// 获取积分信息
pub async fn fetch_app_credits(app_session: &str) -> Result<CreditsInfo, String> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://app.augmentcode.com/api/credits")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Cookie", format!("_session={}", urlencoding::encode(app_session)))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch credits info: {}", e))?;

    response
        .json::<CreditsInfo>()
        .await
        .map_err(|e| format!("Failed to parse credits info: {}", e))
}

/// 使用已有的 app_session 获取完整的用户信息
pub async fn get_user_info_with_app_session(app_session: &str) -> Result<CompleteUserInfo, String> {
    // 只获取用户信息
    let user_info = fetch_app_user(app_session).await.ok();

    // 计算 ban_status
    let ban_status = if let Some(ref user) = user_info {
        if let Some(ref suspensions) = user.suspensions {
            if let Some(arr) = suspensions.as_array() {
                if !arr.is_empty() {
                    if let Some(first) = arr.first() {
                        if let Some(suspension_type) = first.get("suspensionType").and_then(|v| v.as_str()) {
                            format!("BANNED-{}", suspension_type)
                        } else {
                            "BANNED".to_string()
                        }
                    } else {
                        "BANNED".to_string()
                    }
                } else {
                    "ACTIVE".to_string()
                }
            } else {
                "ACTIVE".to_string()
            }
        } else {
            "ACTIVE".to_string()
        }
    } else {
        "ACTIVE".to_string()
    };

    Ok(CompleteUserInfo {
        suspensions: user_info.and_then(|u| u.suspensions),
        ban_status,
    })
}

/// 获取完整的用户信息 (使用缓存的 app_session)
pub async fn get_user_info(
    auth_session: &str,
    app_session_cache: &std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, crate::AppSessionCache>>>,
) -> Result<CompleteUserInfo, String> {
    use std::time::SystemTime;

    // 1. 检查缓存中是否有有效的 app_session
    let cached_app_session = {
        let cache = app_session_cache.lock().unwrap();
        cache.get(auth_session).map(|c| c.app_session.clone())
    };

    // 2. 尝试使用缓存的 app_session
    if let Some(app_session) = cached_app_session {
        match get_user_info_with_app_session(&app_session).await {
            Ok(user_info) => return Ok(user_info),
            Err(e) => println!("Cached app_session failed: {}, will refresh", e),
        }
    }

    // 3. 交换新的 app_session
    let app_session = exchange_auth_session_for_app_session(auth_session).await?;

    println!("App session obtained: {}", &app_session[..20.min(app_session.len())]);

    // 4. 更新缓存
    {
        let mut cache = app_session_cache.lock().unwrap();
        cache.insert(auth_session.to_string(), crate::AppSessionCache {
            app_session: app_session.clone(),
            created_at: SystemTime::now(),
        });
    }

    // 5. 获取用户信息
    get_user_info_with_app_session(&app_session).await
}

