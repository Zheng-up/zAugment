# 🚀 ZAugment - 智能代码助手管理器

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vue.js](https://img.shields.io/badge/Vue.js-35495E?style=flat&logo=vuedotjs&logoColor=4FC08D)](https://vuejs.org/)
[![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=flat&logo=tauri&logoColor=white)](https://tauri.app/)
[![基于开源项目](https://img.shields.io/badge/基于-ATM_开源项目-blue.svg)](https://github.com/zhaochengcube/augment-token-mng)

> 基于 Tauri + Vue.js 构建的现代化桌面应用，专为 Augment Code 智能代码助手提供账号管理、云同步和防封保护功能。

## 📌 项目说明

本项目基于开源项目 [**Augment Token Manager (ATM)**](https://github.com/zhaochengcube/augment-token-mng) 进行二次开发和功能增强。

**原项目信息：**

- 🌟 **383+ Stars**：受到众多开发者认可
- 🍴 **79+ Forks**：活跃的开源社区
- 📜 **MIT License**：开放友好的开源协议
- 👨‍💻 **原作者**：[@zhaochengcube](https://github.com/zhaochengcube)

感谢原项目提供的优秀基础框架，本项目在此基础上增加了云同步、多平台支持、防封保护等增强功能。

## ✨ 核心功能

### 🎯 **智能注册系统**

- **一键生成授权 URL**：自动生成 Augment 注册链接
- **简化注册流程**：通过防封浏览器安全注册
- **JSON 数据提取**：自动解析注册响应数据
- **自动保存账号**：注册完成后自动保存到本地
- **Portal URL**：一键获取账号管理后台地址

### 👥 **多账号管理**

- **账号列表管理**：统一管理所有 Augment 账号
- **状态实时监控**：显示账号正常/异常状态
- **批量操作支持**：批量保存、刷新、导出
- **安全信息存储**：本地加密存储敏感信息
- **快速编辑删除**：支持账号信息的快速修改

### ☁️ **多平台云同步**

- **🥜 坚果云**：国内首选，访问稳定
- **☁️ TeraCloud**：日本服务器，亚洲优化
- **🔴 Yandex Disk**：俄罗斯服务，免费 10GB
- **🟢 Koofr**：欧洲服务器，隐私友好
- **📦 Box 企业版**：企业级解决方案
- **⚙️ 自定义 WebDAV**：支持其他兼容服务
- **智能平台选择**：弹窗式平台选择器，支持一键切换

### 🛡️ **防封保护系统**

- **VSCode 系列插件**：适配 Visual Studio Code 生态
- **JetBrains 系列插件**：支持 IDEA、WebStorm 等
- **智能防检测**：降低账号封禁风险
- **一键下载安装**：简化插件获取流程
- **分类插件管理**：按开发环境分类提供插件

### ⚙️ **高级设置**

- **灵活数据存储**：自定义本地数据目录
- **自动同步策略**：文件变化时自动云端备份
- **强制同步控制**：手动强制上传/下载数据
- **同步历史记录**：详细的操作日志追踪

## 🖼️ 界面预览

### 📱 主要功能界面

<table>
<tr>
<td width="50%">

**🎯 Augment 注册界面**

- 一键生成授权 URL
- JSON 数据解析处理
- 自动保存账号信息
- Portal URL 获取

</td>
<td width="50%">

**👥 账号管理界面**

- 多账号统一管理
- 实时状态监控
- 批量操作支持
- 快速编辑删除

</td>
</tr>
<tr>
<td width="50%">

**☁️ 云同步设置界面**

- 多平台选择器
- WebDAV 配置管理
- 自动同步策略
- 强制同步控制

</td>
<td width="50%">

**🛡️ 支持的平台**

- VSCode 系列登录
- JetBrains 系列登录
- 分类插件展示
- 一键下载防封插件

</td>
</tr>
</table>

## 🌟 产品特色

### 🎨 **现代化设计**

- **毛玻璃界面**：现代化 UI 设计，视觉效果出众
- **响应式布局**：完美适配不同屏幕尺寸
- **暗色主题**：护眼的深色调侧边栏设计
- **流畅动画**：细腻的交互动效，提升使用体验

### 🔒 **安全保障**

- **本地优先**：敏感数据优先本地存储
- **加密传输**：云同步采用 HTTPS 加密传输
- **隐私保护**：不收集用户隐私信息
- **数据可控**：用户完全控制数据存储位置

### ⚡ **性能优越**

- **启动速度快**：基于 Rust 的高性能后端
- **内存占用低**：比传统 Electron 应用节省 50% 内存
- **跨平台原生**：真正的原生应用性能体验
- **资源友好**：低 CPU 占用，不影响其他应用

### 🎯 **用户友好**

- **零学习成本**：直观的界面设计，无需学习即可使用
- **一键操作**：复杂流程简化为简单的一键操作
- **智能提示**：贴心的操作提示和错误处理
- **多语言支持**：完整的中文界面本地化

## 🚀 快速开始

### 📋 系统要求

- **操作系统**：Windows 10+ / macOS 10.15+ / Linux (Ubuntu 18.04+)
- **内存**：最低 4GB RAM
- **存储空间**：至少 100MB 可用空间
- **网络**：需要互联网连接用于注册和同步

### ⚡ 一键构建

#### Windows 用户：

```powershell
# 克隆项目
git clone https://github.com/your-username/zaugment.git
cd zaugment

# 一键构建
.\build.ps1
```

#### macOS/Linux 用户：

```bash
# 克隆项目
git clone https://github.com/your-username/zaugment.git
cd zaugment

# 一键构建
chmod +x build.sh
./build.sh
```

### 🔧 开发环境搭建

#### 1. 环境准备

**安装 Rust：**

```bash
# Windows (PowerShell)
Invoke-WebRequest -Uri https://win.rustup.rs/ -OutFile rustup-init.exe
.\rustup-init.exe

# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**安装 Node.js：**

- 从 [nodejs.org](https://nodejs.org/) 下载安装
- 推荐版本：Node.js 18+

**安装 Tauri CLI：**

```bash
cargo install tauri-cli
```

#### 2. 开发模式

```bash
# 安装依赖
npm install

# 启动开发服务器（热重载）
cargo tauri dev

# 构建生产版本
npm run tauri build
```

## 📖 使用指南

### 🎯 **步骤 1：Augment 注册** _(参见图 1)_

1. 点击 **"生成授权 URL"** 按钮，系统自动生成注册链接
2. 复制生成的 URL，在防封浏览器中打开进行注册
3. 注册完成后，复制账号管理页面的 **Portal URL**
4. 填入邮箱账号信息，点击 **"保存账号"** 完成注册

💡 **小贴士**：整个流程只需 3 步，无需手动处理复杂的 JSON 数据

### 👥 **步骤 2：账号管理** _(参见图 2)_

- **💾 查看账号**：切换到"账号管理"标签，查看所有已保存账号
- **✏️ 编辑账号**：点击账号卡片的编辑按钮，修改账号信息
- **🗑️ 删除账号**：点击删除按钮，确认后安全删除账号
- **📊 批量操作**：使用顶部的保存/刷新按钮进行批量管理
- **📈 状态监控**：实时显示每个账号的正常/异常状态

### ☁️ **步骤 3：配置云同步** _(参见图 3)_

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

## 🔧 高级功能

### 💾 **数据管理**

- **自定义存储位置**：可更改应用数据存储目录
- **数据备份**：支持手动和自动云端备份
- **历史记录**：完整的同步操作历史

### 🌐 **云同步策略**

- **自动同步**：文件变化时自动上传到云端
- **强制同步**：手动强制上传/下载，支持确认对话框
- **冲突处理**：全覆盖策略，确保数据一致性

### 🔒 **安全特性**

- **本地加密存储**：敏感信息本地安全存储
- **多平台隔离**：不同云平台的配置独立管理
- **访问日志**：详细的操作日志记录

## 🏗️ 技术架构

### 前端技术栈

- **Vue.js 3**：响应式前端框架
- **Composition API**：现代化 Vue 开发模式
- **CSS3**：现代化 UI 设计，支持毛玻璃效果
- **响应式设计**：完美适配桌面和移动端

### 后端技术栈

- **Rust**：高性能系统级编程语言
- **Tauri**：轻量级桌面应用框架
- **WebDAV 客户端**：标准 WebDAV 协议实现
- **本地存储**：JSON 文件数据持久化

### 核心特性

- **跨平台**：Windows、macOS、Linux 全平台支持
- **轻量级**：安装包小于 20MB
- **高性能**：Rust 后端确保运行效率
- **安全性**：本地数据加密，云端传输安全

## 📁 项目结构

```
zaugment/
├── src/                    # Vue.js 前端源码
│   ├── App.vue            # 主应用组件
│   ├── components/        # Vue组件库
│   │   ├── TokenList.vue  # Token列表组件
│   │   ├── TokenForm.vue  # Token表单组件
│   │   └── ...
│   └── main.js           # 应用入口文件
├── src-tauri/            # Tauri后端源码
│   ├── src/              # Rust源码
│   │   ├── main.rs       # 主程序入口
│   │   ├── webdav/       # WebDAV模块
│   │   └── storage/      # 存储模块
│   └── Cargo.toml        # Rust依赖配置
├── public/               # 静态资源
├── dist/                 # 构建输出
└── build.ps1/build.sh    # 一键构建脚本
```

## 🌟 主要优势

### 🎯 **用户体验**

- **现代化界面**：Material Design 风格，直观易用
- **一键操作**：复杂流程简化为一键完成
- **实时反馈**：操作状态实时显示，错误提示清晰
- **多语言支持**：完整的中文界面

### 🔧 **技术优势**

- **内存占用低**：Tauri 架构比 Electron 更轻量
- **启动速度快**：Rust 后端确保快速响应
- **跨平台兼容**：一次开发，多平台运行
- **安全可靠**：本地数据加密，云端安全传输

### 🌐 **生态整合**

- **多 IDE 支持**：VSCode、JetBrains 全系列支持
- **云存储兼容**：支持 5+主流 WebDAV 服务
- **插件生态**：丰富的防封插件资源
- **开源透明**：代码完全开源，可自定义修改

### 🙏 赞赏支持

![赞赏码](./赞赏.png)

**您的支持是我们持续改进的动力！**

感谢每一位用户的理解与支持 💖

</div>
