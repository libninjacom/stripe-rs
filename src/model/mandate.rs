use serde::{Serialize, Deserialize};
use super::{
    CustomerAcceptance, MandateMultiUse, MandatePaymentMethodDetails, MandateSingleUse,
};
///A Mandate is a record of the permission that your customer gives you to debit their payment method.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Mandate {
    ///
    pub customer_acceptance: CustomerAcceptance,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<MandateMultiUse>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The account (if any) that the mandate is intended for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    ///ID of the payment method associated with this mandate.
    pub payment_method: serde_json::Value,
    ///
    pub payment_method_details: MandatePaymentMethodDetails,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use: Option<MandateSingleUse>,
    ///The mandate status indicates whether or not you can use it to initiate a payment.
    pub status: String,
    ///The type of the mandate.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for Mandate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}