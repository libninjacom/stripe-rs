
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetupAttemptPaymentMethodDetailsSofort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsSofort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}