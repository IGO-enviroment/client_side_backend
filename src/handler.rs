use axum::{
    extract::{Query},
    response::IntoResponse,
    Json
};

use serde::{Deserialize, Serialize};

use crate::global_search::{GlobalSearchOperation};

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
    Query(conditions): Query<QuerySearch>
) -> impl IntoResponse {
    let global_search_operation = GlobalSearchOperation::init(String::from(""), conditions);

    let result = match global_search_operation.call() {
        Ok(response) => Json(response),
        Err(e) => Json(e),
    };

    result
}
