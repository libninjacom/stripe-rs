use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodPaypal {
    /**Owner's email. Values are provided by PayPal directly
(if supported) at the time of authorization or settlement. They cannot be set or mutated.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_email: Option<String>,
    ///PayPal account PayerID. This identifier uniquely identifies the PayPal customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_id: Option<String>,
}
impl std::fmt::Display for PaymentMethodPaypal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}