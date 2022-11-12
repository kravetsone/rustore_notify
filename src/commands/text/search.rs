use std::error::Error;

use regex::Regex;
use teloxide::{payloads::SendMessageSetters, requests::Requester, types::Message, Bot};

use crate::{helpers::make_keyboard, structs::SearchResult};

pub async fn search(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let text = Regex::new(r"поиск (.*)")
        .unwrap()
        .captures(msg.text().unwrap_or(""))
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let res = reqwest::Client::new()
        .get("https://backapi.rustore.ru/search/suggest")
        .query(&[("query", text)])
        .send()
        .await?
        .json::<SearchResult>()
        .await?;

    println!("{:?}", res);
    bot.send_message(msg.chat.id, format!("{}", text))
        .reply_markup(make_keyboard(res))
        .await?;
    Ok(())
}
