use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::path::Path;
use axum::response::{Redirect};
use axum::routing::get_service;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/static", get(|| async { Redirect::permanent("/static/") }))
        .nest(
            "/static/",
            static_router("static"),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 20921));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}


pub fn static_router<P: AsRef<Path>>(path: P) -> Router {
    let serve_dir = get_service(ServeDir::new(path).append_index_html_on_directories(true));

    let router = Router::new()
        .fallback_service(serve_dir);

    router
}