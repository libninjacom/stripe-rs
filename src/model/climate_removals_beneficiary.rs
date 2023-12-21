use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClimateRemovalsBeneficiary {
    ///Publicly displayable name for the end beneficiary of carbon removal.
    pub public_name: String,
}
impl std::fmt::Display for ClimateRemovalsBeneficiary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}