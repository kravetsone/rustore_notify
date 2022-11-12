use crate::structs::{SearchResult, SuggestCallback};

use serde_json::to_string;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
pub fn make_keyboard(res: SearchResult) -> InlineKeyboardMarkup {
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
pub fn bytes_to_size(bytes: f64) -> String {
    let units = ["байт", "КБ", "МБ", "ГБ", "ТБ"];
    let unit = bytes.log(1024_f64).floor() as u32;
    println!("{}", unit);
    format!(
        "{} {}",
        (bytes as i64 / i64::pow(1024, unit as u32)),
        (units[unit as usize])
    )
}
