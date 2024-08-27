#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url = String::from("postgres://user:pass@localhost:5432/postgres"); // std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        // let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        // let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");

        // let smtp_host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        // let smtp_port = std::env::var("SMTP_PORT").expect("SMTP_PORT must be set");
        // let smtp_user = std::env::var("SMTP_USER").expect("SMTP_USER must be set");
        // let smtp_pass = std::env::var("SMTP_PASS").expect("SMTP_PASS must be set");
        // let smtp_from = std::env::var("SMTP_FROM").expect("SMTP_FROM must be set");

        // let frontend_origin =
        //     std::env::var("FRONTEND_ORIGIN").expect("FRONTEND_ORIGIN must be set");

        return Config {
            database_url: database_url.clone()
        };
    }
}
