use askama::Template;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(hello))
        .route("/click", get(click));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "pages/hello.html")]
struct HelloTemplate;

#[derive(Template)]
#[template(path = "pages/click.html")]
struct ClickTemplate;

async fn hello() -> HelloTemplate {
    HelloTemplate {}
}
async fn click() -> ClickTemplate {
    ClickTemplate {}
}
