use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditNoteTaxAmount {
    ///The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,
    ///Whether this tax amount is inclusive or exclusive.
    pub inclusive: bool,
    ///The tax rate that was applied to get this tax amount.
    pub tax_rate: serde_json::Value,
    ///The reasoning behind this tax, for example, if the product is tax exempt. The possible values for this field may be extended as new tax rules are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxability_reason: Option<String>,
    ///The amount on which tax is calculated, in cents (or local equivalent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<i64>,
}
impl std::fmt::Display for CreditNoteTaxAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}