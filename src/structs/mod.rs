use serde::{Deserialize, Serialize};

pub mod app_info;
pub mod search_result;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CallbackCommand {
    pub cmd: String,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SuggestCallback {
    pub cmd: String,
    pub package: String,
}
