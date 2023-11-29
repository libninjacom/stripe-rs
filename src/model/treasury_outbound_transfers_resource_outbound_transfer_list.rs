
use serde::{Serialize, Deserialize};
use super::TreasuryOutboundTransfer;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryOutboundTransfersResourceOutboundTransferList {
    pub data: Vec<TreasuryOutboundTransfer>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TreasuryOutboundTransfersResourceOutboundTransferList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}