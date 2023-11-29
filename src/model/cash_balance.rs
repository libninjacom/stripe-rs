
use serde::{Serialize, Deserialize};
use super::CustomerBalanceCustomerBalanceSettings;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashBalance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<serde_json::Value>,
    pub customer: String,
    pub livemode: bool,
    pub object: String,
    pub settings: CustomerBalanceCustomerBalanceSettings,
}
impl std::fmt::Display for CashBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}