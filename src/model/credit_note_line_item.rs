use serde::{Serialize, Deserialize};
use super::{CreditNoteTaxAmount, DiscountsResourceDiscountAmount, TaxRate};
///The credit note line item object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditNoteLineItem {
    ///The integer amount in cents (or local equivalent) representing the gross amount being credited for this line item, excluding (exclusive) tax and discounts.
    pub amount: i64,
    ///The integer amount in cents (or local equivalent) representing the amount being credited for this line item, excluding all tax and discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_excluding_tax: Option<i64>,
    ///Description of the item being credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The integer amount in cents (or local equivalent) representing the discount being credited for this line item.
    pub discount_amount: i64,
    ///The amount of discount calculated per discount for this line item
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,
    ///Unique identifier for the object.
    pub id: String,
    ///ID of the invoice line item being credited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The number of units of product being credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    ///The amount of tax calculated per tax rate for this line item
    pub tax_amounts: Vec<CreditNoteTaxAmount>,
    ///The tax rates which apply to the line item.
    pub tax_rates: Vec<TaxRate>,
    ///The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`. When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
    #[serde(rename = "type")]
    pub type_: String,
    ///The cost of each unit of product being credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    ///Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub unit_amount_decimal: Option<rust_decimal::Decimal>,
    ///The amount in cents (or local equivalent) representing the unit amount being credited for this line item, excluding all tax and discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub unit_amount_excluding_tax: Option<rust_decimal::Decimal>,
}
impl std::fmt::Display for CreditNoteLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}