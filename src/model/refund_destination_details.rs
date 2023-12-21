use serde::{Serialize, Deserialize};
use super::{
    DestinationDetailsUnimplemented, RefundDestinationDetailsCard,
    RefundDestinationDetailsGeneric,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefundDestinationDetails {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_bank_transfer: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<RefundDestinationDetailsGeneric>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub br_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<RefundDestinationDetailsCard>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_cash_balance: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<RefundDestinationDetailsGeneric>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    ///The type of transaction-specific details of the payment method used in the refund (e.g., `card`). An additional hash is included on `destination_details` with a name matching this value. It contains information specific to the refund transaction.
    #[serde(rename = "type")]
    pub type_: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer: Option<RefundDestinationDetailsGeneric>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<DestinationDetailsUnimplemented>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<DestinationDetailsUnimplemented>,
}
impl std::fmt::Display for RefundDestinationDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}