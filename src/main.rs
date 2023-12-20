pub mod portfolio;

use std::env::current_dir;

use askama::Template;
use axum::{routing::get, Router};
use portfolio::{PortfolioTemplate, sync_portfolio_json, SyncTemplate};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let assets_path = format!("{}/assets", current_dir().unwrap().to_str().unwrap());
    let app = Router::new()
        .route("/", get(home))
        .route("/portfolio", get(portfolio_handler))
        .route("/sync", get(sync_handler))
        .route("/click", get(click))
        .nest_service("/assets", ServeDir::new(assets_path));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn portfolio_handler() -> PortfolioTemplate {
    PortfolioTemplate::new("data.json")
}
async fn sync_handler() -> SyncTemplate {
    sync_portfolio_json().await;
    SyncTemplate {}
}

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate;
async fn home() -> HomeTemplate {
    HomeTemplate {}
}

// #[derive(Template)]
// #[template(path = "pages/hello.html")]
// struct HelloTemplate;
// async fn hello() -> HelloTemplate {
//     HelloTemplate {}
// }

#[derive(Template)]
#[template(path = "pages/click.html")]
struct ClickTemplate;
async fn click() -> ClickTemplate {
    ClickTemplate {}
}

#[cfg(test)]
mod tests {
    use octocrab::Octocrab;

    #[tokio::test]
    #[ignore = "works. No need to test"]
    async fn get_repo() {
        let pat = std::env::var("GITHUB_API_PAT").expect("Github PAT is missing");
        let octocrab = Octocrab::builder().personal_token(pat).build().unwrap();
        let repo = octocrab.repos("tieje", "portfolio-htmx-axum").get().await;
        match repo {
            Err(e) => println!("{}", e),
            Ok(k) => println!("{:#?}", k.description.unwrap()),
        }
    }
}
