mod config;
mod handlers;
mod operations;
mod api;

use std::sync::Arc;
use crate::{
    config::application::Application,
    config::routes::create_router,
};

#[tokio::main]
async fn main() {
    let app = match Application::init() {
        Ok(app) => Arc::new(app),
        Err(err) => panic!("Ошибка запуска приложения: {}", err)
    };

    let routes = create_router(app.clone()); 

    app.run(routes).await;
}
