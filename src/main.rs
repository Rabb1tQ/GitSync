use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod github;
mod git;
mod config;

use github::GitHubClient;
use git::GitManager;
use config::Config;

#[derive(Parser)]
#[command(name = "gitsync")]
#[command(about = "GitHub仓库自动同步备份工具")]
#[command(version)]
struct Cli {
    /// 指定同步目录，如果不指定则使用当前目录
    #[arg(short, long)]
    directory: Option<PathBuf>,
    
    /// GitHub个人访问令牌，如果不指定则从环境变量GITHUB_TOKEN读取
    #[arg(short, long)]
    token: Option<String>,
    
    /// 是否包含私有仓库
    #[arg(short, long)]
    include_private: bool,
    
    /// 是否强制更新已存在的仓库
    #[arg(short, long)]
    force: bool,
    
    /// 是否跳过失败的仓库继续同步其他仓库
    #[arg(short, long)]
    continue_on_error: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // 获取配置
    let config = Config::new(cli.directory, cli.token, cli.include_private, cli.force)?;
    
    println!("🚀 GitSync - GitHub仓库同步工具");
    println!("📁 同步目录: {}", config.sync_dir.display());
    println!("🔐 认证方式: GitHub Token (HTTPS)");
    
    // 创建GitHub客户端
    let github_client = GitHubClient::new(&config.github_token)?;
    
    // 获取用户信息
    let user = github_client.get_user().await?;
    println!("👤 用户: {} ({})", user.login, user.name.unwrap_or_default());
    
    // 获取仓库列表
    println!("📋 正在获取仓库列表...");
    let repos = github_client.get_repositories(config.include_private).await?;
    println!("📊 找到 {} 个仓库", repos.len());
    
    if repos.is_empty() {
        println!("✅ 没有找到需要同步的仓库");
        return Ok(());
    }
    
    // 创建Git管理器
    let git_manager = GitManager::new(&config.sync_dir);
    
    // 同步仓库
    println!("🔄 开始同步仓库...");
    let mut success_count = 0;
    let mut error_count = 0;
    
    for (i, repo) in repos.iter().enumerate() {
        let progress = format!("[{}/{}]", i + 1, repos.len());
        println!("{} 🔄 同步: {}", progress, repo.name);
        
        match git_manager.sync_repository(repo, &config).await {
            Ok(_) => {
                println!("{} ✅ 完成: {}", progress, repo.name);
                success_count += 1;
            }
            Err(e) => {
                println!("{} ❌ 失败: {} - {}", progress, repo.name, e);
                // 显示详细的错误链
                let mut source = e.source();
                while let Some(err) = source {
                    println!("   └─ 原因: {}", err);
                    source = err.source();
                }
                error_count += 1;
                
                // 如果不允许继续，则退出
                if !cli.continue_on_error {
                    anyhow::bail!("同步失败，使用 --continue-on-error 参数可以跳过失败的仓库继续同步其他仓库");
                }
            }
        }
    }
    
    // 输出结果
    println!("\n📈 同步完成!");
    println!("✅ 成功: {} 个仓库", success_count);
    if error_count > 0 {
        println!("❌ 失败: {} 个仓库", error_count);
    }
    
    Ok(())
}
