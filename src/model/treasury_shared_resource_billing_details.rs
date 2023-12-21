use serde::{Serialize, Deserialize};
use super::Address;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasurySharedResourceBillingDetails {
    ///
    pub address: Address,
    ///Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for TreasurySharedResourceBillingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}