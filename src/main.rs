use std::env::current_dir;

use askama::Template;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let assets_path = current_dir().unwrap();
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(home))
        .route("/portfolio", get(portfolio))
        .route("/click", get(click))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "pages/portfolio.html")]
struct PortfolioTemplate;

async fn portfolio() -> PortfolioTemplate {
    PortfolioTemplate {}
}
#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate;
async fn home() -> HomeTemplate {
    HomeTemplate {}
}

#[derive(Template)]
#[template(path = "pages/hello.html")]
struct HelloTemplate;
async fn hello() -> HelloTemplate {
    HelloTemplate {}
}

#[derive(Template)]
#[template(path = "pages/click.html")]
struct ClickTemplate;
async fn click() -> ClickTemplate {
    ClickTemplate {}
}

