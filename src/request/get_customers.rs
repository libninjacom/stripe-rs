use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
use serde::{Serialize, Deserialize};
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Serialize, Deserialize, Clone)]
pub struct GetCustomersRequest {
    pub created: Option<serde_json::Value>,
    pub email: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub test_clock: Option<String>,
}

impl FluentRequest<'_, GetCustomersRequest> {
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.params.created = Some(created);
        self
    }
    pub fn email(mut self, email: &str) -> Self {
        self.params.email = Some(email.to_owned());
        self
    }
    pub fn ending_before(mut self, ending_before: &str) -> Self {
        self.params.ending_before = Some(ending_before.to_owned());
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.params.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn test_clock(mut self, test_clock: &str) -> Self {
        self.params.test_clock = Some(test_clock.to_owned());
        self
    }
}

impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetCustomersRequest> {
    type Output = httpclient::InMemoryResult<CustomerResourceCustomerList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let mut r = self.client.client.get("/v1/customers");
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}