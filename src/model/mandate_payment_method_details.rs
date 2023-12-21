use serde::{Serialize, Deserialize};
use super::{
    CardMandatePaymentMethodDetails, MandateAcssDebit, MandateAuBecsDebit,
    MandateBacsDebit, MandateCashapp, MandateLink, MandatePaypal, MandateSepaDebit,
    MandateUsBankAccount,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MandatePaymentMethodDetails {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<MandateAcssDebit>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<MandateAuBecsDebit>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<MandateBacsDebit>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardMandatePaymentMethodDetails>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<MandateCashapp>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<MandateLink>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<MandatePaypal>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<MandateSepaDebit>,
    ///This mandate corresponds with a specific payment method type. The `payment_method_details` includes an additional hash with the same name and contains mandate information that's specific to that payment method.
    #[serde(rename = "type")]
    pub type_: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<MandateUsBankAccount>,
}
impl std::fmt::Display for MandatePaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}