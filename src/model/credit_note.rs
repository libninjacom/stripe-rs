use serde::{Serialize, Deserialize};
use super::{CreditNoteLinesList, CreditNoteTaxAmount, DiscountsResourceDiscountAmount};
/**Issue a credit note to adjust an invoice's amount after the invoice is finalized.

Related guide: [Credit notes](https://stripe.com/docs/billing/invoices/credit-notes)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditNote {
    ///The integer amount in cents (or local equivalent) representing the total amount of the credit note, including tax.
    pub amount: i64,
    ///This is the sum of all the shipping amounts.
    pub amount_shipping: i64,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///ID of the customer.
    pub customer: serde_json::Value,
    ///Customer balance transaction related to this credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance_transaction: Option<serde_json::Value>,
    ///The integer amount in cents (or local equivalent) representing the total amount of discount that was credited.
    pub discount_amount: i64,
    ///The aggregate amounts calculated per discount for all line items.
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,
    ///The date when this credit note is in effect. Same as `created` unless overwritten. When defined, this value replaces the system-generated 'Date of issue' printed on the credit note PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<i64>,
    ///Unique identifier for the object.
    pub id: String,
    ///ID of the invoice.
    pub invoice: serde_json::Value,
    ///Line items that make up the credit note
    pub lines: CreditNoteLinesList,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Customer-facing text that appears on the credit note PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///A unique number that identifies this particular credit note and appears on the PDF of the credit note and its associated invoice.
    pub number: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Amount that was credited outside of Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_band_amount: Option<i64>,
    ///The link to download the PDF of the credit note.
    pub pdf: String,
    ///Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    ///Refund related to this credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<serde_json::Value>,
    ///The details of the cost of shipping, including the ShippingRate applied to the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<serde_json::Value>,
    ///Status of this credit note, one of `issued` or `void`. Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
    pub status: String,
    ///The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding exclusive tax and invoice level discounts.
    pub subtotal: i64,
    ///The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding all tax and invoice level discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_excluding_tax: Option<i64>,
    ///The aggregate amounts calculated per tax rate for all line items.
    pub tax_amounts: Vec<CreditNoteTaxAmount>,
    ///The integer amount in cents (or local equivalent) representing the total amount of the credit note, including tax and all discount.
    pub total: i64,
    ///The integer amount in cents (or local equivalent) representing the total amount of the credit note, excluding tax, but including discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_excluding_tax: Option<i64>,
    ///Type of this credit note, one of `pre_payment` or `post_payment`. A `pre_payment` credit note means it was issued when the invoice was open. A `post_payment` credit note means it was issued when the invoice was paid.
    #[serde(rename = "type")]
    pub type_: String,
    ///The time that the credit note was voided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voided_at: Option<i64>,
}
impl std::fmt::Display for CreditNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}