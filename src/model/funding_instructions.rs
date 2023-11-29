
use serde::{Serialize, Deserialize};
use super::FundingInstructionsBankTransfer;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingInstructions {
    pub bank_transfer: FundingInstructionsBankTransfer,
    pub currency: String,
    pub funding_type: String,
    pub livemode: bool,
    pub object: String,
}
impl std::fmt::Display for FundingInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}