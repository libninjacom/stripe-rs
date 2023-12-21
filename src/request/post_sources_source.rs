use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_sources_source`].

On request success, this will return a [`Source`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSourcesSourceRequest {
    pub source: String,
}
impl PostSourcesSourceRequest {}
impl FluentRequest<'_, PostSourcesSourceRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostSourcesSourceRequest> {
    type Output = httpclient::InMemoryResult<Source>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/sources/{source}", source = self.params.source);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}