use serde::{Serialize, Deserialize};
use super::ShippingRateFixedAmount;
/**Shipping rates describe the price of shipping presented to your customers and
applied to a purchase. For more information, see [Charge for shipping](https://stripe.com/docs/payments/during-payment/charge-shipping).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShippingRate {
    ///Whether the shipping rate can be used for new purchases. Defaults to `true`.
    pub active: bool,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The estimated range for how long shipping will take, meant to be displayable to the customer. This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<serde_json::Value>,
    ///The name of the shipping rate, meant to be displayable to the customer. This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<ShippingRateFixedAmount>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Specifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<String>,
    ///A [tax code](https://stripe.com/docs/tax/tax-categories) ID. The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<serde_json::Value>,
    ///The type of calculation to use on the shipping rate. Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for ShippingRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}