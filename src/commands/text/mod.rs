use regex::Regex;

use std::error::Error;
use teloxide::prelude::*;

pub mod search;
use search::search;
lazy_static! {
    static ref SEARCH_REGEX: Regex = Regex::new(r"поиск (.*)").unwrap();
}
pub async fn message_handler(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let text = msg.text().unwrap_or("").to_owned();

    if SEARCH_REGEX.is_match(&text) {
        search(bot, msg, SEARCH_REGEX.captures(&text).unwrap()).await?;
    }
    Ok(())
}
