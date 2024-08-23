mod handler;
mod global_search;

use axum::{
    extract::{Path, Query},
    routing::{get},
    Router,
};
use serde::Deserialize;

use crate::{
    handler::global_search_handler,
};

// A struct for query parameters
#[derive(Deserialize)]
struct Page {
    number: Option<u64>,
}

pub struct AppState {
}

// A handler to demonstrate path and query extractors
async fn show_item(Path(id): Path<u32>, Query(page): Query<Page>) -> String {
    format!("Item {} on page {}", id, page.number.unwrap_or(1))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/item/:id", get(show_item))
        .route("/test", get(global_search_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
