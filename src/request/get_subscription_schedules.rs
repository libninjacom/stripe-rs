use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetSubscriptionSchedulesRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub canceled_at: Option<serde_json::Value>,
    pub completed_at: Option<serde_json::Value>,
    pub created: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub released_at: Option<serde_json::Value>,
    pub scheduled: Option<bool>,
    pub starting_after: Option<String>,
}
impl<'a> GetSubscriptionSchedulesRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<SubscriptionSchedulesResourceScheduleList> {
        let mut r = self.http_client.client.get("/v1/subscription_schedules");
        if let Some(ref unwrapped) = self.canceled_at {
            r = r.query("canceled_at", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.completed_at {
            r = r.query("completed_at", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.released_at {
            r = r.query("released_at", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.scheduled {
            r = r.query("scheduled", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.query("starting_after", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn canceled_at(mut self, canceled_at: serde_json::Value) -> Self {
        self.canceled_at = Some(canceled_at);
        self
    }
    pub fn completed_at(mut self, completed_at: serde_json::Value) -> Self {
        self.completed_at = Some(completed_at);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
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
    pub fn released_at(mut self, released_at: serde_json::Value) -> Self {
        self.released_at = Some(released_at);
        self
    }
    pub fn scheduled(mut self, scheduled: bool) -> Self {
        self.scheduled = Some(scheduled);
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetSubscriptionSchedulesRequest<'a> {
    type Output = httpclient::InMemoryResult<SubscriptionSchedulesResourceScheduleList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}