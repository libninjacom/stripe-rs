use serde::{Serialize, Deserialize};
use super::ClimateRemovalsLocation;
///A supplier of carbon removal.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClimateSupplier {
    ///Unique identifier for the object.
    pub id: String,
    ///Link to a webpage to learn more about the supplier.
    pub info_url: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///The locations in which this supplier operates.
    pub locations: Vec<ClimateRemovalsLocation>,
    ///Name of this carbon removal supplier.
    pub name: String,
    ///String representing the object’s type. Objects of the same type share the same value.
    pub object: String,
    ///The scientific pathway used for carbon removal.
    pub removal_pathway: String,
}
impl std::fmt::Display for ClimateSupplier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}