
use serde::{Serialize, Deserialize};
use super::FundingInstructionsBankTransferFinancialAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransfer {
    pub country: String,
    pub financial_addresses: Vec<FundingInstructionsBankTransferFinancialAddress>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for FundingInstructionsBankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}