
use serde::{Serialize, Deserialize};
use super::PromotionCode;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromotionCodesResourcePromotionCodeList {
    pub data: Vec<PromotionCode>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for PromotionCodesResourcePromotionCodeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}