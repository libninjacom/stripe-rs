use serde::{Serialize, Deserialize};
use super::Discount;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LineItemsDiscountAmount {
    ///The amount discounted.
    pub amount: i64,
    /**A discount represents the actual application of a [coupon](https://stripe.com/docs/api#coupons) or [promotion code](https://stripe.com/docs/api#promotion_codes).
It contains information about when the discount began, when it will end, and what it is applied to.

Related guide: [Applying discounts to subscriptions](https://stripe.com/docs/billing/subscriptions/discounts)*/
    pub discount: Discount,
}
impl std::fmt::Display for LineItemsDiscountAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}