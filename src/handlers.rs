use std::error::Error;

use regex::Regex;
use serde::{Deserialize, Serialize};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, Me},
};
use ureq::serde_json::{from_str, to_string, Value};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Suggest {
    text: String,
    text_marked: String,
    package_name: String,
    icon_url: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct SearchBody {
    suggests: Vec<Suggest>,
}
#[derive(Deserialize, Serialize, Debug)]
struct SearchResult {
    code: String,
    message: String,
    body: SearchBody,
    timestamp: String,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SuggestCallback {
    cmd: String,
    package: String,
}
//app info
#[derive(Deserialize, Serialize, Debug)]
pub struct GetAppResult {
    pub code: String,
    pub message: String,
    pub body: Body,
    pub timestamp: String,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub app_id: i64,
    pub apk_uid: String,
    pub package_name: String,
    pub app_name: String,
    pub category: Value,
    pub categories: Vec<String>,
    pub company_name: String,
    pub owner_vk_id: String,
    pub short_description: String,
    pub full_description: String,
    pub moder_info: Value,
    pub company_status: Value,
    pub app_status: Value,
    pub age_legal: String,
    pub file_size: i64,
    pub version_name: String,
    pub version_code: i64,
    pub min_sdk_version: i64,
    pub target_sdk_version: i64,
    pub max_sdk_version: i64,
    pub whats_new: String,
    pub icon_url: String,
    pub file_urls: Vec<FileUrl>,
    pub app_type: String,
    pub website: String,
    pub signature: String,
    pub company_legal_form: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileUrl {
    pub file_url: String,
    pub order: i64,
    pub ordinal: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub orientation: Value,
}

fn bytes_to_size(bytes: f64) -> String {
    let units = ["байт", "КБ", "МБ", "ГБ", "ТБ"];
    let unit = bytes.log(1024_f64).floor() as u32;
    println!("{}", unit);
    format!(
        "{} {}",
        (bytes as i64 / i64::pow(1024, unit as u32)),
        (units[unit as usize])
    )
}
fn make_keyboard(res: SearchResult) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for x in res.body.suggests {
        keyboard.push(vec![InlineKeyboardButton::callback(
            x.text.to_owned(),
            to_string(&SuggestCallback {
                cmd: "search".to_string(),
                package: x.package_name.to_string(),
            })
            .unwrap(),
        )]);
    }
    println!("{:?}", keyboard);
    InlineKeyboardMarkup::new(keyboard)
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if Regex::new(r"поиск (.*)")
        .unwrap()
        .is_match(msg.text().unwrap_or(""))
    {
        let text = Regex::new(r"поиск (.*)")
            .unwrap()
            .captures(msg.text().unwrap_or(""))
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        let resp = ureq::get("https://backapi.rustore.ru/search/suggest")
            .query("query", text)
            .call()
            .unwrap()
            .into_string()
            .unwrap();

        let res: SearchResult = from_str(&resp).unwrap();
        println!("{:?}", res);
        bot.send_message(msg.chat.id, format!("{}", text))
            .reply_markup(make_keyboard(res))
            .await?;
    }
    Ok(())
}

pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("{:?}", q);
    let cmd_data: SuggestCallback = from_str(q.data.as_deref().unwrap()).unwrap();
    println!("{:?}", cmd_data);
    bot.answer_callback_query(q.id).await?;
    if cmd_data.cmd == "search".to_string() {
        if let Some(Message { id, chat, .. }) = q.message {
            let resp = ureq::get(&format!(
                "https://backapi.rustore.ru/applicationData/overallInfo/{}",
                cmd_data.package
            ))
            .call()
            .unwrap()
            .into_string()
            .unwrap();
            let res: GetAppResult = from_str(&resp).unwrap();

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
    }

    Ok(())
}
