use regex::Regex;

use std::error::Error;
use teloxide::prelude::*;

pub mod search;
use search::search;
pub async fn message_handler(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if Regex::new(r"поиск (.*)")
        .unwrap()
        .is_match(msg.text().unwrap_or(""))
    {
        search(bot, msg).await?;
    }
    Ok(())
}
