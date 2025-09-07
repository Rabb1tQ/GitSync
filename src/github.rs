use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct User {
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub clone_url: String,
    pub ssh_url: String,
    pub private: bool,
    pub default_branch: String,
    pub updated_at: String,
    pub size: u64,
}

pub struct GitHubClient {
    client: Client,
    token: String,
}

impl GitHubClient {
    pub fn new(token: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("创建HTTP客户端失败")?;
        
        Ok(Self {
            client,
            token: token.to_string(),
        })
    }
    
    pub async fn get_user(&self) -> Result<User> {
        let response = self
            .client
            .get("https://api.github.com/user")
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", "GitSync/1.0")
            .send()
            .await
            .context("获取用户信息失败")?;
        
        if !response.status().is_success() {
            anyhow::bail!("GitHub API请求失败: {}", response.status());
        }
        
        let user: User = response
            .json()
            .await
            .context("解析用户信息失败")?;
        
        Ok(user)
    }
    
    pub async fn get_repositories(&self, include_private: bool) -> Result<Vec<Repository>> {
        let mut all_repos = Vec::new();
        let mut page = 1;
        let per_page = 100;
        
        loop {
            let url = format!(
                "https://api.github.com/user/repos?page={}&per_page={}&sort=updated&direction=desc",
                page, per_page
            );
            
            let response = self
                .client
                .get(&url)
                .header("Authorization", format!("token {}", self.token))
                .header("User-Agent", "GitSync/1.0")
                .send()
                .await
                .context("获取仓库列表失败")?;
            
            if !response.status().is_success() {
                anyhow::bail!("GitHub API请求失败: {}", response.status());
            }
            
            let repos: Vec<Repository> = response
                .json()
                .await
                .context("解析仓库列表失败")?;
            
            if repos.is_empty() {
                break;
            }
            
            // 根据include_private参数过滤仓库
            for repo in &repos {
                if !repo.private || include_private {
                    all_repos.push(repo.clone());
                }
            }
            
            // 如果返回的仓库数量少于per_page，说明已经是最后一页
            if repos.len() < per_page {
                break;
            }
            
            page += 1;
        }
        
        Ok(all_repos)
    }
}
