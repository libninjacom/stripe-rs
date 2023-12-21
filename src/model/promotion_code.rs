use serde::{Serialize, Deserialize};
use super::{Coupon, PromotionCodesResourceRestrictions};
/**A Promotion Code represents a customer-redeemable code for a [coupon](https://stripe.com/docs/api#coupons). It can be used to
create multiple codes for a single coupon.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromotionCode {
    ///Whether the promotion code is currently active. A promotion code is only active if the coupon is also valid.
    pub active: bool,
    ///The customer-facing code. Regardless of case, this code must be unique across all active promotion codes for each customer.
    pub code: String,
    /**A coupon contains information about a percent-off or amount-off discount you
might want to apply to a customer. Coupons may be applied to [subscriptions](https://stripe.com/docs/api#subscriptions), [invoices](https://stripe.com/docs/api#invoices),
[checkout sessions](https://stripe.com/docs/api/checkout/sessions), [quotes](https://stripe.com/docs/api#quotes), and more. Coupons do not work with conventional one-off [charges](https://stripe.com/docs/api#create_charge) or [payment intents](https://stripe.com/docs/api/payment_intents).*/
    pub coupon: Coupon,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The customer that this promotion code can be used by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    ///Date at which the promotion code can no longer be redeemed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Maximum number of times this promotion code can be redeemed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///
    pub restrictions: PromotionCodesResourceRestrictions,
    ///Number of times this promotion code has been used.
    pub times_redeemed: i64,
}
impl std::fmt::Display for PromotionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}