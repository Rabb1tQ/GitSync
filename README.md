# GitSync - GitHub仓库自动同步备份工具

GitSync是一个用Rust编写的GitHub仓库自动同步备份工具，可以自动从GitHub同步所有仓库到本地指定文件夹。

## 功能特性

- 🚀 自动获取GitHub用户的所有仓库
- 📁 支持指定同步目录或使用当前目录
- 🔐 支持私有仓库同步
- 🔄 智能更新已存在的仓库
- ⚡ 支持强制更新模式
- 📊 显示同步进度和结果统计
- 🛡️ 使用GitHub Token进行HTTPS认证

## 安装

### 从源码编译

```bash
git clone <your-repo-url>
cd gitsync
cargo build --release
```

编译完成后，可执行文件位于 `target/release/gitsync`

## 使用方法

### 基本用法

```bash
# 同步到当前目录
gitsync

# 同步到指定目录
gitsync --directory /path/to/backup

# 包含私有仓库
gitsync --include-private

# 强制更新所有仓库
gitsync --force
```

### 命令行参数

- `-d, --directory <DIR>`: 指定同步目录，如果不指定则使用当前目录
- `-t, --token <TOKEN>`: GitHub个人访问令牌，如果不指定则从环境变量GITHUB_TOKEN读取
- `-p, --include-private`: 是否包含私有仓库
- `-f, --force`: 是否强制更新已存在的仓库
- `-h, --help`: 显示帮助信息
- `-V, --version`: 显示版本信息

## 配置

### GitHub令牌设置

1. 访问 [GitHub Settings > Developer settings > Personal access tokens](https://github.com/settings/tokens)
2. 生成一个新的个人访问令牌，确保包含以下权限：
   - `repo` (完整仓库访问权限)
   - `read:user` (读取用户信息)
3. 设置环境变量：
   ```bash
   # Windows
   set GITHUB_TOKEN=your_token_here
   
   # Linux/macOS
   export GITHUB_TOKEN=your_token_here
   ```


## 使用示例

```bash
# 基本同步（只同步公开仓库到当前目录）
gitsync

# 同步所有仓库（包括私有仓库）到指定目录
gitsync --directory ~/github-backup --include-private

# 强制更新所有仓库
gitsync --force

# 使用指定的GitHub令牌
gitsync --token ghp_xxxxxxxxxxxxxxxxxxxx
```

## 输出示例

```
🚀 GitSync - GitHub仓库同步工具
📁 同步目录: /home/user/github-backup
🔐 认证方式: GitHub Token (HTTPS)
👤 用户: username (Your Name)
📋 正在获取仓库列表...
📊 找到 15 个仓库
🔄 开始同步仓库...
[1/15] 🔄 同步: my-project
[1/15] ✅ 完成: my-project
[2/15] 🔄 同步: another-repo
[2/15] ✅ 完成: another-repo
...

📈 同步完成!
✅ 成功: 15 个仓库
```

## 注意事项

1. **必须配置GitHub令牌**：需要GitHub令牌来访问API获取仓库列表和进行Git克隆操作
2. 私有仓库需要相应的访问权限
3. 大量仓库同步可能需要较长时间
4. 建议定期运行以保持仓库同步

## 故障排除

### 常见问题

1. **认证失败**: 检查GitHub令牌是否正确设置
2. **权限不足**: 检查令牌权限是否包含所需范围
3. **网络问题**: 检查网络连接和防火墙设置

## 开发

### 依赖项

- tokio: 异步运行时
- reqwest: HTTP客户端
- serde: 序列化/反序列化
- clap: 命令行参数解析
- git2: Git操作
- anyhow: 错误处理
- dirs: 获取用户目录

### 构建

```bash
cargo build
cargo test
cargo run -- --help
```

## 许可证

MIT License
