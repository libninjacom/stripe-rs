use serde::{Serialize, Deserialize};
use super::{InvoiceLineItemPeriod, TaxRate};
/**Invoice Items represent the component lines of an [invoice](https://stripe.com/docs/api/invoices). An invoice item is added to an
invoice by creating or updating it with an `invoice` field, at which point it will be included as
[an invoice line item](https://stripe.com/docs/api/invoices/line_item) within
[invoice.lines](https://stripe.com/docs/api/invoices/object#invoice_object-lines).

Invoice Items can be created before you are ready to actually send the invoice. This can be particularly useful when combined
with a [subscription](https://stripe.com/docs/api/subscriptions). Sometimes you want to add a charge or credit to a customer, but actually charge
or credit the customer’s card only at the end of a regular billing cycle. This is useful for combining several charges
(to minimize per-transaction fees), or for having Stripe tabulate your usage-based billing totals.

Related guides: [Integrate with the Invoicing API](https://stripe.com/docs/invoicing/integration), [Subscription Invoices](https://stripe.com/docs/billing/invoices/subscription#adding-upcoming-invoice-items).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Invoiceitem {
    ///Amount (in the `currency` specified) of the invoice item. This should always be equal to `unit_amount * quantity`.
    pub amount: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///The ID of the customer who will be billed when this invoice item is billed.
    pub customer: serde_json::Value,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub date: i64,
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///If true, discounts will apply to this invoice item. Always false for prorations.
    pub discountable: bool,
    ///The discounts which apply to the invoice item. Item discounts are applied before invoice discounts. Use `expand[]=discounts` to expand each discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<serde_json::Value>>,
    ///Unique identifier for the object.
    pub id: String,
    ///The ID of the invoice this invoice item belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<serde_json::Value>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///
    pub period: InvoiceLineItemPeriod,
    ///The price of the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
    ///Whether the invoice item was created automatically as a proration adjustment when the customer switched plans.
    pub proration: bool,
    ///Quantity of units for the invoice item. If the invoice item is a proration, the quantity of the subscription that the proration was computed for.
    pub quantity: i64,
    ///The subscription that this invoice item has been created for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<serde_json::Value>,
    ///The subscription item that this invoice item has been created for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,
    ///The tax rates which apply to the invoice item. When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,
    ///ID of the test clock this invoice item belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<serde_json::Value>,
    ///Unit amount (in the `currency` specified) of the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    ///Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub unit_amount_decimal: Option<rust_decimal::Decimal>,
}
impl std::fmt::Display for Invoiceitem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}