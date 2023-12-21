use std::{fs::File, io::Read};

use askama::Template;
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Portfolio {
    pages: Vec<PortfolioPage>,
}
impl Portfolio {
    pub fn new(file_path: &str) -> Self {
        let mut json_data = String::new();
        let _ = File::open(file_path)
            .expect("could not open file")
            .read_to_string(&mut json_data)
            .expect("could not parse json");
        let portfolio: Portfolio =
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
                page.image_url = Some(format!("/assets/{}.png", page.github_repo_name.clone().unwrap()));
            }
        }
        self
    }
    pub fn serialize(&self) -> PortfolioTemplate {
        let mut res = PortfolioTemplate { pages: Vec::new() };
        for page in self.pages.iter() {
            let new_page = PortfolioPageTemplate {
                title: page.title.clone(),
                order: page.order,
                is_github_project: page.is_github_project,
                image_url: page.image_url.clone().unwrap_or_default(),
                github_repo_name: page.github_repo_name.clone().unwrap_or_default(),
                description: page.description.clone().unwrap_or_default(),
                project_url: page.project_url.clone().unwrap_or_default(),
                github_url: page.github_url.clone().unwrap_or_default(),
                topics: page.topics.clone().unwrap_or_default(),
                workplace: page.workplace.clone().unwrap_or_default(),
                start_date: page.start_date.clone().unwrap_or_default(),
                end_date: page.end_date.clone().unwrap_or_default(),
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
    let mut portfolio: Portfolio =
        serde_json::from_str(&json_data).expect("could not parse json");
    portfolio.pages.sort_by_key(|item| item.order);
    let portfolio = portfolio.merge_repo_data().await;
    let file = File::create("data.json").expect("Failed to create file");
    serde_json::to_writer(file, &portfolio.serialize()).expect("Failed to write to file");
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default, Clone)]
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
    image_url: Option<String>,
}

#[derive(Template, Default, Serialize, Debug)]
#[template(path = "pages/portfolio.html")]
pub struct PortfolioTemplate {
    pages: Vec<PortfolioPageTemplate>,
}

#[derive(Template, Default, Serialize, Debug)]
#[template(path = "components/portfolio/page.html")]
pub struct PortfolioPageTemplate {
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
    image_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "output is too long"]
    fn parse_local_json() {
        let portfolio = Portfolio::new("data.json");
        dbg!(portfolio);
    }
    #[test]
    #[ignore = "output is too long"]
    fn serialize_struct() {
        let portfolio = Portfolio::new("data.json");
        dbg!(portfolio.serialize());
    }
    #[tokio::test]
    // #[ignore = "sync only when desired"]
    async fn push_portfolio_json_to_data_json() {
        sync_portfolio_json().await;
    }
}
