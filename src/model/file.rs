
use serde::{Serialize, Deserialize};
use super::FileResourceFileLinkList;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct File {
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<FileResourceFileLinkList>,
    pub object: String,
    pub purpose: String,
    pub size: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}