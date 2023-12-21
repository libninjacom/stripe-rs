use serde::{Serialize, Deserialize};
use super::ProductFeature;
/**Products describe the specific goods or services you offer to your customers.
For example, you might offer a Standard and Premium version of your goods or service; each version would be a separate Product.
They can be used in conjunction with [Prices](https://stripe.com/docs/api#prices) to configure pricing in Payment Links, Checkout, and Subscriptions.

Related guides: [Set up a subscription](https://stripe.com/docs/billing/subscriptions/set-up-subscription),
[share a Payment Link](https://stripe.com/docs/payment-links),
[accept payments with Checkout](https://stripe.com/docs/payments/accept-a-payment#create-product-prices-upfront),
and more about [Products and Prices](https://stripe.com/docs/products-prices/overview)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Product {
    ///Whether the product is currently available for purchase.
    pub active: bool,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The ID of the [Price](https://stripe.com/docs/api/prices) object that is the default price for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_price: Option<serde_json::Value>,
    ///The product's description, meant to be displayable to the customer. Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///A list of up to 15 features for this product. These are displayed in [pricing tables](https://stripe.com/docs/payments/checkout/pricing-table).
    pub features: Vec<ProductFeature>,
    ///Unique identifier for the object.
    pub id: String,
    ///A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    pub images: Vec<String>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///The product's name, meant to be displayable to the customer.
    pub name: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The dimensions of this product for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<serde_json::Value>,
    ///Whether this product is shipped (i.e., physical goods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,
    ///Extra information about a product which will appear on your customer's credit card statement. In the case that multiple products are billed at once, the first statement descriptor will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    ///A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<serde_json::Value>,
    ///A label that represents units of this product. When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,
    ///Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: i64,
    ///A URL of a publicly-accessible webpage for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}