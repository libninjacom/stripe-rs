use serde::{Serialize, Deserialize};
use super::{
    SetupAttemptPaymentMethodDetailsAcssDebit,
    SetupAttemptPaymentMethodDetailsAuBecsDebit,
    SetupAttemptPaymentMethodDetailsBacsDebit,
    SetupAttemptPaymentMethodDetailsBancontact, SetupAttemptPaymentMethodDetailsBoleto,
    SetupAttemptPaymentMethodDetailsCard, SetupAttemptPaymentMethodDetailsCardPresent,
    SetupAttemptPaymentMethodDetailsCashapp, SetupAttemptPaymentMethodDetailsIdeal,
    SetupAttemptPaymentMethodDetailsKlarna, SetupAttemptPaymentMethodDetailsLink,
    SetupAttemptPaymentMethodDetailsPaypal, SetupAttemptPaymentMethodDetailsSepaDebit,
    SetupAttemptPaymentMethodDetailsSofort, SetupAttemptPaymentMethodDetailsUsBankAccount,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetupAttemptPaymentMethodDetails {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<SetupAttemptPaymentMethodDetailsAcssDebit>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<SetupAttemptPaymentMethodDetailsAuBecsDebit>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<SetupAttemptPaymentMethodDetailsBacsDebit>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<SetupAttemptPaymentMethodDetailsBancontact>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<SetupAttemptPaymentMethodDetailsBoleto>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<SetupAttemptPaymentMethodDetailsCard>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<SetupAttemptPaymentMethodDetailsCardPresent>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<SetupAttemptPaymentMethodDetailsCashapp>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<SetupAttemptPaymentMethodDetailsIdeal>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<SetupAttemptPaymentMethodDetailsKlarna>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<SetupAttemptPaymentMethodDetailsLink>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<SetupAttemptPaymentMethodDetailsPaypal>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<SetupAttemptPaymentMethodDetailsSepaDebit>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<SetupAttemptPaymentMethodDetailsSofort>,
    ///The type of the payment method used in the SetupIntent (e.g., `card`). An additional hash is included on `payment_method_details` with a name matching this value. It contains confirmation-specific information for the payment method.
    #[serde(rename = "type")]
    pub type_: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<SetupAttemptPaymentMethodDetailsUsBankAccount>,
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}