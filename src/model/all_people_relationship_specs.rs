use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AllPeopleRelationshipSpecs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_guardian: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
}
impl std::fmt::Display for AllPeopleRelationshipSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}