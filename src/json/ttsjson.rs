use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
    pub extra: Extra,
    pub message: String,
    #[serde(rename = "status_code")]
    pub status_code: i64,
    #[serde(rename = "status_msg")]
    pub status_msg: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "s_key")]
    pub s_key: String,
    #[serde(rename = "v_str")]
    pub v_str: String,
    pub duration: String,
    pub speaker: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extra {
    #[serde(rename = "log_id")]
    pub log_id: String,
}