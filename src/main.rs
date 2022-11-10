use teloxide::prelude::*;

mod handlers;

use crate::handlers::{callback_handler, message_handler};

#[tokio::main]
async fn main() {
    let bot = Bot::new("");
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
