use serde::{Serialize, Deserialize};
use super::{
    TaxProductResourceCustomerDetails, TaxProductResourceTaxBreakdown,
    TaxProductResourceTaxCalculationLineItemList,
};
/**A Tax Calculation allows you to calculate the tax to collect from your customer.

Related guide: [Calculate tax in your custom payment flow](https://stripe.com/docs/tax/custom)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxCalculation {
    ///Total after taxes.
    pub amount_total: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///The ID of an existing [Customer](https://stripe.com/docs/api/customers/object) used for the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    ///
    pub customer_details: TaxProductResourceCustomerDetails,
    ///Timestamp of date at which the tax calculation will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    ///Unique identifier for the calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The list of items the customer is purchasing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<TaxProductResourceTaxCalculationLineItemList>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The shipping cost details for the calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<serde_json::Value>,
    ///The amount of tax to be collected on top of the line item prices.
    pub tax_amount_exclusive: i64,
    ///The amount of tax already included in the line item prices.
    pub tax_amount_inclusive: i64,
    ///Breakdown of individual tax amounts that add up to the total.
    pub tax_breakdown: Vec<TaxProductResourceTaxBreakdown>,
    ///Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: i64,
}
impl std::fmt::Display for TaxCalculation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}