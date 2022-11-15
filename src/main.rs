#[macro_use]
extern crate lazy_static;

use teloxide::prelude::*;

mod commands;
mod helpers;
mod structs;

use crate::commands::callback::callback_handler;
use crate::commands::text::message_handler;

#[tokio::main]
async fn main() {
    let bot = Bot::new("5636771268:AAFF1qdkZi4QDf5Hj209-xopNbITCseR75c");
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![{}])
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    println!("[RuStore] Bot was started!");
}
