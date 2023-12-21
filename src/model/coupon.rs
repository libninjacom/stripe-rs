use serde::{Serialize, Deserialize};
use super::CouponAppliesTo;
/**A coupon contains information about a percent-off or amount-off discount you
might want to apply to a customer. Coupons may be applied to [subscriptions](https://stripe.com/docs/api#subscriptions), [invoices](https://stripe.com/docs/api#invoices),
[checkout sessions](https://stripe.com/docs/api/checkout/sessions), [quotes](https://stripe.com/docs/api#quotes), and more. Coupons do not work with conventional one-off [charges](https://stripe.com/docs/api#create_charge) or [payment intents](https://stripe.com/docs/api/payment_intents).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Coupon {
    ///Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<i64>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<CouponAppliesTo>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///If `amount_off` has been set, the three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the amount to take off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///Coupons defined in each available currency option. Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<serde_json::Value>,
    ///One of `forever`, `once`, and `repeating`. Describes how long a customer who applies this coupon will get the discount.
    pub duration: String,
    ///If `duration` is `repeating`, the number of months the coupon applies. Null if coupon `duration` is `forever` or `once`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_months: Option<i64>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Maximum number of times this coupon can be redeemed, in total, across all customers, before it is no longer valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///Name of the coupon displayed to customers on for instance invoices or receipts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Percent that will be taken off the subtotal of any invoices for this customer for the duration of the coupon. For example, a coupon with percent_off of 50 will make a $ (or local equivalent)100 invoice $ (or local equivalent)50 instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<f64>,
    ///Date after which the coupon can no longer be redeemed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_by: Option<i64>,
    ///Number of times this coupon has been applied to a customer.
    pub times_redeemed: i64,
    ///Taking account of the above properties, whether this coupon can still be applied to a customer.
    pub valid: bool,
}
impl std::fmt::Display for Coupon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}