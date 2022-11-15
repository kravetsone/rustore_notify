use crate::structs::{search_result::SearchResult, SuggestCallback};

use serde_json::to_string;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
pub fn make_keyboard(res: SearchResult) -> InlineKeyboardMarkup {
    let keyboard = res
        .body
        .suggests
        .into_iter()
        .map(|x| {
            vec![InlineKeyboardButton::callback(
                x.text.to_owned(),
                to_string(&SuggestCallback {
                    cmd: "search".to_string(),
                    package: x.package_name.to_string(),
                })
                .unwrap(),
            )]
        })
        .collect::<Vec<_>>();

    println!("{:?}", keyboard);
    InlineKeyboardMarkup::new(keyboard)
}
const SUFFIX: [&str; 5] = ["байт", "КБ", "МБ", "ГБ", "ТБ"];
const UNIT: f64 = 1024.0;

pub fn format_bytes(bytes: f64) -> String {
    if bytes <= 0.0 {
        return "0 байт".to_owned();
    }

    let base = bytes.log10() / UNIT.log10();

    let num = format!("{:.1}", UNIT.powf(base - base.floor()))
        .trim_end_matches(".0")
        .to_owned();

    format!("{} {}", num, SUFFIX[base.floor() as usize])
}
