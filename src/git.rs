use anyhow::{Context, Result};
use git2::{Repository, RemoteCallbacks, Cred, FetchOptions, build::CheckoutBuilder};
use std::path::{Path, PathBuf};
use std::fs;
use crate::github::Repository as GitHubRepo;
use crate::config::Config;

pub struct GitManager {
    sync_dir: PathBuf,
}

impl GitManager {
    pub fn new(sync_dir: &Path) -> Self {
        Self {
            sync_dir: sync_dir.to_path_buf(),
        }
    }
    
    pub async fn sync_repository(&self, repo: &GitHubRepo, config: &Config) -> Result<()> {
        let repo_path = self.sync_dir.join(&repo.name);
        
        // 检查仓库是否已存在
        if repo_path.exists() {
            if config.force {
                // 强制更新：删除现有目录并重新克隆
                fs::remove_dir_all(&repo_path)
                    .context("删除现有仓库目录失败")?;
                self.clone_repository(repo, &repo_path, config).await?;
            } else {
                // 更新现有仓库
                self.update_repository(&repo_path, repo, config).await?;
            }
        } else {
            // 克隆新仓库
            self.clone_repository(repo, &repo_path, config).await?;
        }
        
        Ok(())
    }
    
    async fn clone_repository(&self, repo: &GitHubRepo, path: &Path, config: &Config) -> Result<()> {
        // 确保父目录存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context("创建目录失败")?;
        }
        
        // 使用HTTPS URL和token认证
        let url = format!("https://{}@github.com/{}.git", config.github_token, repo.full_name);
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::userpass_plaintext(username_from_url.unwrap_or("git"), &config.github_token)
        });
        
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);
        
        // 执行克隆
        let _repo = builder
            .clone(&url, path)
            .with_context(|| format!("克隆仓库失败: {} -> {}", repo.full_name, path.display()))?;
        
        Ok(())
    }
    
    async fn update_repository(&self, path: &Path, repo: &GitHubRepo, config: &Config) -> Result<()> {
        let git_repo = Repository::open(path)
            .context("打开现有仓库失败")?;
        
        // 获取远程仓库
        let mut remote = git_repo
            .find_remote("origin")
            .context("找不到origin远程仓库")?;
        
        // 配置fetch选项
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::userpass_plaintext(username_from_url.unwrap_or("git"), &config.github_token)
        });
        
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        
        // 获取最新更改
        remote
            .fetch(&[&repo.default_branch], Some(&mut fetch_options), None)
            .context("获取远程更改失败")?;
        
        // 获取FETCH_HEAD引用
        let fetch_head = git_repo
            .find_reference("FETCH_HEAD")
            .context("找不到FETCH_HEAD引用")?;
        
        let fetch_commit = git_repo
            .reference_to_annotated_commit(&fetch_head)
            .context("获取fetch提交失败")?;
        
        // 分析合并
        let analysis = git_repo
            .merge_analysis(&[&fetch_commit])
            .context("分析合并失败")?;
        
        if analysis.0.is_up_to_date() {
            // 已经是最新的
            return Ok(());
        } else if analysis.0.is_fast_forward() {
            // 快进合并
            let mut reference = git_repo
                .find_reference(&format!("refs/heads/{}", repo.default_branch))
                .context("找不到分支引用")?;
            
            reference
                .set_target(fetch_commit.id(), "Fast-forward")
                .context("设置引用目标失败")?;
            
            git_repo
                .set_head(&format!("refs/heads/{}", repo.default_branch))
                .context("设置HEAD失败")?;
            
            git_repo
                .checkout_head(Some(CheckoutBuilder::default().force()))
                .context("检出HEAD失败")?;
        } else {
            // 需要手动合并，这里我们选择重置到远程分支
            let obj = git_repo
                .reference_to_annotated_commit(&fetch_head)
                .context("获取fetch对象失败")?;
            
            let obj = git_repo
                .find_object(obj.id(), None)
                .context("查找对象失败")?;
            
            git_repo
                .reset(&obj, git2::ResetType::Hard, None)
                .context("重置仓库失败")?;
        }
        
        Ok(())
    }
}
