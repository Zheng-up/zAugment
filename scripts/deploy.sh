#!/bin/bash

# ZAugment 部署脚本
# 用于推送代码并触发 GitHub Actions 构建

VERSION=${1:-"v0.1.1"}
MESSAGE=${2:-"优化构建配置和文件命名"}

echo "🚀 ZAugment 部署脚本"
echo "==================="
echo "版本: $VERSION"
echo "提交信息: $MESSAGE"
echo ""

# 检查 git 状态
echo "📋 检查 Git 状态..."
if ! git status &>/dev/null; then
    echo "❌ 不是 Git 仓库或 Git 未安装"
    exit 1
fi

GIT_STATUS=$(git status --porcelain)
if [ -n "$GIT_STATUS" ]; then
    echo "发现未提交的更改:"
    git status --short
    echo ""
else
    echo "✅ 工作目录干净"
fi

# 检查远程仓库
echo "🔗 检查远程仓库..."
if ! git remote get-url origin &>/dev/null; then
    echo "❌ 未找到远程仓库 origin"
    exit 1
fi

echo "✅ 远程仓库已配置"
git remote -v

# 添加所有更改
if [ -n "$GIT_STATUS" ]; then
    echo "📦 添加所有更改..."
    git add .
    echo "✅ 更改已添加"
fi

# 提交更改
if [ -n "$GIT_STATUS" ]; then
    echo "💾 提交更改..."
    if git commit -m "$MESSAGE"; then
        echo "✅ 更改已提交"
    else
        echo "❌ 提交失败"
        exit 1
    fi
else
    echo "ℹ️ 没有新的更改需要提交"
fi

# 推送到远程仓库
echo "🚀 推送到远程仓库..."
if git push origin master; then
    echo "✅ 代码已推送到 master 分支"
else
    echo "❌ 推送失败，请检查网络连接和权限"
    echo "可能需要配置 SSH 密钥或使用 HTTPS"
    exit 1
fi

# 检查标签是否已存在
echo "🏷️ 检查标签..."
if git tag -l | grep -q "^$VERSION$"; then
    echo "⚠️ 标签 $VERSION 已存在，将删除并重新创建"
    git tag -d "$VERSION"
    git push origin --delete "$VERSION" 2>/dev/null || true
fi

# 创建标签
echo "🏷️ 创建标签 $VERSION..."
if git tag "$VERSION"; then
    echo "✅ 标签已创建"
else
    echo "❌ 标签创建失败"
    exit 1
fi

# 推送标签
echo "🚀 推送标签到远程仓库..."
if git push origin "$VERSION"; then
    echo "✅ 标签已推送"
else
    echo "❌ 标签推送失败"
    exit 1
fi

echo ""
echo "🎉 部署完成！"
echo ""
echo "📊 接下来的步骤:"
echo "1. 访问 GitHub Actions 页面查看构建状态:"
echo "   https://github.com/Zheng-up/zAugment/actions"
echo ""
echo "2. 构建完成后，在 Releases 页面查看发布:"
echo "   https://github.com/Zheng-up/zAugment/releases"
echo ""
echo "3. 如果构建失败，检查 Actions 日志并修复问题"
echo ""
echo "⏱️ 预计构建时间: 15-20 分钟"
