# ZAugment 部署脚本
# 用于推送代码并触发 GitHub Actions 构建

param(
    [string]$Version = "v0.1.1",
    [string]$Message = "优化构建配置和文件命名"
)

Write-Host "🚀 ZAugment 部署脚本" -ForegroundColor Green
Write-Host "===================" -ForegroundColor Green
Write-Host "版本: $Version" -ForegroundColor Cyan
Write-Host "提交信息: $Message" -ForegroundColor Cyan
Write-Host ""

# 检查 git 状态
Write-Host "📋 检查 Git 状态..." -ForegroundColor Yellow
try {
    $gitStatus = git status --porcelain
    if ($gitStatus) {
        Write-Host "发现未提交的更改:" -ForegroundColor Yellow
        git status --short
        Write-Host ""
    } else {
        Write-Host "✅ 工作目录干净" -ForegroundColor Green
    }
} catch {
    Write-Host "❌ Git 状态检查失败" -ForegroundColor Red
    exit 1
}

# 检查远程仓库
Write-Host "🔗 检查远程仓库..." -ForegroundColor Yellow
try {
    $remotes = git remote -v
    if ($remotes -match "origin") {
        Write-Host "✅ 远程仓库已配置" -ForegroundColor Green
        Write-Host $remotes -ForegroundColor Gray
    } else {
        Write-Host "❌ 未找到远程仓库 origin" -ForegroundColor Red
        exit 1
    }
} catch {
    Write-Host "❌ 远程仓库检查失败" -ForegroundColor Red
    exit 1
}

# 添加所有更改
if ($gitStatus) {
    Write-Host "📦 添加所有更改..." -ForegroundColor Yellow
    git add .
    Write-Host "✅ 更改已添加" -ForegroundColor Green
}

# 提交更改
if ($gitStatus) {
    Write-Host "💾 提交更改..." -ForegroundColor Yellow
    try {
        git commit -m $Message
        Write-Host "✅ 更改已提交" -ForegroundColor Green
    } catch {
        Write-Host "❌ 提交失败" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "ℹ️ 没有新的更改需要提交" -ForegroundColor Blue
}

# 推送到远程仓库
Write-Host "🚀 推送到远程仓库..." -ForegroundColor Yellow
try {
    git push origin master
    Write-Host "✅ 代码已推送到 master 分支" -ForegroundColor Green
} catch {
    Write-Host "❌ 推送失败，请检查网络连接和权限" -ForegroundColor Red
    Write-Host "可能需要配置 SSH 密钥或使用 HTTPS" -ForegroundColor Yellow
    exit 1
}

# 检查标签是否已存在
Write-Host "🏷️ 检查标签..." -ForegroundColor Yellow
$existingTag = git tag -l $Version
if ($existingTag) {
    Write-Host "⚠️ 标签 $Version 已存在，将删除并重新创建" -ForegroundColor Yellow
    git tag -d $Version
    git push origin --delete $Version 2>$null
}

# 创建标签
Write-Host "🏷️ 创建标签 $Version..." -ForegroundColor Yellow
try {
    git tag $Version
    Write-Host "✅ 标签已创建" -ForegroundColor Green
} catch {
    Write-Host "❌ 标签创建失败" -ForegroundColor Red
    exit 1
}

# 推送标签
Write-Host "🚀 推送标签到远程仓库..." -ForegroundColor Yellow
try {
    git push origin $Version
    Write-Host "✅ 标签已推送" -ForegroundColor Green
} catch {
    Write-Host "❌ 标签推送失败" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "🎉 部署完成！" -ForegroundColor Green
Write-Host ""
Write-Host "📊 接下来的步骤:" -ForegroundColor Cyan
Write-Host "1. 访问 GitHub Actions 页面查看构建状态:" -ForegroundColor White
Write-Host "   https://github.com/Zheng-up/zAugment/actions" -ForegroundColor Blue
Write-Host ""
Write-Host "2. 构建完成后，在 Releases 页面查看发布:" -ForegroundColor White
Write-Host "   https://github.com/Zheng-up/zAugment/releases" -ForegroundColor Blue
Write-Host ""
Write-Host "3. 如果构建失败，检查 Actions 日志并修复问题" -ForegroundColor White
Write-Host ""
Write-Host "⏱️ 预计构建时间: 15-20 分钟" -ForegroundColor Yellow
