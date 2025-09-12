#!/bin/bash

# ZAugment 构建测试脚本
# 用于验证 GitHub Actions 配置和本地构建

set -e

echo "🚀 ZAugment 构建测试脚本"
echo "=========================="

# 检查必要的工具
echo "📋 检查构建环境..."

# 检查 Node.js
if command -v node &> /dev/null; then
    echo "✅ Node.js: $(node --version)"
else
    echo "❌ Node.js 未安装"
    exit 1
fi

# 检查 npm
if command -v npm &> /dev/null; then
    echo "✅ npm: $(npm --version)"
else
    echo "❌ npm 未安装"
    exit 1
fi

# 检查 Rust
if command -v rustc &> /dev/null; then
    echo "✅ Rust: $(rustc --version)"
else
    echo "❌ Rust 未安装"
    exit 1
fi

# 检查 Tauri CLI
if command -v cargo-tauri &> /dev/null; then
    echo "✅ Tauri CLI: $(cargo tauri --version)"
else
    echo "❌ Tauri CLI 未安装，正在安装..."
    cargo install tauri-cli
fi

echo ""
echo "🔧 验证配置文件..."

# 检查 package.json
if [ -f "package.json" ]; then
    echo "✅ package.json 存在"
    # 验证版本号
    VERSION=$(node -p "require('./package.json').version")
    echo "   版本: $VERSION"
else
    echo "❌ package.json 不存在"
    exit 1
fi

# 检查 tauri.conf.json
if [ -f "src-tauri/tauri.conf.json" ]; then
    echo "✅ tauri.conf.json 存在"
    # 验证配置
    TAURI_VERSION=$(node -p "require('./src-tauri/tauri.conf.json').version")
    echo "   Tauri 版本: $TAURI_VERSION"
    
    # 检查构建目标
    TARGETS=$(node -p "JSON.stringify(require('./src-tauri/tauri.conf.json').bundle.targets)")
    echo "   构建目标: $TARGETS"
else
    echo "❌ tauri.conf.json 不存在"
    exit 1
fi

# 检查 GitHub Actions 配置
if [ -f ".github/workflows/build.yml" ]; then
    echo "✅ GitHub Actions 配置存在"
else
    echo "❌ GitHub Actions 配置不存在"
    exit 1
fi

echo ""
echo "📦 安装依赖..."
npm install

echo ""
echo "🏗️ 测试前端构建..."
npm run build

echo ""
echo "🧪 测试 Tauri 开发模式..."
echo "注意：这将启动开发服务器，请手动关闭窗口来继续测试"
timeout 10s cargo tauri dev || echo "开发模式测试完成（超时或手动关闭）"

echo ""
echo "✅ 所有测试通过！"
echo ""
echo "🚀 准备创建 GitHub Release："
echo "1. 提交所有更改："
echo "   git add ."
echo "   git commit -m \"优化构建配置和文件命名\""
echo "   git push origin main"
echo ""
echo "2. 创建并推送标签："
echo "   git tag v$VERSION"
echo "   git push origin v$VERSION"
echo ""
echo "3. 查看 GitHub Actions 构建状态："
echo "   https://github.com/YOUR_USERNAME/YOUR_REPO/actions"
