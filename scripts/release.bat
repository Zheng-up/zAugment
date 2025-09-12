@echo off
setlocal enabledelayedexpansion

REM ZAugment 发布脚本 (Windows)
REM 用法: scripts\release.bat [版本号]
REM 例如: scripts\release.bat 0.3.1

if "%1"=="" (
    echo [ERROR] 请提供版本号
    echo 用法: %0 ^<版本号^>
    echo 例如: %0 0.3.1
    exit /b 1
)

set VERSION=%1

echo [INFO] 准备发布版本: %VERSION%

REM 检查是否在 git 仓库中
git rev-parse --git-dir >nul 2>&1
if errorlevel 1 (
    echo [ERROR] 当前目录不是 git 仓库
    exit /b 1
)

REM 检查工作目录是否干净
git diff-index --quiet HEAD -- >nul 2>&1
if errorlevel 1 (
    echo [WARNING] 工作目录有未提交的更改
    set /p CONTINUE="是否继续? (y/N): "
    if /i not "!CONTINUE!"=="y" (
        echo [INFO] 发布已取消
        exit /b 1
    )
)

echo [INFO] 更新版本号到 %VERSION%...

REM 更新 Cargo.toml (简单的文本替换)
powershell -Command "(Get-Content src-tauri\Cargo.toml) -replace '^version = \".*\"', 'version = \"%VERSION%\"' | Set-Content src-tauri\Cargo.toml"
echo [SUCCESS] 已更新 src-tauri\Cargo.toml

echo [WARNING] 请手动更新 package.json 和 src-tauri\tauri.conf.json 中的版本号

echo [INFO] 提交版本更新...
git add package.json src-tauri\Cargo.toml src-tauri\tauri.conf.json
git commit -m "chore: bump version to %VERSION%"

echo [INFO] 创建标签 v%VERSION%...
git tag -a "v%VERSION%" -m "Release version %VERSION%"

echo [INFO] 推送到远程仓库...
git push github master
git push github "v%VERSION%"

echo [SUCCESS] 版本 %VERSION% 发布完成！
echo [INFO] GitHub Actions 将自动开始构建，请访问 GitHub Actions 页面查看进度
echo [INFO] 构建完成后，Release 将自动创建并包含所有平台的安装包

pause
