use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub website: Value,
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
