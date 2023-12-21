use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_prices`].

On request success, this will return a [`GetPricesResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPricesRequest {
    pub active: Option<bool>,
    pub created: Option<serde_json::Value>,
    pub currency: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub lookup_keys: Option<Vec<String>>,
    pub product: Option<String>,
    pub recurring: Option<AllPricesRecurringParams>,
    pub starting_after: Option<String>,
    pub type_: Option<String>,
}
impl GetPricesRequest {}
impl FluentRequest<'_, GetPricesRequest> {
    pub fn active(mut self, active: bool) -> Self {
        self.params.active = Some(active);
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.params.created = Some(created);
        self
    }
    pub fn currency(mut self, currency: &str) -> Self {
        self.params.currency = Some(currency.to_owned());
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
    pub fn lookup_keys(
        mut self,
        lookup_keys: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .lookup_keys = Some(
            lookup_keys.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn product(mut self, product: &str) -> Self {
        self.params.product = Some(product.to_owned());
        self
    }
    pub fn recurring(mut self, recurring: AllPricesRecurringParams) -> Self {
        self.params.recurring = Some(recurring);
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.params.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetPricesRequest> {
    type Output = httpclient::InMemoryResult<GetPricesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/prices";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}