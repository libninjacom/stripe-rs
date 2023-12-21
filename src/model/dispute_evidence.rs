use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisputeEvidence {
    ///Any server or activity logs showing proof that the customer accessed or downloaded the purchased digital product. This information should include IP addresses, corresponding timestamps, and any detailed recorded activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_activity_log: Option<String>,
    ///The billing address provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<String>,
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your subscription cancellation policy, as shown to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy: Option<serde_json::Value>,
    ///An explanation of how and when the customer was shown your refund policy prior to purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_disclosure: Option<String>,
    ///A justification for why the customer's subscription was not canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_rebuttal: Option<String>,
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any communication with the customer that you feel is relevant to your case. Examples include emails proving that the customer received the product or service, or demonstrating their use of or satisfaction with the product or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_communication: Option<serde_json::Value>,
    ///The email address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,
    ///The name of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    ///The IP address that the customer used when making the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_purchase_ip: Option<String>,
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A relevant document or contract showing the customer's signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_signature: Option<serde_json::Value>,
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation for the prior charge that can uniquely identify the charge, such as a receipt, shipping label, work order, etc. This document should be paired with a similar document from the disputed payment that proves the two payments are separate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_documentation: Option<serde_json::Value>,
    ///An explanation of the difference between the disputed charge versus the prior charge that appears to be a duplicate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_explanation: Option<String>,
    ///The Stripe ID for the prior charge which appears to be a duplicate of the disputed charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_id: Option<String>,
    ///A description of the product or service that was sold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any receipt or message sent to the customer notifying them of the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<serde_json::Value>,
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your refund policy, as shown to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy: Option<serde_json::Value>,
    ///Documentation demonstrating that the customer was shown your refund policy prior to purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_disclosure: Option<String>,
    ///A justification for why the customer is not entitled to a refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_explanation: Option<String>,
    ///The date on which the customer received or began receiving the purchased service, in a clear human-readable format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_date: Option<String>,
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a service was provided to the customer. This could include a copy of a signed contract, work order, or other form of written agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_documentation: Option<serde_json::Value>,
    ///The address to which a physical product was shipped. You should try to include as complete address information as possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<String>,
    ///The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc. If multiple carriers were used for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_carrier: Option<String>,
    ///The date on which a physical product began its route to the shipping address, in a clear human-readable format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_date: Option<String>,
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a product was shipped to the customer at the same address the customer provided to you. This could include a copy of the shipment receipt, shipping label, etc. It should show the customer's full shipping address, if possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_documentation: Option<serde_json::Value>,
    ///The tracking number for a physical product, obtained from the delivery service. If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_tracking_number: Option<String>,
    ///(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any additional evidence or statements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_file: Option<serde_json::Value>,
    ///Any additional evidence or statements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_text: Option<String>,
}
impl std::fmt::Display for DisputeEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}