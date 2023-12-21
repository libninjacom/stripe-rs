use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_subscriptions`].

On request success, this will return a [`GetSubscriptionsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionsRequest {
    pub automatic_tax: Option<AutomaticTaxFilterParams>,
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
impl GetSubscriptionsRequest {}
impl FluentRequest<'_, GetSubscriptionsRequest> {
    pub fn automatic_tax(mut self, automatic_tax: AutomaticTaxFilterParams) -> Self {
        self.params.automatic_tax = Some(automatic_tax);
        self
    }
    pub fn collection_method(mut self, collection_method: &str) -> Self {
        self.params.collection_method = Some(collection_method.to_owned());
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.params.created = Some(created);
        self
    }
    pub fn current_period_end(mut self, current_period_end: serde_json::Value) -> Self {
        self.params.current_period_end = Some(current_period_end);
        self
    }
    pub fn current_period_start(
        mut self,
        current_period_start: serde_json::Value,
    ) -> Self {
        self.params.current_period_start = Some(current_period_start);
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
    pub fn price(mut self, price: &str) -> Self {
        self.params.price = Some(price.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.params.status = Some(status.to_owned());
        self
    }
    pub fn test_clock(mut self, test_clock: &str) -> Self {
        self.params.test_clock = Some(test_clock.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetSubscriptionsRequest> {
    type Output = httpclient::InMemoryResult<GetSubscriptionsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/subscriptions";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}