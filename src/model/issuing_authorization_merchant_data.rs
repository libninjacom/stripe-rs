use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingAuthorizationMerchantData {
    ///A categorization of the seller's type of business. See our [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
    pub category: String,
    ///The merchant category code for the seller’s business
    pub category_code: String,
    ///City where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///Country where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///Name of the seller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Identifier assigned to the seller by the card network. Different card networks may assign different network_id fields to the same merchant.
    pub network_id: String,
    ///Postal code where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    ///State where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    ///An ID assigned by the seller to the location of the sale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_id: Option<String>,
    ///URL provided by the merchant on a 3DS request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for IssuingAuthorizationMerchantData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}