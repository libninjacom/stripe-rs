
use serde::{Serialize, Deserialize};
use super::SourceTransaction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApmsSourcesSourceTransactionList {
    pub data: Vec<SourceTransaction>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for ApmsSourcesSourceTransactionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}