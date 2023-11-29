
use serde::{Serialize, Deserialize};
use super::PaymentMethodConfigResourcePaymentMethodProperties;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_bank_transfer: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub is_default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netbanking: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
}
impl std::fmt::Display for PaymentMethodConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}