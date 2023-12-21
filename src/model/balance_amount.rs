use serde::{Serialize, Deserialize};
use super::BalanceAmountBySourceType;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BalanceAmount {
    ///Balance amount.
    pub amount: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_types: Option<BalanceAmountBySourceType>,
}
impl std::fmt::Display for BalanceAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}