
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferSortCodeRecord {
    pub account_holder_name: String,
    pub account_number: String,
    pub sort_code: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferSortCodeRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}