use serde::{Serialize, Deserialize};
use super::{LineItemsDiscountAmount, LineItemsTaxAmount};
///A line item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Item {
    ///Total discount amount applied. If no discounts were applied, defaults to 0.
    pub amount_discount: i64,
    ///Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    ///Total tax amount applied. If no tax was applied, defaults to 0.
    pub amount_tax: i64,
    ///Total after discounts and taxes.
    pub amount_total: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///An arbitrary string attached to the object. Often useful for displaying to users. Defaults to product name.
    pub description: String,
    ///The discounts applied to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<LineItemsDiscountAmount>>,
    ///Unique identifier for the object.
    pub id: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The price used to generate the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
    ///The quantity of products being purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    ///The taxes applied to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}