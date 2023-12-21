use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodIdeal {
    ///The customer's bank, if provided. Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `n26`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,
    ///The Bank Identifier Code of the customer's bank, if the bank was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
}
impl std::fmt::Display for PaymentMethodIdeal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}