use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerBalanceCustomerBalanceSettings {
    ///The configuration for how funds that land in the customer cash balance are reconciled.
    pub reconciliation_mode: String,
    ///A flag to indicate if reconciliation mode returned is the user's default or is specific to this customer cash balance
    pub using_merchant_default: bool,
}
impl std::fmt::Display for CustomerBalanceCustomerBalanceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}