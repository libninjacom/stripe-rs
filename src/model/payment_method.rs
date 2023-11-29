
use serde::{Serialize, Deserialize};
use super::{
    BillingDetails, PaymentFlowsPrivatePaymentMethodsAlipay, PaymentMethodAcssDebit,
    PaymentMethodAffirm, PaymentMethodAfterpayClearpay, PaymentMethodAuBecsDebit,
    PaymentMethodBacsDebit, PaymentMethodBancontact, PaymentMethodBlik,
    PaymentMethodBoleto, PaymentMethodCard, PaymentMethodCardPresent,
    PaymentMethodCashapp, PaymentMethodCustomerBalance, PaymentMethodEps,
    PaymentMethodFpx, PaymentMethodGiropay, PaymentMethodGrabpay, PaymentMethodIdeal,
    PaymentMethodInteracPresent, PaymentMethodKlarna, PaymentMethodKonbini,
    PaymentMethodLink, PaymentMethodOxxo, PaymentMethodP24, PaymentMethodPaynow,
    PaymentMethodPaypal, PaymentMethodPix, PaymentMethodPromptpay,
    PaymentMethodRevolutPay, PaymentMethodSepaDebit, PaymentMethodSofort,
    PaymentMethodUsBankAccount, PaymentMethodWechatPay, PaymentMethodZip,
    RadarRadarOptions,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<PaymentMethodAffirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<PaymentMethodAfterpayClearpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PaymentFlowsPrivatePaymentMethodsAlipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<PaymentMethodAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<PaymentMethodBacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PaymentMethodBancontact>,
    pub billing_details: BillingDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<PaymentMethodBlik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<PaymentMethodBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<PaymentMethodCardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<PaymentMethodCashapp>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<PaymentMethodCustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<PaymentMethodEps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<PaymentMethodFpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<PaymentMethodGiropay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<PaymentMethodGrabpay>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PaymentMethodIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<PaymentMethodInteracPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PaymentMethodKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<PaymentMethodKonbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodLink>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PaymentMethodOxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PaymentMethodP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<PaymentMethodPaynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodPaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<PaymentMethodPix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<PaymentMethodPromptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<RadarRadarOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<PaymentMethodRevolutPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PaymentMethodSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PaymentMethodSofort>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PaymentMethodUsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PaymentMethodWechatPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<PaymentMethodZip>,
}
impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}