use serde::{Serialize, Deserialize};
///Sort Code Records contain U.K. bank account details per the sort code format.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferSortCodeRecord {
    ///The name of the person or business that owns the bank account
    pub account_holder_name: String,
    ///The account number
    pub account_number: String,
    ///The six-digit sort code
    pub sort_code: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferSortCodeRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}