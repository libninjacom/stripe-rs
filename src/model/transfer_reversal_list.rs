
use serde::{Serialize, Deserialize};
use super::TransferReversal;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferReversalList {
    pub data: Vec<TransferReversal>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TransferReversalList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}