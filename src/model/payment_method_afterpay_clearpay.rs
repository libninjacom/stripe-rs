use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodAfterpayClearpay {}
impl std::fmt::Display for PaymentMethodAfterpayClearpay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}