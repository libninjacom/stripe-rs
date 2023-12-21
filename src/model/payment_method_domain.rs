use serde::{Serialize, Deserialize};
use super::PaymentMethodDomainResourcePaymentMethodStatus;
/**A payment method domain represents a web domain that you have registered with Stripe.
Stripe Elements use registered payment method domains to control where certain payment methods are shown.

Related guides: [Payment method domains](https://stripe.com/docs/payments/payment-methods/pmd-registration).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodDomain {
    ///Indicates the status of a specific payment method on a payment method domain.
    pub apple_pay: PaymentMethodDomainResourcePaymentMethodStatus,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The domain name that this payment method domain object represents.
    pub domain_name: String,
    ///Whether this payment method domain is enabled. If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements.
    pub enabled: bool,
    ///Indicates the status of a specific payment method on a payment method domain.
    pub google_pay: PaymentMethodDomainResourcePaymentMethodStatus,
    ///Unique identifier for the object.
    pub id: String,
    ///Indicates the status of a specific payment method on a payment method domain.
    pub link: PaymentMethodDomainResourcePaymentMethodStatus,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Indicates the status of a specific payment method on a payment method domain.
    pub paypal: PaymentMethodDomainResourcePaymentMethodStatus,
}
impl std::fmt::Display for PaymentMethodDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}