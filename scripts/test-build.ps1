# ZAugment 构建测试脚本 (PowerShell)
# 用于验证 GitHub Actions 配置和本地构建

Write-Host "🚀 ZAugment 构建测试脚本" -ForegroundColor Green
Write-Host "==========================" -ForegroundColor Green

# 检查必要的工具
Write-Host "📋 检查构建环境..." -ForegroundColor Yellow

# 检查 Node.js
try {
    $nodeVersion = node --version
    Write-Host "✅ Node.js: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Node.js 未安装" -ForegroundColor Red
    exit 1
}

# 检查 npm
try {
    $npmVersion = npm --version
    Write-Host "✅ npm: $npmVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ npm 未安装" -ForegroundColor Red
    exit 1
}

# 检查 Rust
try {
    $rustVersion = rustc --version
    Write-Host "✅ Rust: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust 未安装" -ForegroundColor Red
    exit 1
}

# 检查 Tauri CLI
try {
    $tauriVersion = cargo tauri --version
    Write-Host "✅ Tauri CLI: $tauriVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Tauri CLI 未安装，正在安装..." -ForegroundColor Yellow
    cargo install tauri-cli
}

Write-Host ""
Write-Host "🔧 验证配置文件..." -ForegroundColor Yellow

# 检查 package.json
if (Test-Path "package.json") {
    Write-Host "✅ package.json 存在" -ForegroundColor Green
    $packageJson = Get-Content "package.json" | ConvertFrom-Json
    $version = $packageJson.version
    Write-Host "   版本: $version" -ForegroundColor Cyan
} else {
    Write-Host "❌ package.json 不存在" -ForegroundColor Red
    exit 1
}

# 检查 tauri.conf.json
if (Test-Path "src-tauri/tauri.conf.json") {
    Write-Host "✅ tauri.conf.json 存在" -ForegroundColor Green
    $tauriConfig = Get-Content "src-tauri/tauri.conf.json" | ConvertFrom-Json
    $tauriVersion = $tauriConfig.version
    Write-Host "   Tauri 版本: $tauriVersion" -ForegroundColor Cyan
    
    $targets = $tauriConfig.bundle.targets -join ", "
    Write-Host "   构建目标: $targets" -ForegroundColor Cyan
} else {
    Write-Host "❌ tauri.conf.json 不存在" -ForegroundColor Red
    exit 1
}

# 检查 GitHub Actions 配置
if (Test-Path ".github/workflows/build.yml") {
    Write-Host "✅ GitHub Actions 配置存在" -ForegroundColor Green
} else {
    Write-Host "❌ GitHub Actions 配置不存在" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "📦 安装依赖..." -ForegroundColor Yellow
npm install

Write-Host ""
Write-Host "🏗️ 测试前端构建..." -ForegroundColor Yellow
npm run build

Write-Host ""
Write-Host "✅ 所有测试通过！" -ForegroundColor Green
Write-Host ""
Write-Host "🚀 准备创建 GitHub Release：" -ForegroundColor Cyan
Write-Host "1. 提交所有更改：" -ForegroundColor White
Write-Host "   git add ." -ForegroundColor Gray
Write-Host "   git commit -m `"优化构建配置和文件命名`"" -ForegroundColor Gray
Write-Host "   git push origin main" -ForegroundColor Gray
Write-Host ""
Write-Host "2. 创建并推送标签：" -ForegroundColor White
Write-Host "   git tag v$version" -ForegroundColor Gray
Write-Host "   git push origin v$version" -ForegroundColor Gray
Write-Host ""
Write-Host "3. 查看 GitHub Actions 构建状态：" -ForegroundColor White
Write-Host "   https://github.com/YOUR_USERNAME/YOUR_REPO/actions" -ForegroundColor Gray
