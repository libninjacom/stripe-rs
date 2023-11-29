
use serde::{Serialize, Deserialize};
use super::IssuingSettlement;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingSettlementList {
    pub data: Vec<IssuingSettlement>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for IssuingSettlementList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}