use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProductsRequest {
    pub active: Option<bool>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub ids: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub shippable: Option<bool>,
    pub starting_after: Option<String>,
    pub url: Option<String>,
}
impl GetProductsRequest {}
impl FluentRequest<'_, GetProductsRequest> {
    pub fn active(mut self, active: bool) -> Self {
        self.params.active = Some(active);
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
    pub fn ids(mut self, ids: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.params.ids = Some(ids.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    pub fn shippable(mut self, shippable: bool) -> Self {
        self.params.shippable = Some(shippable);
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn url(mut self, url: &str) -> Self {
        self.params.url = Some(url.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetProductsRequest> {
    type Output = httpclient::InMemoryResult<ProductList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/v1/products";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}