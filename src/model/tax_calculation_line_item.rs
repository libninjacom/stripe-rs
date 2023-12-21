use serde::{Serialize, Deserialize};
use super::TaxProductResourceLineItemTaxBreakdown;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxCalculationLineItem {
    ///The line item amount in integer cents. If `tax_behavior=inclusive`, then this amount includes taxes. Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    ///The amount of tax calculated for this line item, in integer cents.
    pub amount_tax: i64,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The ID of an existing [Product](https://stripe.com/docs/api/products/object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    ///The number of units of the item being purchased. For reversals, this is the quantity reversed.
    pub quantity: i64,
    ///A custom identifier for this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    ///Specifies whether the `amount` includes taxes. If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: String,
    ///Detailed account of taxes relevant to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_breakdown: Option<Vec<TaxProductResourceLineItemTaxBreakdown>>,
    ///The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for this resource.
    pub tax_code: String,
}
impl std::fmt::Display for TaxCalculationLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}