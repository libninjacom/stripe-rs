
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplePayDomain {
    pub created: i64,
    pub domain_name: String,
    pub id: String,
    pub livemode: bool,
    pub object: String,
}
impl std::fmt::Display for ApplePayDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}