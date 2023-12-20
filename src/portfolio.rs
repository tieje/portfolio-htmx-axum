use std::{fs::File, io::Read};

use askama::Template;
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};

#[derive(Template, Debug, Deserialize, Default, Clone)]
#[template(path = "pages/portfolio.html")]
pub struct PortfolioTemplate {
    pages: Vec<PortfolioPageTemplate>,
}
impl PortfolioTemplate {
    pub fn new(file_path: &str) -> Self {
        let mut json_data = String::new();
        let _ = File::open(file_path)
            .expect("could not open file")
            .read_to_string(&mut json_data)
            .expect("could not parse json");
        let portfolio: PortfolioTemplate =
            serde_json::from_str(&json_data).expect("could not parse json");
        portfolio
    }
    async fn merge_repo_data(mut self) -> Self {
        let pat = std::env::var("GITHUB_API_PAT").expect("Github PAT is missing");
        let octocrab = Octocrab::builder().personal_token(pat).build().unwrap();
        for page in self.pages.iter_mut() {
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
    pub fn serialize(&self) -> PortfolioJson {
        let mut res = PortfolioJson { pages: Vec::new() };
        for page in self.pages.iter() {
            let new_page = PortfolioPageJson {
                title: page.title.clone(),
                order: page.order,
                is_github_project: page.is_github_project,
                github_repo_name: page.github_repo_name.clone().unwrap_or(Default::default()),
                description: page.description.clone().unwrap_or(Default::default()),
                project_url: page.project_url.clone().unwrap_or(Default::default()),
                github_url: page.github_url.clone().unwrap_or(Default::default()),
                topics: page.topics.clone().unwrap_or(Default::default()),
                workplace: page.workplace.clone().unwrap_or(Default::default()),
                start_date: page.start_date.clone().unwrap_or(Default::default()),
                end_date: page.end_date.clone().unwrap_or(Default::default()),
            };
            res.pages.push(new_page)
        }
        res
    }
}

#[derive(Template)]
#[template(path = "pages/sync.html")]
pub struct SyncTemplate;
pub async fn sync_portfolio_json() {
    let mut json_data = String::new();
    let _ = File::open("portfolio.json")
        .expect("could not open file")
        .read_to_string(&mut json_data)
        .expect("could not parse json");
    let mut portfolio: PortfolioTemplate =
        serde_json::from_str(&json_data).expect("could not parse json");
    portfolio.pages.sort_by_key(|item| item.order);
    let portfolio = portfolio.merge_repo_data().await;
    let file = File::create("data.json").expect("Failed to create file");
    serde_json::to_writer(file, &portfolio.serialize()).expect("Failed to write to file");
}

#[allow(dead_code)]
#[derive(Template, Debug, Deserialize, Default, Clone)]
#[template(path = "components/portfolio/page.html")]
pub struct PortfolioPageTemplate {
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

#[derive(Default, Serialize, Debug)]
pub struct PortfolioJson {
    pages: Vec<PortfolioPageJson>,
}

#[derive(Default, Serialize, Debug)]
pub struct PortfolioPageJson {
    title: String,
    order: u32,
    is_github_project: bool,
    github_repo_name: String,
    description: String,
    project_url: String,
    github_url: String,
    topics: Vec<String>,
    workplace: String,
    start_date: String,
    end_date: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "output is too long"]
    fn parse_local_json() {
        let portfolio = PortfolioTemplate::new("data.json");
        dbg!(portfolio);
    }
    #[test]
    #[ignore = "output is too long"]
    fn serialize_struct() {
        let portfolio = PortfolioTemplate::new("data.json");
        dbg!(portfolio.serialize());
    }
    #[tokio::test]
    #[ignore = "sync only when desired"]
    async fn push_portfolio_json_to_data_json() {
        sync_portfolio_json().await;
    }
}
