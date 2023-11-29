
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferIbanRecord {
    pub account_holder_name: String,
    pub bic: String,
    pub country: String,
    pub iban: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferIbanRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}