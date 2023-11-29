
use serde::{Serialize, Deserialize};
use super::PaymentMethodOptionsParam;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodOptionsParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PaymentMethodOptionsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PaymentMethodOptionsParam>,
}
impl std::fmt::Display for PaymentMethodOptionsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}