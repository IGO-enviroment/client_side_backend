use std::sync::Arc;

use axum::{
    extract::{Query, State}, http::{Response, StatusCode}, response::IntoResponse, Json
};
use serde::{Deserialize, Serialize};
use crate::{config::application::Application, operations::global_search::GlobalSearchOperation};

#[derive(Deserialize)]
pub struct QuerySearch {
    pub q: Option<String>,
    pub page: Option<u32>,
}

#[derive(Serialize)]
pub struct ResultSearch {
    total: u32,
    results: Vec<(String, String, String)>,
}

// Глобавльный поиск по всему контенту приложения.
pub async fn global_search_handler(
    Query(conditions): Query<QuerySearch>,
    State(app): State<Arc<Application>>
) -> impl IntoResponse {
    let mut global_search_operation = GlobalSearchOperation::init(app, conditions);

    match global_search_operation.call().await {
        Ok(response) => {
            Response::builder()
                .status(StatusCode::OK)
                .body(Json(response))
                .unwrap();
        },
        Err(e) => {
            Response::builder()
                .status(StatusCode::OK)
                .body(Json(e))
                .unwrap();
        },
    };
}
