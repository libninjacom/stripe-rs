use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodDetailsCardInstallmentsPlan {
    ///For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /**For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
One of `month`.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    ///Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for PaymentMethodDetailsCardInstallmentsPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}