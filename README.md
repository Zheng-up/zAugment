# 🚀 ZAugment - 智能代码助手管理器

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vue.js](https://img.shields.io/badge/Vue.js-35495E?style=flat&logo=vuedotjs&logoColor=4FC08D)](https://vuejs.org/)
[![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=flat&logo=tauri&logoColor=white)](https://tauri.app/)
[![基于开源项目](https://img.shields.io/badge/基于-ATM_开源项目-blue.svg)](https://github.com/zhaochengcube/augment-token-mng)

> 基于 Tauri + Vue.js 构建的现代化桌面应用，专为 Augment Code 智能代码助手提供账号管理、云同步和防封保护功能。

## 💬 加入交流群

<div align="center">

![交流群二维码](./交流群.png)

**群内不定时发放 Augment、Cursor、Codex 等 AI 账号福利** 🎁

扫码加入 QQ 交流群，与其他用户一起交流使用经验、反馈问题和建议！

</div>

## 📌 项目说明

本项目基于开源项目 [**Augment Token Manager (ATM)**](https://github.com/zhaochengcube/augment-token-mng) 进行二次开发，页面UI重构，操作逻辑更符合直觉，增加webdav云同步功能，自动同步和手动同步，冲突检测和解决，账号状态监控，账号阈值配置，账号批量操作，账号导入导出，账号标签，账号统计，账号图表展示，账号备份恢复等等。


## 🚀 快速开始

### 📋 系统要求

- Windows 10+ / macOS 10.15+ / Linux (Ubuntu 18.04+)
- 4GB+ RAM，100MB+ 存储空间

### ⚡ 开发运行

```bash
# 安装依赖
npm install

# 启动开发服务器
cargo tauri dev

# 构建生产版本
npm run tauri build
```

## 📖 使用指南

### 🎯 **步骤 1：获取 Augment 账号**


**方式一：购买账号**

1. 点击 **"购买账号"** 按钮，跳转到购买页面
2. 购买完成后，将Session信息填入应用保存

**方式二：手动注册**

1. 打开Zaugment新增流程 按照OAuth流程 或 Session导入（点击问号 查看步骤） 进行注册

💡 **小贴士**：Session 导入最简单，无需只需复制Session信息点击导入

### 👥 **步骤 2：账号管理**

- **💾 查看账号**：切换到"账号管理"标签，查看所有已保存账号
- **✏️ 编辑账号**：点击账号卡片的编辑按钮，修改账号信息
- **🗑️ 删除账号**：点击删除按钮，确认后安全删除账号
- **📊 批量操作**：使用顶部的保存/刷新/检测按钮进行批量管理
- **📈 状态监控**：实时显示每个账号的正常/异常/封禁/过期状态
- **🏷️ 标签管理**：为账号添加自定义标签，方便分类
- **🔍 智能搜索**：按状态、标签、邮箱快速筛选账号

### ☁️ **步骤 3：配置云同步** 

1. 进入 **"应用设置"** → **"云同步设置"**
2. 点击 **"云存储平台"** 选择器，从弹窗中选择平台：
   - **🥜 坚果云**（推荐）：国内访问稳定
   - **☁️ TeraCloud**：亚洲地区优化
   - **🔴 Yandex Disk**：免费 10GB 空间
   - **🟢 Koofr**：隐私保护出色
   - **📦 Box 企业版**：企业级解决方案
3. 填入对应平台的认证信息（每个平台单独保存）
4. 点击 **"测试连接"** 验证配置正确性
5. 点击 **"保存配置"** 并开启自动同步

💡 **小贴士**：不同平台的账户信息会分别保存，可以随时切换使用



## 🏗️ 技术栈

- **前端**：Vue.js 3 + Composition API + CSS3
- **后端**：Rust + Tauri + WebDAV
- **特性**：跨平台、轻量级（<20MB）、高性能

## � 支持项目

<div align="center">

![赞赏码](./赞赏.png)

**您的支持是我们持续改进的动力！** 💖

</div>
