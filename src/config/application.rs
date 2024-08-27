use axum::Router;
use deadpool_diesel::postgres::{Pool, Manager};
use elasticsearch::{
    Elasticsearch,
    http::transport::Transport
};

use crate::config::config::Config;

pub struct Application {
    pub db_pool: deadpool_diesel::postgres::Pool,
    pub client_elastic: Elasticsearch,
    pub redis: String,
    pub config: Config,
}

impl Application {
    pub fn init() -> Result<Application, String> {
        let config = Config::init();

        // set up connection pool
        let manager = deadpool_diesel::postgres::Manager::new(
            config.database_url.clone(), deadpool_diesel::Runtime::Tokio1
        );
        let pool = deadpool_diesel::postgres::Pool::builder(manager)
            .build()
            .unwrap();

        let transport = Transport::single_node("http://localhost:9200");
        let elastic = match transport {
            Ok(transport) => Elasticsearch::new(transport),
            Err(err) => panic!("Error creating elasticsearch {}", err)
        };

        Ok(
            Application {
                client_elastic: elastic.clone(),
                db_pool: pool.clone(),
                redis: String::new(),
                config: config,
            }
        )
    }

    /// Запуск приложения.
    pub async fn run(&self, app: Router) {
        println!("Starting app");
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    fn init_logger(&self) {
    }
}
