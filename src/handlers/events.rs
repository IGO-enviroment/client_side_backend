use std::sync::Arc;

use axum::{
    extract::{Query, State}, http::{Response, StatusCode}, response::IntoResponse, Json
};
use serde::{Deserialize, Serialize};
use crate::{config::application::Application, operations::global_search::GlobalSearchOperation};

#[derive(Deserialize)]
pub struct QuerySearch {
    pub q: Option<String>,
    pub date_start: Option<String>,
    pub date_end: Option<String>,
    pub tags: Option<Vec<String>>,
    pub types: Option<Vec<String>>,
    pub areas: Option<Vec<String>>,
    pub price: Option<Vec<String>>, 
    pub page: Option<u32>,
}

// #[derive(Serialize)]
// pub struct ResultSearch {
//     total: u32,
//     results: Vec<(String, String, String)>,
// }

pub async fn events_handler(
    Query(conditions): Query<QuerySearch>,
    State(app): State<Arc<Application>>
) -> impl IntoResponse {
    Json(());
}
