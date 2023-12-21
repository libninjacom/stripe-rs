use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_issuing_disputes`].

On request success, this will return a [`IssuingDispute`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostIssuingDisputesRequest {}
impl PostIssuingDisputesRequest {}
impl FluentRequest<'_, PostIssuingDisputesRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostIssuingDisputesRequest> {
    type Output = httpclient::InMemoryResult<IssuingDispute>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/issuing/disputes";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}