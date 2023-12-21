use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodAcssDebit {
    ///Name of the bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    ///Institution number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_number: Option<String>,
    ///Last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    ///Transit number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_number: Option<String>,
}
impl std::fmt::Display for PaymentMethodAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}