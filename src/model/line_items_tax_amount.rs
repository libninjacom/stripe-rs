use serde::{Serialize, Deserialize};
use super::TaxRate;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LineItemsTaxAmount {
    ///Amount of tax applied for this rate.
    pub amount: i64,
    /**Tax rates can be applied to [invoices](https://stripe.com/docs/billing/invoices/tax-rates), [subscriptions](https://stripe.com/docs/billing/subscriptions/taxes) and [Checkout Sessions](https://stripe.com/docs/payments/checkout/set-up-a-subscription#tax-rates) to collect tax.

Related guide: [Tax rates](https://stripe.com/docs/billing/taxes/tax-rates)*/
    pub rate: TaxRate,
    ///The reasoning behind this tax, for example, if the product is tax exempt. The possible values for this field may be extended as new tax rules are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxability_reason: Option<String>,
    ///The amount on which tax is calculated, in cents (or local equivalent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<i64>,
}
impl std::fmt::Display for LineItemsTaxAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}