# 贡献指南

感谢您对 EPCC (Excel Password Cracker Client) 项目的关注和支持！我们欢迎社区成员的贡献，无论是 bug 报告、功能请求、代码提交还是文档改进。

## 行为准则

在参与本项目时，请遵守以下行为准则：

- 尊重他人，保持友好和专业的沟通
- 接受建设性的批评
- 专注于项目的最佳利益
- 尊重隐私和知识产权
- 遵守相关法律法规

## 如何贡献

### 1. 报告 Bug

如果您发现了 bug，可以通过以下步骤报告：

1. 检查 [GitHub Issues](https://github.com/Jayden-X-L/excel-password-cracker-client/issues) 中是否已有类似的 bug 报告
2. 如果没有，创建一个新的 issue，包含以下信息：
   - 清晰的标题，描述问题
   - 详细的复现步骤
   - 预期行为和实际行为
   - 操作系统和版本
   - 应用版本
   - 任何相关的日志或截图

### 2. 提出功能请求

如果您有新功能的想法，可以通过以下步骤提出：

1. 检查 [GitHub Issues](https://github.com/Jayden-X-L/excel-password-cracker-client/issues) 中是否已有类似的功能请求
2. 如果没有，创建一个新的 issue，包含以下信息：
   - 清晰的标题，描述功能
   - 详细的功能描述
   - 功能的使用场景
   - 任何相关的设计或实现建议

### 3. 提交代码

如果您想要直接贡献代码，请按照以下流程操作：

#### 开发环境设置

1. 克隆仓库：
   ```bash
   git clone https://github.com/Jayden-X-L/excel-password-cracker-client.git
   cd excel-password-cracker-client
   ```

2. 安装依赖：
   ```bash
   npm install
   ```

3. 构建开发版本：
   ```bash
   npm run tauri dev
   ```

#### 开发流程

1. Fork 仓库到您的 GitHub 账号
2. 创建一个特性分支：
   ```bash
   git checkout -b feature/AmazingFeature
   ```
   或修复分支：
   ```bash
   git checkout -b fix/BugFix
   ```

3. 进行代码修改，确保：
   - 代码符合项目的代码规范
   - 所有测试通过
   - 代码有适当的注释
   - 不破坏现有功能

4. 运行测试和 lint 检查：
   ```bash
   npm run lint
   # 如果有测试，运行测试命令
   ```

5. 提交更改，使用清晰的提交信息：
   ```bash
   git commit -m 'Add some AmazingFeature'
   ```
   或
   ```bash
   git commit -m 'Fix some Bug'
   ```

6. 推送到您的分支：
   ```bash
   git push origin feature/AmazingFeature
   ```

7. 提交 Pull Request 到主仓库：
   - 选择正确的分支（通常是 `main`）
   - 提供清晰的 PR 标题和描述
   - 关联相关的 issue（如果有）

#### 代码规范

- **前端代码**：
  - 使用 TypeScript 编写
  - 遵循 ESLint 规则
  - 使用 React Hooks
  - 组件命名使用 PascalCase
  - 变量和函数命名使用 camelCase

- **Rust 代码**：
  - 遵循 Rust 官方风格指南
  - 使用 `cargo fmt` 格式化代码
  - 使用 `cargo clippy` 检查代码
  - 函数命名使用 snake_case
  - 类型命名使用 PascalCase

## 项目结构

```
excel-password-cracker-client/
├── src/              # 前端源代码
│   ├── assets/       # 前端资源
│   ├── App.css       # 主应用样式
│   ├── App.tsx       # 主应用组件
│   ├── index.css     # 全局样式
│   └── main.tsx      # 入口文件
└── src-tauri/        # Tauri 桌面应用代码
    ├── capabilities/ # Tauri 能力配置
    ├── gen/          # 生成的配置文件
    ├── icons/        # 应用图标
    ├── resources/    # 应用资源
    └── src/          # Rust 后端代码
        ├── commands.rs # Tauri 命令处理
        ├── hash.rs     # 哈希提取逻辑
        ├── lib.rs      # Rust 库入口
        ├── main.rs     # 主程序入口
        ├── process.rs  # 进程管理
        └── state.rs    # 应用状态管理
```

## 测试

目前项目没有自动化测试，但我们计划在未来添加。在提交代码之前，请手动测试您的更改，确保它们能够正常工作，并且不会破坏现有功能。

## 文档

如果您的更改涉及到功能或 API 的变化，请确保更新相关的文档，包括：
- README.md
- 使用说明
- 代码注释

## 审核流程

当您提交 Pull Request 后，项目维护者会进行审核：

1. 检查代码质量和规范
2. 验证功能是否符合预期
3. 确保不破坏现有功能
4. 提供反馈和建议
5. 合并或拒绝 Pull Request

## 联系方式

如果您有任何问题或建议，可以通过以下方式联系我们：

- [GitHub Issues](https://github.com/Jayden-X-L/excel-password-cracker-client/issues)
- [电子邮件](mailto:your-email@example.com)

## 致谢

感谢所有为 EPCC 项目做出贡献的社区成员！您的支持和贡献对项目的成功至关重要。

---

再次感谢您的关注和支持！让我们一起打造更好的 Excel 密码破解工具。
