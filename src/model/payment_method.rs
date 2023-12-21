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
/**PaymentMethod objects represent your customer's payment instruments.
You can use them with [PaymentIntents](https://stripe.com/docs/payments/payment-intents) to collect payments or save them to
Customer objects to store instrument details for future payments.

Related guides: [Payment Methods](https://stripe.com/docs/payments/payment-methods) and [More Payment Scenarios](https://stripe.com/docs/payments/more-payment-scenarios).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethod {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodAcssDebit>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<PaymentMethodAffirm>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<PaymentMethodAfterpayClearpay>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PaymentFlowsPrivatePaymentMethodsAlipay>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<PaymentMethodAuBecsDebit>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<PaymentMethodBacsDebit>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PaymentMethodBancontact>,
    ///
    pub billing_details: BillingDetails,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<PaymentMethodBlik>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<PaymentMethodBoleto>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodCard>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<PaymentMethodCardPresent>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<PaymentMethodCashapp>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The ID of the Customer to which this PaymentMethod is saved. This will not be set when the PaymentMethod has not been saved to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<PaymentMethodCustomerBalance>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<PaymentMethodEps>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<PaymentMethodFpx>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<PaymentMethodGiropay>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<PaymentMethodGrabpay>,
    ///Unique identifier for the object.
    pub id: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PaymentMethodIdeal>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<PaymentMethodInteracPresent>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PaymentMethodKlarna>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<PaymentMethodKonbini>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodLink>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PaymentMethodOxxo>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PaymentMethodP24>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<PaymentMethodPaynow>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodPaypal>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<PaymentMethodPix>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<PaymentMethodPromptpay>,
    ///Options to configure Radar. See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<RadarRadarOptions>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<PaymentMethodRevolutPay>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PaymentMethodSepaDebit>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PaymentMethodSofort>,
    ///The type of the PaymentMethod. An additional hash is included on the PaymentMethod with a name matching this value. It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PaymentMethodUsBankAccount>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PaymentMethodWechatPay>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<PaymentMethodZip>,
}
impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}