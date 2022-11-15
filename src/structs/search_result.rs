use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Suggest {
    pub text: String,
    pub text_marked: String,
    pub package_name: String,
    pub icon_url: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SearchBody {
    pub suggests: Vec<Suggest>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SearchResult {
    pub code: String,
    pub message: String,
    pub body: SearchBody,
    pub timestamp: String,
}
