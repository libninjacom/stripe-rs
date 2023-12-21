use serde::{Serialize, Deserialize};
use super::{
    SchedulesPhaseAutomaticTax, SubscriptionScheduleAddInvoiceItem,
    SubscriptionScheduleConfigurationItem, TaxRate,
};
///A phase describes the plans, coupon, and trialing status of a subscription for a predefined time period.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionSchedulePhaseConfiguration {
    ///A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    pub add_invoice_items: Vec<SubscriptionScheduleAddInvoiceItem>,
    ///A non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account during this phase of the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<SchedulesPhaseAutomaticTax>,
    ///Possible values are `phase_start` or `automatic`. If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase. If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase. For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<String>,
    ///Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<serde_json::Value>,
    ///Either `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer. When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<String>,
    ///ID of the coupon to use during this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<serde_json::Value>,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///ID of the default payment method for the subscription schedule. It must belong to the customer associated with the subscription schedule. If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<serde_json::Value>,
    ///The default tax rates to apply to the subscription during this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<TaxRate>>,
    ///Subscription description, meant to be displayable to the customer. Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The end of this phase of the subscription schedule.
    pub end_date: i64,
    ///The invoice settings applicable during this phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<serde_json::Value>,
    ///Subscription items to configure the subscription to during this phase of the subscription schedule.
    pub items: Vec<SubscriptionScheduleConfigurationItem>,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase. Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered. Updating the underlying subscription's `metadata` directly will not affect the current phase's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///The account (if any) the charge was made on behalf of for charges associated with the schedule's subscription. See the Connect documentation for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    ///If the subscription schedule will prorate when transitioning to this phase. Possible values are `create_prorations` and `none`.
    pub proration_behavior: String,
    ///The start of this phase of the subscription schedule.
    pub start_date: i64,
    ///The account (if any) the associated subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<serde_json::Value>,
    ///When the trial ends within the phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<i64>,
}
impl std::fmt::Display for SubscriptionSchedulePhaseConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}