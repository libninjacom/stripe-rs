use serde::{Serialize, Deserialize};
use super::TaxProductResourceTaxRateDetails;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductResourceTaxBreakdown {
    ///The amount of tax, in integer cents.
    pub amount: i64,
    ///Specifies whether the tax amount is included in the line item amount.
    pub inclusive: bool,
    ///
    pub tax_rate_details: TaxProductResourceTaxRateDetails,
    ///The reasoning behind this tax, for example, if the product is tax exempt. We might extend the possible values for this field to support new tax rules.
    pub taxability_reason: String,
    ///The amount on which tax is calculated, in integer cents.
    pub taxable_amount: i64,
}
impl std::fmt::Display for TaxProductResourceTaxBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}