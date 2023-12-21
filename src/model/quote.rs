use serde::{Serialize, Deserialize};
use super::{
    InvoiceSettingQuoteSetting, QuotesResourceAutomaticTax, QuotesResourceComputed,
    QuotesResourceListLineItems, QuotesResourceStatusTransitions,
    QuotesResourceSubscriptionDataSubscriptionData, QuotesResourceTotalDetails,
};
/**A Quote is a way to model prices that you'd like to provide to a customer.
Once accepted, it will automatically create an invoice, subscription or subscription schedule.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Quote {
    ///Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    ///Total after discounts and taxes are applied.
    pub amount_total: i64,
    ///ID of the Connect Application that created the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    ///The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account. Only applicable if there are no line items with recurring prices on the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    ///A non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account. Only applicable if there are line items with recurring prices on the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    ///
    pub automatic_tax: QuotesResourceAutomaticTax,
    ///Either `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or on finalization using the default payment method attached to the subscription or customer. When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`. Defaults to `charge_automatically`.
    pub collection_method: String,
    ///
    pub computed: QuotesResourceComputed,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///The customer which this quote belongs to. A customer is required before finalizing the quote. Once specified, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    ///The tax rates applied to this quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<serde_json::Value>>,
    ///A description that will be displayed on the quote PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The discounts applied to this quote.
    pub discounts: Vec<serde_json::Value>,
    ///The date on which the quote will be canceled if in `open` or `draft` status. Measured in seconds since the Unix epoch.
    pub expires_at: i64,
    ///A footer that will be displayed on the quote PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    ///Details of the quote that was cloned. See the [cloning documentation](https://stripe.com/docs/quotes/clone) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_quote: Option<serde_json::Value>,
    ///A header that will be displayed on the quote PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///The invoice that was created from this quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<serde_json::Value>,
    ///
    pub invoice_settings: InvoiceSettingQuoteSetting,
    ///A list of items the customer is being quoted for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<QuotesResourceListLineItems>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///A unique number that identifies this particular quote. This number is assigned once the quote is [finalized](https://stripe.com/docs/quotes/overview#finalize).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The account on behalf of which to charge. See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    ///The status of the quote.
    pub status: String,
    ///
    pub status_transitions: QuotesResourceStatusTransitions,
    ///The subscription that was created or updated from this quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<serde_json::Value>,
    ///
    pub subscription_data: QuotesResourceSubscriptionDataSubscriptionData,
    ///The subscription schedule that was created or updated from this quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_schedule: Option<serde_json::Value>,
    ///ID of the test clock this quote belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<serde_json::Value>,
    ///
    pub total_details: QuotesResourceTotalDetails,
    ///The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<serde_json::Value>,
}
impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}