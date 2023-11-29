
use serde::{Serialize, Deserialize};
use super::ApplePayDomain;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplePayDomainList {
    pub data: Vec<ApplePayDomain>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for ApplePayDomainList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}