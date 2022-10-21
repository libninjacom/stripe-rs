use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetCheckoutSessionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: Option<String>,
    pub customer_details: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
    pub subscription: Option<String>,
}
impl<'a> GetCheckoutSessionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/checkout/sessions");
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer_details {
            r = r.push_query("customer_details", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.payment_intent {
            r = r.push_query("payment_intent", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription {
            r = r.push_query("subscription", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn customer(mut self, customer: &str) -> Self {
        self.customer = Some(customer.to_owned());
        self
    }
    pub fn customer_details(mut self, customer_details: serde_json::Value) -> Self {
        self.customer_details = Some(customer_details);
        self
    }
    pub fn ending_before(mut self, ending_before: &str) -> Self {
        self.ending_before = Some(ending_before.to_owned());
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn payment_intent(mut self, payment_intent: &str) -> Self {
        self.payment_intent = Some(payment_intent.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn subscription(mut self, subscription: &str) -> Self {
        self.subscription = Some(subscription.to_owned());
        self
    }
}
