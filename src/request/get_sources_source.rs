use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSourcesSourceRequest {
    pub client_secret: Option<String>,
    pub expand: Option<Vec<String>>,
    pub source: String,
}
impl GetSourcesSourceRequest {}
impl FluentRequest<'_, GetSourcesSourceRequest> {
    pub fn client_secret(mut self, client_secret: &str) -> Self {
        self.params.client_secret = Some(client_secret.to_owned());
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetSourcesSourceRequest> {
    type Output = httpclient::InMemoryResult<Source>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!("/v1/sources/{source}", source = self.params.source);
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}