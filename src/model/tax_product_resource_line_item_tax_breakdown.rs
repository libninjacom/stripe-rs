use serde::{Serialize, Deserialize};
use super::TaxProductResourceJurisdiction;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductResourceLineItemTaxBreakdown {
    ///The amount of tax, in integer cents.
    pub amount: i64,
    ///
    pub jurisdiction: TaxProductResourceJurisdiction,
    ///Indicates whether the jurisdiction was determined by the origin (merchant's address) or destination (customer's address).
    pub sourcing: String,
    ///Details regarding the rate for this tax. This field will be `null` when the tax is not imposed, for example if the product is exempt from tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate_details: Option<serde_json::Value>,
    ///The reasoning behind this tax, for example, if the product is tax exempt. The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: String,
    ///The amount on which tax is calculated, in integer cents.
    pub taxable_amount: i64,
}
impl std::fmt::Display for TaxProductResourceLineItemTaxBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}