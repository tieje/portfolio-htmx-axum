use std::{fs::File, io::Read};

use octocrab::Octocrab;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct Portfolio {
    portfolio: Vec<PortfolioPage>,
}
impl Portfolio {
    pub async fn new(file_path: &str) -> Self {
        let mut json_data = String::new();
        let _ = File::open(file_path)
            .expect("could not open file")
            .read_to_string(&mut json_data)
            .expect("could not parse json");
        let mut portfolio: Portfolio =
            serde_json::from_str(&json_data).expect("could not parse json");
        portfolio.portfolio.sort_by_key(|item| item.order);
        portfolio.merge_repo_data().await
    }
    async fn merge_repo_data(mut self) -> Self {
        let pat = std::env::var("GITHUB_API_PAT").expect("Github PAT is missing");
        let octocrab = Octocrab::builder().personal_token(pat).build().unwrap();
        for page in self.portfolio.iter_mut() {
            if page.is_github_project && page.github_repo_name.is_some() {
                let repo = octocrab
                    .clone()
                    .repos("tieje", page.github_repo_name.clone().unwrap())
                    .get()
                    .await
                    .expect("Querying Github repo failed");
                page.description = repo.description;
                page.project_url = Some(repo.url.as_str().to_string());
                page.topics = repo.topics;
                page.github_url = Some(repo.git_url.unwrap().as_str().to_string());
            }
        }
        self
    }
}

#[allow(dead_code)]
#[derive(Default, Debug, Deserialize)]
pub struct PortfolioPage {
    title: String,
    order: u32,
    is_github_project: bool,
    github_repo_name: Option<String>,
    description: Option<String>,
    project_url: Option<String>,
    github_url: Option<String>,
    topics: Option<Vec<String>>,
    workplace: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "output is too long"]
    async fn parse_local_json() {
        let portfolio = Portfolio::new("data.json").await;
        dbg!(portfolio);
    }
}
