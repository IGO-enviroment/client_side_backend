use teloxide::prelude::*;

/// Отправка алертов, ошибок.
pub mod alerts {
    use create::config::Config;

    pub async fn send_message(bot_instance: Option<Bot>, message: String) -> Result<(), Error> {
        let bot = match bot_instance {
            Some(bot) => bot,
            None => Bot::from_env()
        };
        bot.send_message(Config::exception_chat_id(), message).await?
    }
}
