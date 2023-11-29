use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionSchedulesRequest {
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
impl GetSubscriptionSchedulesRequest {}
impl FluentRequest<'_, GetSubscriptionSchedulesRequest> {
    pub fn canceled_at(mut self, canceled_at: serde_json::Value) -> Self {
        self.params.canceled_at = Some(canceled_at);
        self
    }
    pub fn completed_at(mut self, completed_at: serde_json::Value) -> Self {
        self.params.completed_at = Some(completed_at);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.params.created = Some(created);
        self
    }
    pub fn customer(mut self, customer: &str) -> Self {
        self.params.customer = Some(customer.to_owned());
        self
    }
    pub fn ending_before(mut self, ending_before: &str) -> Self {
        self.params.ending_before = Some(ending_before.to_owned());
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    pub fn released_at(mut self, released_at: serde_json::Value) -> Self {
        self.params.released_at = Some(released_at);
        self
    }
    pub fn scheduled(mut self, scheduled: bool) -> Self {
        self.params.scheduled = Some(scheduled);
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetSubscriptionSchedulesRequest> {
    type Output = httpclient::InMemoryResult<SubscriptionSchedulesResourceScheduleList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/v1/subscription_schedules";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}