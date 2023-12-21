use serde::{Serialize, Deserialize};
use super::{
    TaxProductResourceCustomerDetails, TaxProductResourceTaxTransactionLineItemList,
};
/**A Tax Transaction records the tax collected from or refunded to your customer.

Related guide: [Calculate tax in your custom payment flow](https://stripe.com/docs/tax/custom#tax-transaction)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxTransaction {
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///The ID of an existing [Customer](https://stripe.com/docs/api/customers/object) used for the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    ///
    pub customer_details: TaxProductResourceCustomerDetails,
    ///Unique identifier for the transaction.
    pub id: String,
    ///The tax collected or refunded, by line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<TaxProductResourceTaxTransactionLineItemList>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///A custom unique identifier, such as 'myOrder_123'.
    pub reference: String,
    ///If `type=reversal`, contains information about what was reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal: Option<serde_json::Value>,
    ///The shipping cost details for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<serde_json::Value>,
    ///Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: i64,
    ///If `reversal`, this transaction reverses an earlier transaction.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TaxTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}