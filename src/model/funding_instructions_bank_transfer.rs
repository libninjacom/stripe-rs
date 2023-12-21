use serde::{Serialize, Deserialize};
use super::FundingInstructionsBankTransferFinancialAddress;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransfer {
    ///The country of the bank account to fund
    pub country: String,
    ///A list of financial addresses that can be used to fund a particular balance
    pub financial_addresses: Vec<FundingInstructionsBankTransferFinancialAddress>,
    ///The bank_transfer type
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for FundingInstructionsBankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}