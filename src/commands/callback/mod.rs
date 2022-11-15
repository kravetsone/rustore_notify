use serde_json::from_str;

use std::error::Error;
use teloxide::prelude::*;

pub mod search;
use search::search;

use crate::structs::SuggestCallback;
pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("{:?}", q);
    let cmd_data: SuggestCallback = from_str(&q.to_owned().data.unwrap()).unwrap();
    println!("{:?}", cmd_data);
    if cmd_data.cmd == "search" {
        search(bot, q, cmd_data).await?;
    }

    Ok(())
}
