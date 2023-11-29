
use serde::{Serialize, Deserialize};
use super::PaymentMethodOptionsCustomerBalanceEuBankAccount;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<PaymentMethodOptionsCustomerBalanceEuBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}