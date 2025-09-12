use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use imap::Session;
use native_tls::TlsStream;
use std::net::TcpStream;
use chrono;

// XOAUTH2 认证器
struct XOAuth2 {
    user: String,
    access_token: String,
}

impl imap::Authenticator for XOAuth2 {
    type Response = String;

    fn process(&self, _data: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        )
    }
}

// 数据模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlookCredentials {
    pub email: String,
    pub refresh_token: String,
    pub client_id: String,
    pub status: Option<String>, // 保存账户状态："active", "inactive", "error", "unknown"
    pub last_checked: Option<String>, // 最后检查时间
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailItem {
    pub message_id: String,
    pub folder: String,
    pub subject: String,
    pub from_email: String,
    pub date: String,
    pub is_read: bool,
    pub has_attachments: bool,
    pub sender_initial: String,
    pub verification_code: Option<String>, // 提取的验证码
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailListResponse {
    pub email_id: String,
    pub folder_view: String,
    pub page: i32,
    pub page_size: i32,
    pub total_emails: i32,
    pub emails: Vec<EmailItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailDetailsResponse {
    pub message_id: String,
    pub subject: String,
    pub from_email: String,
    pub to_email: String,
    pub date: String,
    pub body_plain: Option<String>,
    pub body_html: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStatus {
    pub email: String,
    pub status: String, // "active", "inactive", "unknown"
}

// OAuth2 令牌响应
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
}

// 邮件管理器 - 支持持久化存储
pub struct OutlookManager {
    credentials: HashMap<String, OutlookCredentials>,
    storage_path: Option<PathBuf>,
}

impl OutlookManager {
    pub fn new() -> Self {
        Self {
            credentials: HashMap::new(),
            storage_path: None,
        }
    }

    // 设置存储路径
    pub fn set_storage_path(&mut self, _path: PathBuf) {
        // 持久化已禁用：忽略设置
        self.storage_path = None;
    }

    // 从文件加载所有凭证（禁用：仅会话内存）
    pub async fn load_from_file(&mut self) -> Result<(), String> {
        // 启动时保持空列表，不读取任何文件
        self.credentials.clear();
        Ok(())
    }

    // 保存所有凭证到文件（禁用：仅会话内存）
    pub async fn save_to_file(&self) -> Result<(), String> {
        // 跳过文件系统写入
        Ok(())
    }

    // 保存账户凭证（仅会话内存，不持久化）
    pub fn save_credentials(&mut self, credentials: OutlookCredentials) -> Result<(), String> {
        self.credentials.insert(credentials.email.clone(), credentials);
        Ok(())
    }

    // 更新账户状态（仅会话内存，不持久化）
    pub fn update_account_status(&mut self, email: &str, status: &str) -> Result<(), String> {
        if let Some(credentials) = self.credentials.get_mut(email) {
            credentials.status = Some(status.to_string());
            credentials.last_checked = Some(chrono::Utc::now().to_rfc3339());
        }
        Ok(())
    }

    // 获取账户状态
    pub fn get_account_status(&self, email: &str) -> Option<String> {
        self.credentials.get(email)
            .and_then(|creds| creds.status.clone())
    }

    // 获取所有账户及其状态
    pub fn get_all_accounts_with_status(&self) -> Result<Vec<(String, Option<String>)>, String> {
        let accounts_with_status = self.credentials.iter()
            .map(|(email, creds)| (email.clone(), creds.status.clone()))
            .collect();
        Ok(accounts_with_status)
    }

    // 获取账户凭证
    pub fn get_credentials(&self, email: &str) -> Result<OutlookCredentials, String> {
        self.credentials.get(email)
            .cloned()
            .ok_or_else(|| format!("Account not found: {}", email))
    }

    // 获取所有账户
    pub fn get_all_accounts(&self) -> Result<Vec<String>, String> {
        Ok(self.credentials.keys().cloned().collect())
    }

    // 删除账户（仅会话内存，不持久化）
    pub fn delete_account(&mut self, email: &str) -> Result<bool, String> {
        let removed = self.credentials.remove(email).is_some();
        Ok(removed)
    }

    // 获取访问令牌（每次重新获取）
    pub async fn get_access_token(&self, credentials: &OutlookCredentials) -> Result<String, String> {
        println!("🔑 获取短期 Access Token for {}", credentials.email);
        
        let token_url = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
        let params = [
            ("client_id", credentials.client_id.as_str()),
            ("grant_type", "refresh_token"),
            ("refresh_token", credentials.refresh_token.as_str()),
            ("scope", "https://outlook.office.com/IMAP.AccessAsUser.All offline_access"),
        ];

        let client = reqwest::Client::new();
        let response = client
            .post(token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "无法读取错误响应".to_string());
            return Err(format!("Token request failed: {} - {}", status, error_text));
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse token response: {}", e))?;

        println!("✅ 成功获取 Access Token for {} (有效期: {}秒)", credentials.email, token_response.expires_in);
        Ok(token_response.access_token)
    }

    // 验证账户状态
    pub async fn check_account_status(&self, email: &str) -> Result<AccountStatus, String> {
        let credentials = self.get_credentials(email)?;
        self.check_account_status_with_credentials(&credentials).await
    }

    // 使用凭证验证账户状态（避免跨 await 持有锁）
    pub async fn check_account_status_with_credentials(&self, credentials: &OutlookCredentials) -> Result<AccountStatus, String> {
        match self.get_access_token(credentials).await {
            Ok(_) => Ok(AccountStatus {
                email: credentials.email.clone(),
                status: "active".to_string(),
            }),
            Err(_) => Ok(AccountStatus {
                email: credentials.email.clone(),
                status: "inactive".to_string(),
            }),
        }
    }

    // 创建 IMAP 连接（每次新建）
    async fn create_imap_connection(&self, credentials: &OutlookCredentials) -> Result<Session<TlsStream<TcpStream>>, String> {
        println!("🔌 开始创建 IMAP 连接 for {}", credentials.email);
        
        // 每次都重新获取 Access Token
        let access_token = self.get_access_token(credentials).await?;

        // 在异步上下文中运行同步IMAP代码
        let email = credentials.email.clone();
        tokio::task::spawn_blocking(move || {
            println!("🌐 连接到 outlook.office365.com:993...");
            
            let tls = native_tls::TlsConnector::builder().build()
                .map_err(|e| format!("TLS connector failed: {}", e))?;

            let client = imap::connect(("outlook.office365.com", 993), "outlook.office365.com", &tls)
                .map_err(|e| format!("IMAP connect failed: {}", e))?;

            println!("🔐 开始 XOAUTH2 认证...");
            // XOAUTH2 认证
            let auth = XOAuth2 {
                user: email,
                access_token,
            };
            let session = client
                .authenticate("XOAUTH2", &auth)
                .map_err(|e| format!("IMAP authentication failed: {:?}", e))?;

            println!("✅ IMAP 连接和认证成功");
            Ok(session)
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }

    // 获取邮件详情
    pub async fn get_email_details(&self, email: &str, message_id: &str) -> Result<EmailDetailsResponse, String> {
        let credentials = self.get_credentials(email)?;
        self.get_email_details_with_credentials(&credentials, message_id).await
    }

    // 使用凭证获取邮件详情（避免跨 await 持有锁）
    pub async fn get_email_details_with_credentials(&self, credentials: &OutlookCredentials, message_id: &str) -> Result<EmailDetailsResponse, String> {
        let access_token = self.get_access_token(credentials).await?;

        // 解析 message_id (格式: folder-id)
        let parts: Vec<&str> = message_id.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid message_id format".to_string());
        }
        let folder_name = parts[0].to_string();
        let msg_id = parts[1].to_string();

        let email_clone = credentials.email.clone();
        let message_id_clone = message_id.to_string();

        tokio::task::spawn_blocking(move || {
            let tls = native_tls::TlsConnector::builder().build()
                .map_err(|e| format!("TLS connector failed: {}", e))?;

            let client = imap::connect(("outlook.office365.com", 993), "outlook.office365.com", &tls)
                .map_err(|e| format!("IMAP connect failed: {}", e))?;

            let auth = XOAuth2 {
                user: email_clone.clone(),
                access_token,
            };
            let mut session = client
                .authenticate("XOAUTH2", &auth)
                .map_err(|e| format!("IMAP authentication failed: {:?}", e))?;

            session.select(&folder_name)
                .map_err(|e| format!("Failed to select folder: {:?}", e))?;

            // 获取完整邮件内容
            let messages = session.fetch(&msg_id, "RFC822")
                .map_err(|e| format!("Failed to fetch message: {:?}", e))?;

            if let Some(message) = messages.iter().next() {
                let body = message.body()
                    .ok_or("No message body found")?;

                // 解析邮件
                let parsed = std::str::from_utf8(body)
                    .map_err(|e| format!("Failed to parse email: {}", e))?;

                // 解析邮件头部和正文
                let (headers, body_content) = Self::parse_email_content(parsed)?;

                let subject = headers.get("Subject").cloned().unwrap_or_else(|| "(No Subject)".to_string());
                let from_email = headers.get("From").cloned().unwrap_or_else(|| "(Unknown Sender)".to_string());
                let to_email = headers.get("To").cloned().unwrap_or_else(|| "(Unknown Recipient)".to_string());
                let date = headers.get("Date").cloned().unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

                // 解析邮件正文
                let (body_plain, body_html) = Self::extract_email_body(&body_content)?;

                session.logout().ok();

                Ok(EmailDetailsResponse {
                    message_id: message_id_clone,
                    subject,
                    from_email,
                    to_email,
                    date,
                    body_plain,
                    body_html,
                })
            } else {
                Err("Message not found".to_string())
            }
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }

    // 获取邮件列表
    pub async fn get_emails(&self, email: &str, folder: &str, page: i32, page_size: i32) -> Result<EmailListResponse, String> {
        let credentials = self.get_credentials(email)?;
        self.get_emails_with_credentials(&credentials, folder, page, page_size).await
    }

    // 使用凭证获取邮件列表（避免跨 await 持有锁）
    pub async fn get_emails_with_credentials(&self, credentials: &OutlookCredentials, folder: &str, page: i32, page_size: i32) -> Result<EmailListResponse, String> {
        println!("📧 准备获取邮件 for {} - 文件夹: {}", credentials.email, folder);
        
        // 每次都重新创建连接（包含获取新的 Access Token）
        let mut session = self.create_imap_connection(credentials).await?;

        let folder_name = match folder {
            "inbox" => "INBOX",
            "junk" => "Junk",
            _ => "INBOX",
        };

        // 在异步上下文中运行同步IMAP代码
        let email_clone = credentials.email.clone();
        let folder_clone = folder.to_string();

        tokio::task::spawn_blocking(move || {
            println!("📂 选择文件夹: {}", folder_name);
            session.select(folder_name)
                .map_err(|e| format!("Failed to select folder: {:?}", e))?;

            println!("🔍 搜索邮件...");
            let messages = session.search("ALL")
                .map_err(|e| format!("Failed to search messages: {:?}", e))?;

            let mut message_vec: Vec<u32> = messages.into_iter().collect();
            // 按消息ID倒序排列，确保最新邮件在前
            message_vec.sort_by(|a, b| b.cmp(a));

            let total_emails = message_vec.len() as i32;
            let start_idx = ((page - 1) * page_size) as usize;
            let end_idx = std::cmp::min(start_idx + page_size as usize, message_vec.len());

            println!("📊 找到 {} 封邮件，显示第 {} 页 ({}-{})", total_emails, page, start_idx + 1, end_idx);

            let mut emails = Vec::new();

            if start_idx < message_vec.len() {
                let page_messages = &message_vec[start_idx..end_idx];

                for &msg_id in page_messages.iter() { // 按消息ID顺序（通常是时间倒序）
                    if let Ok(messages) = session.fetch(msg_id.to_string(), "ENVELOPE") {
                        for msg in messages.iter() {
                            if let Some(envelope) = msg.envelope() {
                                let subject = envelope.subject
                                    .and_then(|s| std::str::from_utf8(s).ok())
                                    .unwrap_or("(No Subject)")
                                    .to_string();

                                let from_email = envelope.from
                                    .as_ref()
                                    .and_then(|addrs| addrs.first())
                                    .and_then(|addr| addr.mailbox)
                                    .and_then(|mb| std::str::from_utf8(mb).ok())
                                    .unwrap_or("(Unknown)")
                                    .to_string();

                                let date = envelope.date
                                    .and_then(|d| std::str::from_utf8(d).ok())
                                    .unwrap_or("")
                                    .to_string();

                                let sender_initial = from_email.chars().next()
                                    .unwrap_or('?')
                                    .to_uppercase()
                                    .to_string();

                                // 尝试获取邮件内容来提取验证码
                                let verification_code = if let Ok(content_messages) = session.fetch(msg_id.to_string(), "BODY[TEXT]") {
                                    if let Some(content_msg) = content_messages.iter().next() {
                                        if let Some(body) = content_msg.body() {
                                            let content_text = std::str::from_utf8(body).unwrap_or("");
                                            Self::extract_verification_code(&subject, content_text)
                                        } else {
                                            Self::extract_verification_code(&subject, "")
                                        }
                                    } else {
                                        Self::extract_verification_code(&subject, "")
                                    }
                                } else {
                                    Self::extract_verification_code(&subject, "")
                                };

                                emails.push(EmailItem {
                                    message_id: format!("{}-{}", folder_name, msg_id),
                                    folder: folder_name.to_string(),
                                    subject,
                                    from_email,
                                    date,
                                    is_read: false, // 简化处理
                                    has_attachments: false, // 简化处理
                                    sender_initial,
                                    verification_code,
                                });
                            }
                        }
                    }
                }
            }

            session.logout().ok();
            println!("✅ 成功获取 {} 封邮件", emails.len());

            Ok(EmailListResponse {
                email_id: email_clone,
                folder_view: folder_clone,
                page,
                page_size,
                total_emails,
                emails,
            })
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }

    // 解析邮件头部和正文
    fn parse_email_content(email_content: &str) -> Result<(HashMap<String, String>, String), String> {
        let mut headers = HashMap::new();
        let mut body = String::new();
        let mut in_headers = true;
        let mut current_header = String::new();
        let mut current_value = String::new();

        for line in email_content.lines() {
            if in_headers {
                if line.is_empty() {
                    // 保存最后一个头部
                    if !current_header.is_empty() {
                        headers.insert(current_header.clone(), Self::decode_header_value(&current_value));
                    }
                    in_headers = false;
                    continue;
                }

                if line.starts_with(' ') || line.starts_with('\t') {
                    // 续行
                    current_value.push(' ');
                    current_value.push_str(line.trim());
                } else if let Some(colon_pos) = line.find(':') {
                    // 保存上一个头部
                    if !current_header.is_empty() {
                        headers.insert(current_header.clone(), Self::decode_header_value(&current_value));
                    }
                    // 开始新头部
                    current_header = line[..colon_pos].to_string();
                    current_value = line[colon_pos + 1..].trim().to_string();
                }
            } else {
                body.push_str(line);
                body.push('\n');
            }
        }

        Ok((headers, body))
    }

    // 解码邮件头部值
    fn decode_header_value(value: &str) -> String {
        // 简单的 RFC 2047 解码
        if value.contains("=?") && value.contains("?=") {
            // 这里可以实现更复杂的编码解码，现在简化处理
            value.replace("=?UTF-8?B?", "").replace("?=", "")
        } else {
            value.to_string()
        }
    }

    // 提取邮件正文
    fn extract_email_body(body_content: &str) -> Result<(Option<String>, Option<String>), String> {
        let mut body_plain = None;
        let mut body_html = None;

        // 检查是否是多部分邮件
        if body_content.contains("multipart/") && body_content.contains("boundary") {
            // 查找边界
            let boundary = Self::find_boundary(body_content);
            if let Some(boundary_str) = boundary {
                let boundary_marker = format!("--{}", boundary_str);
                let parts: Vec<&str> = body_content.split(&boundary_marker).collect();

                for part in parts {
                    if part.trim().is_empty() {
                        continue;
                    }

                    if part.contains("text/plain") {
                        if let Some(content) = Self::extract_part_content(part) {
                            body_plain = Some(content);
                        }
                    } else if part.contains("text/html") {
                        if let Some(content) = Self::extract_part_content(part) {
                            body_html = Some(content);
                        }
                    }
                }
            }
        } else {
            // 单部分邮件 - 直接提取内容
            let cleaned_content = Self::extract_simple_body(body_content);
            if !cleaned_content.trim().is_empty() {
                if body_content.contains("text/html") {
                    body_html = Some(cleaned_content);
                } else {
                    body_plain = Some(cleaned_content);
                }
            }
        }

        // 如果没有找到任何内容，尝试提取所有可见文本
        if body_plain.is_none() && body_html.is_none() {
            let fallback_content = Self::extract_fallback_content(body_content);
            if !fallback_content.trim().is_empty() {
                body_plain = Some(fallback_content);
            }
        }

        Ok((body_plain, body_html))
    }

    // 查找边界字符串
    fn find_boundary(content: &str) -> Option<String> {
        // 查找 boundary= 后面的值
        if let Some(start) = content.find("boundary=") {
            let after_boundary = &content[start + 9..];
            let boundary_line = after_boundary.lines().next().unwrap_or("");

            // 移除引号和分号
            let boundary = boundary_line
                .split(';')
                .next()
                .unwrap_or("")
                .trim()
                .trim_matches('"')
                .trim_matches('\'');

            if !boundary.is_empty() {
                Some(boundary.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    // 提取部分内容
    fn extract_part_content(part: &str) -> Option<String> {
        let lines: Vec<&str> = part.lines().collect();
        let mut content_start = 0;
        let in_headers = true;

        // 找到空行，表示头部结束
        for (i, line) in lines.iter().enumerate() {
            if in_headers && line.trim().is_empty() {
                content_start = i + 1;
                break;
            }
        }

        if content_start < lines.len() {
            let content_lines = &lines[content_start..];
            let content = content_lines
                .iter()
                .filter(|line| !line.starts_with("--")) // 过滤边界标记
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join("\n");

            if !content.trim().is_empty() {
                Some(Self::decode_content(&content))
            } else {
                None
            }
        } else {
            None
        }
    }

    // 提取简单邮件正文
    fn extract_simple_body(content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut body_start = 0;

        // 跳过头部，找到第一个空行后的内容
        for (i, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                body_start = i + 1;
                break;
            }
        }

        if body_start < lines.len() {
            let body_lines = &lines[body_start..];
            let content = body_lines
                .iter()
                .filter(|line| !line.starts_with("Content-"))
                .filter(|line| !line.starts_with("MIME-"))
                .filter(|line| !line.starts_with("--"))
                .map(|line| *line)
                .collect::<Vec<_>>()
                .join("\n");

            Self::decode_content(&content)
        } else {
            String::new()
        }
    }

    // 提取备用内容（当其他方法都失败时）
    fn extract_fallback_content(content: &str) -> String {
        // 简单地提取所有看起来像正文的内容
        content
            .lines()
            .skip_while(|line| {
                line.starts_with("Content-") ||
                line.starts_with("MIME-") ||
                line.starts_with("Date:") ||
                line.starts_with("From:") ||
                line.starts_with("To:") ||
                line.starts_with("Subject:") ||
                line.starts_with("Message-ID:") ||
                line.contains("boundary=") ||
                line.trim().is_empty()
            })
            .filter(|line| {
                !line.starts_with("--") &&
                !line.starts_with("Content-") &&
                !line.starts_with("MIME-") &&
                !line.trim().is_empty()
            })
            .take(50) // 限制行数，避免过长
            .collect::<Vec<_>>()
            .join("\n")
    }

    // 解码内容
    fn decode_content(content: &str) -> String {
        // 处理 Quoted-Printable 编码
        if content.contains('=') {
            let decoded = content
                .replace("=\n", "")
                .replace("=20", " ")
                .replace("=3D", "=")
                .replace("=0A", "\n")
                .replace("=0D", "\r");
            return decoded;
        }

        // 限制长度
        if content.len() > 5000 {
            let mut truncated = content.chars().take(5000).collect::<String>();
            truncated.push_str("\n\n[内容已截断...]");
            truncated
        } else {
            content.to_string()
        }
    }

    // 提取验证码 - 优化Augment Code格式
    fn extract_verification_code(subject: &str, content: &str) -> Option<String> {
        // 清理HTML标签和实体
        let cleaned_content = content
            .replace(r"<[^>]+>", " ")  // 移除HTML标签
            .replace(r"&nbsp;", " ")   // 移除HTML实体
            .replace(r"&[a-zA-Z0-9]+;", " ")  // 移除所有HTML实体
            .replace("\n", " ")
            .replace("\r", " ")
            .replace("\t", " ");
        
        let combined_text = format!("{} {}", subject, cleaned_content);
        println!("🔍 验证码提取 - 清理后的文本: {}", combined_text);
        
        // 针对Augment Code和常见验证码格式的优化正则
        let patterns = [
            // Augment Code特定格式："Your verification code is: 554775"
            r"(?i)your\s+verification\s+code\s+is\s*[:\-]?\s*(\d{4,8})",
            // 通用格式："verification code is: 123456"
            r"(?i)verification\s+code\s+is\s*[:\-]?\s*(\d{4,8})",
            // 简化格式："code is: 123456"
            r"(?i)code\s+is\s*[:\-]?\s*(\d{4,8})",
            // 冒号格式："code: 123456"
            r"(?i)(?:code|verification code)\s*:\s*(\d{4,8})",
            // 更宽泛的匹配："verification code" 后面跟数字
            r"(?i)verification\s+code[^0-9]*?(\d{4,8})",
            // "Your code" 格式
            r"(?i)your\s+code\s*[:\-]?\s*(\d{4,8})",
            // 中文格式
            r"(?i)验证码\s*[：:\-]?\s*(\d{4,8})",
        ];

        // 用于存储找到的所有验证码候选
        let mut candidates: Vec<String> = Vec::new();

        for pattern in &patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                for captures in regex.captures_iter(&combined_text) {
                    if let Some(code_match) = captures.get(1) {
                        let code = code_match.as_str();
                        // 验证码长度检查：4-8位
                        if code.len() >= 4 && code.len() <= 8 {
                            let code_num: u32 = code.parse().unwrap_or(0);
                            // 排除明显不是验证码的数字（如年份、过小的数）
                            if code_num >= 1000 && !(1900..=2100).contains(&code_num) {
                                candidates.push(code.to_string());
                                println!("🔢 找到验证码候选: {}", code);
                            }
                        }
                    }
                }
            }
        }

        // 优先返回6位数字（最常见的验证码长度）
        for candidate in &candidates {
            if candidate.len() == 6 {
                println!("✅ 选择6位验证码: {}", candidate);
                return Some(candidate.clone());
            }
        }

        // 其次返回第一个找到的4-8位验证码
        if let Some(first) = candidates.first() {
            println!("✅ 选择验证码: {}", first);
            return Some(first.clone());
        }

        println!("❌ 未找到验证码");
        None
    }
}