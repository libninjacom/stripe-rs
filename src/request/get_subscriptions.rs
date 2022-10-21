use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSubscriptionsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub collection_method: Option<String>,
    pub created: Option<serde_json::Value>,
    pub current_period_end: Option<serde_json::Value>,
    pub current_period_start: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub price: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub test_clock: Option<String>,
}
impl<'a> GetSubscriptionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/subscriptions");
        if let Some(ref unwrapped) = self.collection_method {
            r = r.push_query("collection_method", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.current_period_end {
            r = r.push_query("current_period_end", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.current_period_start {
            r = r.push_query("current_period_start", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.push_query("customer", &unwrapped.to_string());
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
        if let Some(ref unwrapped) = self.price {
            r = r.push_query("price", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.test_clock {
            r = r.push_query("test_clock", &unwrapped.to_string());
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
    pub fn collection_method(mut self, collection_method: &str) -> Self {
        self.collection_method = Some(collection_method.to_owned());
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn current_period_end(mut self, current_period_end: serde_json::Value) -> Self {
        self.current_period_end = Some(current_period_end);
        self
    }
    pub fn current_period_start(
        mut self,
        current_period_start: serde_json::Value,
    ) -> Self {
        self.current_period_start = Some(current_period_start);
        self
    }
    pub fn customer(mut self, customer: &str) -> Self {
        self.customer = Some(customer.to_owned());
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
    pub fn price(mut self, price: &str) -> Self {
        self.price = Some(price.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
    pub fn test_clock(mut self, test_clock: &str) -> Self {
        self.test_clock = Some(test_clock.to_owned());
        self
    }
}
