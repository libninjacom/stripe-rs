
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerBalanceCustomerBalanceSettings {
    pub reconciliation_mode: String,
    pub using_merchant_default: bool,
}
impl std::fmt::Display for CustomerBalanceCustomerBalanceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}