use serde::{Serialize, Deserialize};
use super::CustomerBalanceCustomerBalanceSettings;
///A customer's `Cash balance` represents real funds. Customers can add funds to their cash balance by sending a bank transfer. These funds can be used for payment and can eventually be paid out to your bank account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CashBalance {
    ///A hash of all cash balances available to this customer. You cannot delete a customer with any cash balances, even if the balance is 0. Amounts are represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<serde_json::Value>,
    ///The ID of the customer whose cash balance this object represents.
    pub customer: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///
    pub settings: CustomerBalanceCustomerBalanceSettings,
}
impl std::fmt::Display for CashBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}