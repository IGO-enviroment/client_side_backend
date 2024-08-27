
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use http::{
    Method
};

use crate::config::application::Application;
use crate::handlers::{
    global_search::global_search_handler,
    billing::{callbacks_notify_payment, create_order_handler},
};

/// Инициализация всех роутов приложения.
pub fn create_router(app_state: Arc<Application>) -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    return Router::new()
        .route("/test", get(global_search_handler))
        .route("/billing/callbacks", get(callbacks_notify_payment))
        .route("/billing/callbacks", post(callbacks_notify_payment))
        .route("/order", get(create_order_handler))
        .layer(cors)
        .with_state(app_state);
}
