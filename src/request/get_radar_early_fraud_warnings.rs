use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_radar_early_fraud_warnings`].

On request success, this will return a [`GetRadarEarlyFraudWarningsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRadarEarlyFraudWarningsRequest {
    pub charge: Option<String>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
}
impl GetRadarEarlyFraudWarningsRequest {}
impl FluentRequest<'_, GetRadarEarlyFraudWarningsRequest> {
    pub fn charge(mut self, charge: &str) -> Self {
        self.params.charge = Some(charge.to_owned());
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.params.created = Some(created);
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
    pub fn payment_intent(mut self, payment_intent: &str) -> Self {
        self.params.payment_intent = Some(payment_intent.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetRadarEarlyFraudWarningsRequest> {
    type Output = httpclient::InMemoryResult<GetRadarEarlyFraudWarningsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/radar/early_fraud_warnings";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}