
use serde::{Serialize, Deserialize};
use super::TreasuryInboundTransfer;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryInboundTransfersResourceInboundTransferList {
    pub data: Vec<TreasuryInboundTransfer>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TreasuryInboundTransfersResourceInboundTransferList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}