use serde::{Serialize, Deserialize};
use super::PaymentMethodConfigResourcePaymentMethodProperties;
/**PaymentMethodConfigurations control which payment methods are displayed to your customers when you don't explicitly specify payment method types. You can have multiple configurations with different sets of payment methods for different scenarios.

There are two types of PaymentMethodConfigurations. Which is used depends on the [charge type](https://stripe.com/docs/connect/charges):

**Direct** configurations apply to payments created on your account, including Connect destination charges, Connect separate charges and transfers, and payments not involving Connect.

**Child** configurations apply to payments created on your connected accounts using direct charges, and charges with the on_behalf_of parameter.

Child configurations have a `parent` that sets default values and controls which settings connected accounts may override. You can specify a parent ID at payment time, and Stripe will automatically resolve the connected account’s associated child configuration. Parent configurations are [managed in the dashboard](https://dashboard.stripe.com/settings/payment_methods/connected_accounts) and are not available in this API.

Related guides:
- [Payment Method Configurations API](https://stripe.com/docs/connect/payment-method-configurations)
- [Multiple configurations on dynamic payment methods](https://stripe.com/docs/payments/multiple-payment-method-configs)
- [Multiple configurations for your Connect accounts](https://stripe.com/docs/connect/multiple-payment-method-configurations)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodConfiguration {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///Whether the configuration can be used for new payments.
    pub active: bool,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///For child configs, the Connect application associated with the configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///Unique identifier for the object.
    pub id: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///The default configuration is used whenever a payment method configuration is not specified.
    pub is_default: bool,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///The configuration's name.
    pub name: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///For child configs, the configuration's parent configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PaymentMethodConfigResourcePaymentMethodProperties>,
}
impl std::fmt::Display for PaymentMethodConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}