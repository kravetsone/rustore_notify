use std::error::Error;

use regex::Captures;
use teloxide::{payloads::SendMessageSetters, requests::Requester, types::Message, Bot};

use crate::{helpers::make_keyboard, structs::search_result::SearchResult};

pub async fn search(
    bot: Bot,
    msg: Message,
    args: Captures<'_>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let query = args.get(1).unwrap().as_str();

    let res = reqwest::Client::new()
        .get("https://backapi.rustore.ru/search/suggest")
        .query(&[("query", query)])
        .send()
        .await?
        .json::<SearchResult>()
        .await?;

    println!("{:?}", res);
    bot.send_message(msg.chat.id, format!("{}", query))
        .reply_markup(make_keyboard(res))
        .await?;
    Ok(())
}
