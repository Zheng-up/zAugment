#!/bin/bash

# ZAugment 发布脚本
# 用法: ./scripts/release.sh [版本号]
# 例如: ./scripts/release.sh 0.3.1

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查是否提供了版本号
if [ $# -eq 0 ]; then
    print_error "请提供版本号"
    echo "用法: $0 <版本号>"
    echo "例如: $0 0.3.1"
    exit 1
fi

VERSION=$1

# 验证版本号格式 (简单的语义化版本检查)
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "版本号格式不正确，请使用语义化版本 (例如: 1.0.0)"
    exit 1
fi

print_info "准备发布版本: $VERSION"

# 检查是否在 git 仓库中
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "当前目录不是 git 仓库"
    exit 1
fi

# 检查工作目录是否干净
if ! git diff-index --quiet HEAD --; then
    print_warning "工作目录有未提交的更改"
    read -p "是否继续? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "发布已取消"
        exit 1
    fi
fi

# 更新版本号
print_info "更新版本号到 $VERSION..."

# 更新 package.json
if command -v jq > /dev/null; then
    jq ".version = \"$VERSION\"" package.json > package.json.tmp && mv package.json.tmp package.json
    print_success "已更新 package.json"
else
    print_warning "未找到 jq，请手动更新 package.json 中的版本号"
fi

# 更新 Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" src-tauri/Cargo.toml
rm -f src-tauri/Cargo.toml.bak
print_success "已更新 src-tauri/Cargo.toml"

# 更新 tauri.conf.json
if command -v jq > /dev/null; then
    jq ".version = \"$VERSION\"" src-tauri/tauri.conf.json > src-tauri/tauri.conf.json.tmp && mv src-tauri/tauri.conf.json.tmp src-tauri/tauri.conf.json
    print_success "已更新 src-tauri/tauri.conf.json"
else
    print_warning "未找到 jq，请手动更新 src-tauri/tauri.conf.json 中的版本号"
fi

# 提交更改
print_info "提交版本更新..."
git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json
git commit -m "chore: bump version to $VERSION"

# 创建标签
print_info "创建标签 v$VERSION..."
git tag -a "v$VERSION" -m "Release version $VERSION"

# 推送更改和标签
print_info "推送到远程仓库..."
git push github master
git push github "v$VERSION"

print_success "版本 $VERSION 发布完成！"
print_info "GitHub Actions 将自动开始构建，请访问 GitHub Actions 页面查看进度"
print_info "构建完成后，Release 将自动创建并包含所有平台的安装包"
