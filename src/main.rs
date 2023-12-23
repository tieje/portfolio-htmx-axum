pub mod portfolio;

use std::env::current_dir;

use axum::{routing::get, Router};
use portfolio::{sync_portfolio_json, Portfolio, PortfolioTemplate, SyncTemplate};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let assets_path = format!("{}/assets", current_dir().unwrap().to_str().unwrap());
    let app = Router::new()
        .route("/", get(portfolio_handler))
        .route("/portfolio", get(portfolio_handler))
        .route("/sync", get(sync_handler))
        .nest_service("/assets", ServeDir::new(assets_path));

    // let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn portfolio_handler() -> PortfolioTemplate {
    Portfolio::new("data.json").serialize()
}
async fn sync_handler() -> SyncTemplate {
    sync_portfolio_json().await;
    SyncTemplate {}
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
