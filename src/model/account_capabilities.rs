
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub india_international_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<String>,
    #[serde(rename = "tax_reporting_us_1099_k")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us1099_k: Option<String>,
    #[serde(rename = "tax_reporting_us_1099_misc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us1099_misc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account_ach_payments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_payments: Option<String>,
}
impl std::fmt::Display for AccountCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}