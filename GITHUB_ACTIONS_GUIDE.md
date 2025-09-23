# GitHub Actions 完整使用指南

## 📋 概述

本项目配置了两个 GitHub Actions 工作流：

- **build.yml**: 多平台构建和发布
- **test.yml**: 代码测试和质量检查

## 🔧 步骤 1：配置 GitHub 仓库权限

**重要：** 在推送代码之前，需要先配置 GitHub Actions 权限，否则会出现 "Resource not accessible by integration" 错误。

### 配置方法：

1. 进入你的 GitHub 仓库
2. 点击 **Settings** 标签
3. 在左侧菜单中找到 **Actions** → **General**
4. 滚动到 **Workflow permissions** 部分
5. 选择 **Read and write permissions**
6. 勾选 **Allow GitHub Actions to create and approve pull requests**
7. 点击 **Save** 保存设置

![GitHub Actions 权限设置](https://docs.github.com/assets/cb-45061/images/help/repository/actions-workflow-permissions-repository.png)

## 🚀 步骤 2：推送代码到 GitHub

```bash
# 添加所有文件
git add .

# 提交更改
git commit -m "添加 GitHub Actions 构建配置"

# 推送到 GitHub
git push origin main
```

## 🏷️ 步骤 3：创建版本标签触发构建

### 方法一：命令行创建标签

```bash
# 创建并推送标签
git tag v0.3.0
git push origin v0.3.0

## 🔧 故障排除

### 常见错误及解决方案

#### 1. "Resource not accessible by integration" 错误

**错误信息：**
```

Resource not accessible by integration - https://docs.github.com/rest/releases/releases#create-a-release

````

**解决方案：**
1. 进入 GitHub 仓库 → Settings → Actions → General
2. 在 "Workflow permissions" 部分选择 "Read and write permissions"
3. 勾选 "Allow GitHub Actions to create and approve pull requests"
4. 保存设置后重新运行 workflow

#### 2. "Cache not found" 警告

**说明：** 这是正常现象，首次运行时 Rust 缓存不存在，后续运行会自动缓存。

#### 3. Windows runner 迁移警告

**说明：** GitHub 的提醒信息，不影响当前构建功能。

### 手动触发构建

如果不想创建标签，也可以手动触发构建：

1. 进入 GitHub 仓库的 Actions 页面
2. 选择 "Build Multi-Platform" workflow
3. 点击 "Run workflow" 按钮
4. 选择分支并点击绿色的 "Run workflow" 按钮

## 🎯 构建优化说明

### 📦 构建产物优化

本次更新优化了构建配置：

1. **简化文件类型**：
   - Windows: 只保留 `.exe` 安装程序，移除 `.msi`
   - macOS: 保留 `.dmg` 安装包
   - Linux: 保留 `.deb`、`.rpm` 和 `.AppImage`

2. **规范文件命名**：
   - 格式：`ZAugment-{版本号}-{平台}-{架构}.{扩展名}`
   - 示例：`ZAugment-v0.1.1-windows-x64-setup.exe`

3. **平台标识**：
   - `windows-x64`: Windows 64位
   - `macos-x64`: macOS Intel 芯片
   - `macos-arm64`: macOS Apple Silicon (M1/M2/M3)
   - `linux-amd64`: Linux 64位通用
   - `linux-x86_64`: Linux RPM 包

### 🧪 本地测试

在推送到 GitHub 之前，可以使用测试脚本验证配置：

**Windows:**
```powershell
.\scripts\test-build.ps1
```

**macOS/Linux:**
```bash
chmod +x scripts/test-build.sh
./scripts/test-build.sh
```

### 方法二：GitHub 网页创建 Release

1. 进入 GitHub 仓库页面
2. 点击右侧的 "Releases"
3. 点击 "Create a new release"
4. 输入标签版本 (如 `v0.3.0`)
5. 填写发布说明
6. 点击 "Publish release"

## 📊 步骤 3：查看构建进度

1. 打开 GitHub 仓库页面
2. 点击 "Actions" 标签
3. 查看 "Build Multi-Platform" 工作流
4. 等待所有平台构建完成（通常 10-20 分钟）

### 构建平台说明

- **Windows** (x64): 生成 `.msi` 和 `.exe` 安装包
- **macOS** (Intel x64): 生成 `.dmg` 文件，适用于 Intel Mac
- **macOS** (Apple Silicon arm64): 生成 `.dmg` 文件，适用于 M1/M2/M3 Mac
- **Linux** (x64): 生成 `.AppImage` 和 `.deb` 包

## 📦 步骤 4：下载构建产物

### 从 Artifacts 下载（临时文件）

构建完成后：

1. 在 Actions 页面点击对应的构建任务
2. 滚动到底部的 "Artifacts" 部分
3. 下载对应平台的构建文件

### 从 Releases 下载（正式发布）

如果是通过标签触发的构建：

1. 进入 "Releases" 页面
2. 找到对应版本的 Release
3. 下载 Assets 中的安装包

## 🔧 手动触发构建

如果不想创建标签，可以手动触发：

1. 进入 GitHub 仓库的 Actions 页面
2. 点击 "Build Multi-Platform" 工作流
3. 点击 "Run workflow" 按钮
4. 选择分支后点击 "Run workflow"

## 🧪 测试工作流

每次推送到 `master` 或 `develop` 分支时，会自动运行测试工作流：

- 前端构建测试
- Rust 代码测试
- 代码格式检查
- Clippy 静态分析

## ⚙️ 高级配置

### 自定义构建参数

在 `.github/workflows/build.yml` 中可以修改：

- 目标平台
- 构建参数
- 发布说明模板

### 添加代码签名（可选）

对于生产环境，建议添加代码签名：

1. **Windows**: 配置证书和密码
2. **macOS**: 配置 Apple Developer 证书
3. **Linux**: 配置 GPG 签名

## 💡 使用技巧

### 版本管理

- 使用语义化版本号：`v1.0.0`、`v1.0.1`、`v1.1.0`
- 主版本号：不兼容的 API 修改
- 次版本号：向下兼容的功能性新增
- 修订号：向下兼容的问题修正

### 构建优化

- **缓存**: 已配置 Rust 和 Node.js 缓存
- **并行构建**: 多平台同时构建
- **失败处理**: `fail-fast: false` 确保其他平台继续构建

### 调试构建问题

1. 查看 Actions 日志中的错误信息
2. 本地复现构建环境
3. 检查依赖版本兼容性

## 🔍 常见问题

### Q: 构建失败怎么办？

A: 查看 Actions 日志，常见原因：

- 依赖版本不兼容
- 平台特定的依赖缺失
- 代码语法错误

### Q: 如何加速构建？

A:

- 使用缓存（已配置）
- 减少不必要的依赖
- 优化 Rust 编译设置

### Q: 如何添加新的目标平台？

A: 在 `build.yml` 的 matrix 中添加新的平台配置

## 📝 文件结构

```

.github/
└── workflows/
├── build.yml # 多平台构建和发布
└── test.yml # 代码测试和质量检查

````

## 📋 检查清单

发布前请确认：

- [ ] 所有测试通过
- [ ] 版本号已更新
- [ ] 更新日志已准备
- [ ] 代码已推送到主分支
- [ ] 标签已创建并推送
