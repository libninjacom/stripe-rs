
use serde::{Serialize, Deserialize};
use super::Address;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingCardholderAddress {
    pub address: Address,
}
impl std::fmt::Display for IssuingCardholderAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}