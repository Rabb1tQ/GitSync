use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

pub struct Config {
    pub sync_dir: PathBuf,
    pub github_token: String,
    pub include_private: bool,
    pub force: bool,
}

impl Config {
    pub fn new(
        directory: Option<PathBuf>,
        token: Option<String>,
        include_private: bool,
        force: bool,
    ) -> Result<Self> {
        // 确定同步目录
        let sync_dir = match directory {
            Some(dir) => {
                if dir.is_absolute() {
                    dir
                } else {
                    env::current_dir()
                        .context("获取当前目录失败")?
                        .join(dir)
                }
            }
            None => env::current_dir()
                .context("获取当前目录失败")?,
        };
        
        // 确保同步目录存在
        std::fs::create_dir_all(&sync_dir)
            .context("创建同步目录失败")?;
        
        // 获取GitHub令牌
        let github_token = match token {
            Some(t) => t,
            None => env::var("GITHUB_TOKEN")
                .context("未找到GitHub令牌，请设置GITHUB_TOKEN环境变量或使用--token参数")?,
        };
        
        if github_token.is_empty() {
            anyhow::bail!("GitHub令牌不能为空");
        }
        
        Ok(Self {
            sync_dir,
            github_token,
            include_private,
            force,
        })
    }
}
