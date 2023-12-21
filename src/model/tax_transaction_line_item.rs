use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxTransactionLineItem {
    ///The line item amount in integer cents. If `tax_behavior=inclusive`, then this amount includes taxes. Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    ///The amount of tax calculated for this line item, in integer cents.
    pub amount_tax: i64,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The ID of an existing [Product](https://stripe.com/docs/api/products/object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    ///The number of units of the item being purchased. For reversals, this is the quantity reversed.
    pub quantity: i64,
    ///A custom identifier for this line item in the transaction.
    pub reference: String,
    ///If `type=reversal`, contains information about what was reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal: Option<serde_json::Value>,
    ///Specifies whether the `amount` includes taxes. If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: String,
    ///The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for this resource.
    pub tax_code: String,
    ///If `reversal`, this line item reverses an earlier transaction.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TaxTransactionLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}