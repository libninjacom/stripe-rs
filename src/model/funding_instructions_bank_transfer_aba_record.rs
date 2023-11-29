
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferAbaRecord {
    pub account_number: String,
    pub bank_name: String,
    pub routing_number: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferAbaRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}