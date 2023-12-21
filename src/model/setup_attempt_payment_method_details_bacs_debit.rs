use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetupAttemptPaymentMethodDetailsBacsDebit {}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}