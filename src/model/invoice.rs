use serde::{Serialize, Deserialize};
use super::{
    AutomaticTax, DiscountsResourceDiscountAmount, InvoiceLinesList,
    InvoiceSettingCustomField, InvoiceTaxAmount, InvoiceThresholdReason,
    InvoicesPaymentSettings, InvoicesResourceInvoiceTaxId, InvoicesStatusTransitions,
    TaxRate,
};
/**Invoices are statements of amounts owed by a customer, and are either
generated one-off, or generated periodically from a subscription.

They contain [invoice items](https://stripe.com/docs/api#invoiceitems), and proration adjustments
that may be caused by subscription upgrades/downgrades (if necessary).

If your invoice is configured to be billed through automatic charges,
Stripe automatically finalizes your invoice and attempts payment. Note
that finalizing the invoice,
[when automatic](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection), does
not happen immediately as the invoice is created. Stripe waits
until one hour after the last webhook was successfully sent (or the last
webhook timed out after failing). If you (and the platforms you may have
connected to) have no webhooks configured, Stripe waits one hour after
creation to finalize the invoice.

If your invoice is configured to be billed by sending an email, then based on your
[email settings](https://dashboard.stripe.com/account/billing/automatic),
Stripe will email the invoice to your customer and await payment. These
emails can contain a link to a hosted page to pay the invoice.

Stripe applies any customer credit on the account before determining the
amount due for the invoice (i.e., the amount that will be actually
charged). If the amount due for the invoice is less than Stripe's [minimum allowed charge
per currency](/docs/currencies#minimum-and-maximum-charge-amounts), the
invoice is automatically marked paid, and we add the amount due to the
customer's credit balance which is applied to the next invoice.

More details on the customer's credit balance are
[here](https://stripe.com/docs/billing/customer/balance).

Related guide: [Send invoices to customers](https://stripe.com/docs/billing/invoices/sending)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Invoice {
    ///The country of the business associated with this invoice, most often the business creating the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_country: Option<String>,
    ///The public name of the business associated with this invoice, most often the business creating the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    ///The account tax IDs associated with the invoice. Only editable when the invoice is a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<serde_json::Value>>,
    ///Final amount due at this time for this invoice. If the invoice's total is smaller than the minimum charge amount, for example, or if there is account credit that can be applied to the invoice, the `amount_due` may be 0. If there is a positive `starting_balance` for the invoice (the customer owes money), the `amount_due` will also take that into account. The charge that gets generated for the invoice will be for the amount specified in `amount_due`.
    pub amount_due: i64,
    ///The amount, in cents (or local equivalent), that was paid.
    pub amount_paid: i64,
    ///The difference between amount_due and amount_paid, in cents (or local equivalent).
    pub amount_remaining: i64,
    ///This is the sum of all the shipping amounts.
    pub amount_shipping: i64,
    ///ID of the Connect Application that created the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    ///The fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account when the invoice is paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    ///Number of payment attempts made for this invoice, from the perspective of the payment retry schedule. Any payment attempt counts as the first attempt, and subsequently only automatic retries increment the attempt count. In other words, manual payment attempts after the first attempt do not affect the retry schedule.
    pub attempt_count: i64,
    ///Whether an attempt has been made to pay the invoice. An invoice is not attempted until 1 hour after the `invoice.created` webhook, for example, so you might not want to display that invoice as unpaid to your users.
    pub attempted: bool,
    ///Controls whether Stripe performs [automatic collection](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection) of the invoice. If `false`, the invoice's state doesn't automatically advance without an explicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,
    ///
    pub automatic_tax: AutomaticTax,
    /**Indicates the reason why the invoice was created.

* `manual`: Unrelated to a subscription, for example, created via the invoice editor.
* `subscription`: No longer in use. Applies to subscriptions from before May 2018 where no distinction was made between updates, cycles, and thresholds.
* `subscription_create`: A new subscription was created.
* `subscription_cycle`: A subscription advanced into a new period.
* `subscription_threshold`: A subscription reached a billing threshold.
* `subscription_update`: A subscription was updated.
* `upcoming`: Reserved for simulated invoices, per the upcoming invoice endpoint.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reason: Option<String>,
    ///ID of the latest charge generated for this invoice, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<serde_json::Value>,
    ///Either `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer. When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    pub collection_method: String,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///Custom fields displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<InvoiceSettingCustomField>>,
    ///The ID of the customer who will be billed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    ///The customer's address. Until the invoice is finalized, this field will equal `customer.address`. Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<serde_json::Value>,
    ///The customer's email. Until the invoice is finalized, this field will equal `customer.email`. Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    ///The customer's name. Until the invoice is finalized, this field will equal `customer.name`. Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    ///The customer's phone number. Until the invoice is finalized, this field will equal `customer.phone`. Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_phone: Option<String>,
    ///The customer's shipping information. Until the invoice is finalized, this field will equal `customer.shipping`. Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_shipping: Option<serde_json::Value>,
    ///The customer's tax exempt status. Until the invoice is finalized, this field will equal `customer.tax_exempt`. Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tax_exempt: Option<String>,
    ///The customer's tax IDs. Until the invoice is finalized, this field will contain the same tax IDs as `customer.tax_ids`. Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tax_ids: Option<Vec<InvoicesResourceInvoiceTaxId>>,
    ///ID of the default payment method for the invoice. It must belong to the customer associated with the invoice. If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<serde_json::Value>,
    ///ID of the default payment source for the invoice. It must belong to the customer associated with the invoice and be in a chargeable state. If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<serde_json::Value>,
    ///The tax rates applied to this invoice, if any.
    pub default_tax_rates: Vec<TaxRate>,
    ///An arbitrary string attached to the object. Often useful for displaying to users. Referenced as 'memo' in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Describes the current discount applied to this invoice, if there is one. Not populated if there are multiple discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<serde_json::Value>,
    ///The discounts applied to the invoice. Line item discounts are applied before invoice discounts. Use `expand[]=discounts` to expand each discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<serde_json::Value>>,
    ///The date on which payment for this invoice is due. This value will be `null` for invoices where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<i64>,
    ///The date when this invoice is in effect. Same as `finalized_at` unless overwritten. When defined, this value replaces the system-generated 'Date of issue' printed on the invoice PDF and receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<i64>,
    ///Ending customer balance after the invoice is finalized. Invoices are finalized approximately an hour after successful webhook delivery or when payment collection is attempted for the invoice. If the invoice has not been finalized yet, this will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_balance: Option<i64>,
    ///Footer displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    ///Details of the invoice that was cloned. See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_invoice: Option<serde_json::Value>,
    ///The URL for the hosted invoice page, which allows customers to view and pay an invoice. If the invoice has not been finalized yet, this will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_invoice_url: Option<String>,
    ///Unique identifier for the object. This property is always present unless the invoice is an upcoming invoice. See [Retrieve an upcoming invoice](https://stripe.com/docs/api/invoices/upcoming) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The link to download the PDF for the invoice. If the invoice has not been finalized yet, this will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_pdf: Option<String>,
    ///The error encountered during the previous attempt to finalize the invoice. This field is cleared when the invoice is successfully finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_finalization_error: Option<serde_json::Value>,
    ///The ID of the most recent non-draft revision of this invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<serde_json::Value>,
    ///The individual line items that make up the invoice. `lines` is sorted as follows: (1) pending invoice items (including prorations) in reverse chronological order, (2) subscription items in reverse chronological order, and (3) invoice items added after invoice creation in chronological order.
    pub lines: InvoiceLinesList,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///The time at which payment will next be attempted. This value will be `null` for invoices where `collection_method=send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_payment_attempt: Option<i64>,
    ///A unique, identifying string that appears on emails sent to the customer for this invoice. This starts with the customer's unique invoice_prefix if it is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The account (if any) for which the funds of the invoice payment are intended. If set, the invoice will be presented with the branding and support information of the specified account. See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    ///Whether payment was successfully collected for this invoice. An invoice can be paid (most commonly) with a charge or with credit from the customer's account balance.
    pub paid: bool,
    ///Returns true if the invoice was manually marked paid, returns false if the invoice hasn't been paid yet or was paid on Stripe.
    pub paid_out_of_band: bool,
    ///The PaymentIntent associated with this invoice. The PaymentIntent is generated when the invoice is finalized, and can then be used to pay the invoice. Note that voiding an invoice will cancel the PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<serde_json::Value>,
    ///
    pub payment_settings: InvoicesPaymentSettings,
    ///End of the usage period during which invoice items were added to this invoice.
    pub period_end: i64,
    ///Start of the usage period during which invoice items were added to this invoice.
    pub period_start: i64,
    ///Total amount of all post-payment credit notes issued for this invoice.
    pub post_payment_credit_notes_amount: i64,
    ///Total amount of all pre-payment credit notes issued for this invoice.
    pub pre_payment_credit_notes_amount: i64,
    ///The quote this invoice was generated from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<serde_json::Value>,
    ///This is the transaction number that appears on email receipts sent for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    ///The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering: Option<serde_json::Value>,
    ///The details of the cost of shipping, including the ShippingRate applied on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<serde_json::Value>,
    ///Shipping details for the invoice. The Invoice PDF will use the `shipping_details` value if it is set, otherwise the PDF will render the shipping address from the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<serde_json::Value>,
    ///Starting customer balance before the invoice is finalized. If the invoice has not been finalized yet, this will be the current customer balance. For revision invoices, this also includes any customer balance that was applied to the original invoice.
    pub starting_balance: i64,
    ///Extra information about an invoice for the customer's credit card statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    ///The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`. [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///
    pub status_transitions: InvoicesStatusTransitions,
    ///The subscription that this invoice was prepared for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<serde_json::Value>,
    ///Details about the subscription that created this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_details: Option<serde_json::Value>,
    ///Only set for upcoming invoices that preview prorations. The time used to calculate prorations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<i64>,
    ///Total of all subscriptions, invoice items, and prorations on the invoice before any invoice level discount or exclusive tax is applied. Item discounts are already incorporated
    pub subtotal: i64,
    ///The integer amount in cents (or local equivalent) representing the subtotal of the invoice before any invoice level discount or tax is applied. Item discounts are already incorporated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_excluding_tax: Option<i64>,
    ///The amount of tax on this invoice. This is the sum of all the tax amounts on this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<i64>,
    ///ID of the test clock this invoice belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<serde_json::Value>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_reason: Option<InvoiceThresholdReason>,
    ///Total after discounts and taxes.
    pub total: i64,
    ///The aggregate amounts calculated per discount across all line items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount_amounts: Option<Vec<DiscountsResourceDiscountAmount>>,
    ///The integer amount in cents (or local equivalent) representing the total amount of the invoice including all discounts but excluding all tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_excluding_tax: Option<i64>,
    ///The aggregate amounts calculated per tax rate for all line items.
    pub total_tax_amounts: Vec<InvoiceTaxAmount>,
    ///The account (if any) the payment will be attributed to for tax reporting, and where funds from the payment will be transferred to for the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<serde_json::Value>,
    ///Invoices are automatically paid or sent 1 hour after webhooks are delivered, or until all webhook delivery attempts have [been exhausted](https://stripe.com/docs/billing/webhooks#understand). This field tracks the time when webhooks for this invoice were successfully delivered. If the invoice had no webhooks to deliver, this will be set while the invoice is being created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks_delivered_at: Option<i64>,
}
impl std::fmt::Display for Invoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}