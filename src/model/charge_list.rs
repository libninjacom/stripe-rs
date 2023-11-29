
use serde::{Serialize, Deserialize};
use super::Charge;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChargeList {
    pub data: Vec<Charge>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for ChargeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}