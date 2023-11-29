
use serde::{Serialize, Deserialize};
use super::FileLink;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileResourceFileLinkList {
    pub data: Vec<FileLink>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for FileResourceFileLinkList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}