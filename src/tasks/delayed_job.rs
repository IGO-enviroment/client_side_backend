use celery::prelude::*;
use celery::error::CeleryError;

const DEFAULT_QUEUE_NAME: &str = "default_delayed";

pub async fn start_jobs() -> Result<(), CeleryError> {
    let my_app = celery::app!(
        broker = RedisBroker { std::env::var("REDIS_ADDR").unwrap_or_else(|_| "redis://127.0.0.1:6379/".into()) },
        tasks = [
            send_email_code
        ],
        task_routes = [
            "*" => DEFAULT_QUEUE_NAME,
        ],
        heartbeat = Some(10),
    ).await?;

    my_app.display_pretty().await;
    my_app.consume_from(&[DEFAULT_QUEUE_NAME]).await?;

    my_app.close().await?;

    Ok(())
}


#[celery::task(max_retries = 2)]
async fn send_email_code(email: String, code: String) -> TaskResult<()> {
    println!("email {email:?} {code:?}");

    Ok(())
}

#[celery::task(max_retries = 2)]
async fn send_order_info_after_success(order_id: u64) -> TaskResult<()> {
    println!("email {order_id:?}");

    Ok(())
}
