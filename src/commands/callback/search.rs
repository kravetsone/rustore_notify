use std::error::Error;

use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile},
};

use crate::helpers::bytes_to_size;
use crate::structs::{GetAppResult, SuggestCallback};

pub async fn search(
    bot: Bot,
    q: CallbackQuery,
    cmd_data: SuggestCallback,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(Message { id, chat, .. }) = q.message {
        bot.answer_callback_query(q.id).await?;
        let res = reqwest::get(&format!(
            "https://backapi.rustore.ru/applicationData/overallInfo/{}",
            cmd_data.package
        ))
        .await?
        .json::<GetAppResult>()
        .await?;

        bot.edit_message_text(
            chat.id,
            id,
            format!(
                "Название: {}\nИмя пакета: {}\nВерсия: {}\n{}",
                res.body.app_name,
                cmd_data.package,
                res.body.version_name,
                (if res.body.file_size < 52_428_800 {
                    "Ожидайте загрузки apk-файла в телеграм..."
                } else {
                    ""
                })
            ),
        )
        .reply_markup(InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::url(
                format!("Скачать ({})", bytes_to_size(res.body.file_size as f64)),
                reqwest::Url::parse(
                    ("https://static.rustore.ru/".to_owned() + &res.body.apk_uid).as_str(),
                )
                .unwrap(),
            ),
        ]]))
        .await?;

        if res.body.file_size < 52_428_800 {
            println!("меньше 50 мб можно грузить");
            let body = reqwest::get(
                ("https://static.rustore.ru/".to_owned() + &res.body.apk_uid).as_str(),
            )
            .await?
            .bytes()
            .await?;
            bot.send_document(
                chat.id,
                InputFile::memory(body)
                    .file_name(res.body.package_name + "-" + &res.body.version_name + ".apk"),
            )
            .await?;
        }
    }
    Ok(())
}
