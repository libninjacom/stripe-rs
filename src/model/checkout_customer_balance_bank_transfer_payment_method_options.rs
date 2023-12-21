use serde::{Serialize, Deserialize};
use super::PaymentMethodOptionsCustomerBalanceEuBankAccount;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<PaymentMethodOptionsCustomerBalanceEuBankAccount>,
    /**List of address types that should be returned in the financial_addresses response. If not specified, all valid types will be returned.

Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<Vec<String>>,
    ///The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}