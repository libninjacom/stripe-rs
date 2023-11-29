
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferSpeiRecord {
    pub bank_code: String,
    pub bank_name: String,
    pub clabe: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferSpeiRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}