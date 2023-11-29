
use serde::{Serialize, Deserialize};
use super::File;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileResourceFileList {
    pub data: Vec<File>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for FileResourceFileList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}