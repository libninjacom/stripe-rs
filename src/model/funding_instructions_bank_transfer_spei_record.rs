use serde::{Serialize, Deserialize};
///SPEI Records contain Mexico bank account details per the SPEI format.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferSpeiRecord {
    ///The three-digit bank code
    pub bank_code: String,
    ///The short banking institution name
    pub bank_name: String,
    ///The CLABE number
    pub clabe: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferSpeiRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}