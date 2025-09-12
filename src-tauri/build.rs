fn main() {
    // 设置构建时间环境变量
    println!("cargo:rustc-env=BUILD_DATE={}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));

    // 尝试获取 Git 提交哈希
    if let Ok(output) = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        if output.status.success() {
            let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("cargo:rustc-env=GIT_HASH={}", git_hash);
        }
    }

    tauri_build::build()
}
