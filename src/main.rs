mod config;
mod handlers;
mod operations;
mod models;
mod api;
mod tasks;

use std::sync::Arc;
use crate::{
    config::application::Application,
    config::routes::create_router,
};

const ARG_WEB_APP: &str = "web_app";
const ARG_TASKS_APP: &str = "tasks_app";
const ARG_SCHELDUE_APP: &str = "schedule_app";

#[tokio::main]
async fn main() {
    let args = std::env::var("JWT_SECRET").expect("");
    match args.as_str() {
        ARG_WEB_APP => run_web_app().await,
        ARG_TASKS_APP => run_tasks_app().await,
        ARG_SCHELDUE_APP => run_scheldue_app().await,
        _ => panic!("Не указан параметр указывающий на тип запускаемого приложения.")
    }
}

async fn run_web_app() {
    let app: Arc<Application> = match Application::init() {
        Ok(app) => Arc::new(app),
        Err(err) => panic!("Ошибка запуска приложения: {}", err)
    };

    let routes = create_router(app.clone()); 

    app.run(routes).await;
}

async fn run_tasks_app() {
    match tasks::delayed_job::start_jobs().await  {
        Ok(_) => (),
        Err(e) => panic!("Ошибка {e}")
    }
}

async fn run_scheldue_app() {
    match tasks::schedule::start_schedule().await  {
        Ok(_) => (),
        Err(e) => panic!("Ошибка {e}")
    }
}
