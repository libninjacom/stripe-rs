
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferSwiftRecord {
    pub account_number: String,
    pub bank_name: String,
    pub swift_code: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferSwiftRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}