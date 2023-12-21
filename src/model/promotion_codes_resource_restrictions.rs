use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromotionCodesResourceRestrictions {
    ///Promotion code restrictions defined in each available currency option. Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<serde_json::Value>,
    ///A Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices
    pub first_time_transaction: bool,
    ///Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
    ///Three-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount_currency: Option<String>,
}
impl std::fmt::Display for PromotionCodesResourceRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}