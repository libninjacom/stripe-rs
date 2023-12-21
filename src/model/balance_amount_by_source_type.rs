use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BalanceAmountBySourceType {
    ///Amount for bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<i64>,
    ///Amount for card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<i64>,
    ///Amount for FPX.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<i64>,
}
impl std::fmt::Display for BalanceAmountBySourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}