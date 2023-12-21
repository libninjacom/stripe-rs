use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodAuBecsDebit {
    ///Six-digit number identifying bank and branch associated with this bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsb_number: Option<String>,
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    ///Last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}
impl std::fmt::Display for PaymentMethodAuBecsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}