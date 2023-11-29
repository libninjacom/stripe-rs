use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostDisputesDisputeRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub dispute: String,
}
impl<'a> PostDisputesDisputeRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Dispute> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/v1/disputes/{dispute}", dispute = self.dispute));
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostDisputesDisputeRequest<'a> {
    type Output = httpclient::InMemoryResult<Dispute>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}