use serde::{Serialize, Deserialize};
use super::Coupon;
/**A discount represents the actual application of a [coupon](https://stripe.com/docs/api#coupons) or [promotion code](https://stripe.com/docs/api#promotion_codes).
It contains information about when the discount began, when it will end, and what it is applied to.

Related guide: [Applying discounts to subscriptions](https://stripe.com/docs/billing/subscriptions/discounts)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Discount {
    ///The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode. Will not be present for subscription mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_session: Option<String>,
    /**A coupon contains information about a percent-off or amount-off discount you
might want to apply to a customer. Coupons may be applied to [subscriptions](https://stripe.com/docs/api#subscriptions), [invoices](https://stripe.com/docs/api#invoices),
[checkout sessions](https://stripe.com/docs/api/checkout/sessions), [quotes](https://stripe.com/docs/api#quotes), and more. Coupons do not work with conventional one-off [charges](https://stripe.com/docs/api#create_charge) or [payment intents](https://stripe.com/docs/api/payment_intents).*/
    pub coupon: Coupon,
    ///The ID of the customer associated with this discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    ///If the coupon has a duration of `repeating`, the date that this discount will end. If the coupon has a duration of `once` or `forever`, this attribute will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    ///The ID of the discount object. Discounts cannot be fetched by ID. Use `expand[]=discounts` in API calls to expand discount IDs in an array.
    pub id: String,
    ///The invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    ///The invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The promotion code applied to create this discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<serde_json::Value>,
    ///Date that the coupon was applied.
    pub start: i64,
    ///The subscription that this coupon is applied to, if it is applied to a particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
}
impl std::fmt::Display for Discount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}