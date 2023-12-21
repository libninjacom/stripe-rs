use serde::{Serialize, Deserialize};
///ABA Records contain U.S. bank account details per the ABA format.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferAbaRecord {
    ///The ABA account number
    pub account_number: String,
    ///The bank name
    pub bank_name: String,
    ///The ABA routing number
    pub routing_number: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferAbaRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}