use serde::{Serialize, Deserialize};
///SWIFT Records contain U.S. bank account details per the SWIFT format.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferSwiftRecord {
    ///The account number
    pub account_number: String,
    ///The bank name
    pub bank_name: String,
    ///The SWIFT code
    pub swift_code: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferSwiftRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}