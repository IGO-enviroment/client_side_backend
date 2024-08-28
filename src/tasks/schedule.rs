use celery::prelude::*;
use celery::beat::CronSchedule;
use celery::error::BeatError;

const DEFAULT_QUEUE_NAME: &str = "default_scheldue";

pub async fn start_schedule() -> Result<(), BeatError> {
    let mut beat = celery::beat!(
        broker = RedisBroker { std::env::var("REDIS_ADDR").unwrap_or_else(|_| "redis://127.0.0.1:6379/".into()) },
        tasks = [
            "check_awaiting_orders" => {
                check_awaiting_orders,
                schedule = CronSchedule::from_string("*/2 * * * *")?,
                args = (),
            }
        ],
        task_routes = [
            "*" => DEFAULT_QUEUE_NAME,
        ],
    ).await?;

    beat.start().await?;

    Ok(())
}


#[celery::task]
async fn check_awaiting_orders() -> TaskResult<()> {
    println!("Orders");

    Ok(())
}
