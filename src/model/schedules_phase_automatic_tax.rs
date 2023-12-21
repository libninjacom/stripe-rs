use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchedulesPhaseAutomaticTax {
    ///Whether Stripe automatically computes tax on invoices created during this phase.
    pub enabled: bool,
}
impl std::fmt::Display for SchedulesPhaseAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}