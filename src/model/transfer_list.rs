
use serde::{Serialize, Deserialize};
use super::Transfer;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferList {
    pub data: Vec<Transfer>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TransferList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}