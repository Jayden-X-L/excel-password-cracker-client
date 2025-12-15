# Excel Password Cracker Client (EPCC)

## 🚀 产品概述

EPCC 是一款功能强大、用户友好的跨平台 Excel 密码破解桌面应用，集成了业界领先的密码破解工具 `hashcat` 和 `office2hashcat`，为用户提供高效、可靠的 Excel 密码恢复解决方案。

### 🔍 核心价值

- **高效破解**：利用 hashcat 的强大计算能力，支持 CPU/GPU 加速
- **用户友好**：直观的图形界面，无需命令行操作
- **多种攻击模式**：字典、暴力、掩码攻击，满足不同场景需求
- **跨平台支持**：同时支持 macOS 和 Windows 系统
- **灵活配置**：支持自定义字符集、设备选择、会话管理

### 🎯 目标用户

1. **个人用户**：忘记自己 Excel 文件密码的普通用户
2. **企业 IT 人员**：需要恢复员工遗留文件的 IT 管理员
3. **安全专业人员**：进行授权安全测试的渗透测试人员
4. **教育研究人员**：用于密码学教育和研究的学术人员

## ✨ 主要功能

### 📁 Excel 文件处理

- **多文件支持**：同时处理多个 Excel 文件
- **自动哈希提取**：一键提取 Excel 文件哈希值
- **版本兼容**：支持 Office 2007/2010/2013/2016/2019/2021
- **自动模式识别**：智能识别哈希类型和对应的破解模式

### 🔧 攻击模式

#### 📚 字典攻击

- 使用指定字典文件进行密码破解
- 支持自定义字典路径
- 适合常见密码和弱密码破解

#### 💪 纯暴力攻击

- 根据字符集和长度范围生成所有可能的密码
- 支持配置数字、小写字母、大写字母、特殊字符
- 增量破解：从最小长度到最大长度

#### 🎭 掩码攻击

- 使用自定义掩码进行精准破解
- 支持 hashcat 掩码语法（?d, ?l, ?u, ?s, ?a 等）
- 适合已知部分密码结构的场景

### 🖥️ 设备管理

- **自动检测**：智能检测系统可用设备
- **设备选择**：支持仅 CPU、仅 GPU 或自动模式
- **特定设备**：可选择特定设备进行破解
- **实时监控**：显示设备使用情况和破解进度

### 🎮 攻击控制

- **开始/暂停/恢复/停止**：灵活的攻击控制
- **会话管理**：保存和恢复破解进度
- **实时状态**：显示破解速度、已尝试密码数、剩余时间等
- **命令预览**：可视化显示生成的 hashcat 命令

### ⚙️ 高级配置

- **自定义工具路径**：支持配置 hashcat 和 office2hashcat 路径
- **会话命名**：自定义会话名称，方便管理
- **状态更新频率**：可配置状态更新间隔

## 🛠️ 技术优势

### 🏗️ 架构设计

- **前后端分离**：React 前端 + Rust 后端，实现高效通信
- **模块化设计**：清晰的代码结构，便于维护和扩展
- **跨平台兼容**：基于 Tauri 框架，支持 macOS 和 Windows

### 🚀 性能优化

- **GPU 加速**：充分利用现代 GPU 的并行计算能力
- **异步处理**：后台运行破解任务，不阻塞 UI
- **高效 I/O**：优化的文件处理和进程管理

### 🔒 安全性

- **本地运行**：所有破解过程在本地完成，数据不离开设备
- **安全沙箱**：基于 Tauri 的安全模型，限制应用权限
- **透明操作**：可视化显示所有执行的命令和操作

## 📊 技术规格

### 系统要求

#### macOS

- **系统版本**：macOS 13.0+（Ventura 或更高）
- **CPU**：64 位 Intel 或 Apple Silicon
- **内存**：4GB 以上
- **GPU**：支持 OpenCL 或 Metal 的 GPU（推荐）
- **存储空间**：1GB 以上可用空间

#### Windows

- **系统版本**：Windows 10+（64 位）
- **CPU**：64 位 Intel 或 AMD
- **内存**：4GB 以上
- **GPU**：支持 OpenCL 或 CUDA 的 GPU（推荐）
- **存储空间**：1GB 以上可用空间

### 支持的 Excel 版本

- Office 2007 (.xlsx)
- Office 2010 (.xlsx)
- Office 2013 (.xlsx)
- Office 2016 (.xlsx)
- Office 2019 (.xlsx)
- Office 2021 (.xlsx)
- Office 365 (.xlsx)

### 支持的哈希类型

- Office 2007：hashcat 模式 9400
- Office 2010：hashcat 模式 9500
- Office 2013-2021：hashcat 模式 9600

## 📈 使用场景

### 1. 个人用户场景

**场景**：忘记了自己重要 Excel 文件的密码
**解决方案**：使用 EPCC 进行字典攻击或暴力攻击，恢复文件访问权限
**优势**：操作简单，无需专业知识，支持多种攻击模式

### 2. 企业 IT 场景

**场景**：员工离职后遗留的加密 Excel 文件，需要恢复访问
**解决方案**：使用 EPCC 进行高效破解，结合公司内部密码策略配置字符集
**优势**：支持多文件处理，可配置设备资源，提高破解效率

### 3. 安全测试场景

**场景**：评估公司内部 Excel 文件的密码强度
**解决方案**：使用 EPCC 进行授权的安全测试，识别弱密码
**优势**：支持多种攻击模式，可生成详细的测试报告

### 4. 教育研究场景

**场景**：密码学课程中的实践教学
**解决方案**：使用 EPCC 演示不同密码破解技术的原理和效果
**优势**：可视化界面，便于理解密码破解过程，支持多种攻击模式

## 🎨 界面预览

### 主界面

![主界面](https://github.com/Jayden-X-L/excel-password-cracker-client/assets/Jayden-X-L/12345678/abc12345-6789-0123-4567-890abcdef123)

### 攻击配置

![攻击配置](https://github.com/Jayden-X-L/excel-password-cracker-client/assets/Jayden-X-L/12345678/def45678-9012-3456-7890-123abcdef456)

### 状态监控

![状态监控](https://github.com/Jayden-X-L/excel-password-cracker-client/assets/Jayden-X-L/12345678/ghi78901-2345-6789-0123-4567890abcdef)

## 📦 安装方式

### 预编译安装包

从 [GitHub Releases](https://github.com/Jayden-X-L/excel-password-cracker-client/releases) 下载适合您系统的安装包：

- **macOS**：`EPCC_*.dmg`
- **Windows**：`EPCC_*.exe`

### 源代码构建

```bash
# 克隆仓库
git clone https://github.com/Jayden-X-L/excel-password-cracker-client.git
cd excel-password-cracker-client

# 安装依赖
npm install

# 构建开发版本
npm run tauri dev

# 构建生产版本
npm run tauri build
```

## 🤝 社区与支持

### 社区贡献

欢迎社区成员参与项目贡献，包括：

- 报告 Bug
- 提出功能请求
- 提交代码
- 改进文档
- 提供反馈

### 支持渠道

- **GitHub Issues**：[https://github.com/Jayden-X-L/excel-password-cracker-client/issues](https://github.com/Jayden-X-L/excel-password-cracker-client/issues)
- **电子邮件**：your-email@example.com
- **Discord**：[https://discord.gg/yourserver](https://discord.gg/yourserver)（可选）

## 📄 许可证

本项目采用 [MIT 许可证](LICENSE) 开源。

## 📞 联系方式

- **项目主页**：[https://github.com/Jayden-X-L/excel-password-cracker-client](https://github.com/Jayden-X-L/excel-password-cracker-client)
- **作者**：Your Name
- **电子邮件**：your-email@example.com

---

**EPCC - 让 Excel 密码恢复变得简单高效！**

*使用本工具时，请遵守相关法律法规，仅用于合法目的！*
