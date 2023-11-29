use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostTransfersIdReversalsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostTransfersIdReversalsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<TransferReversal> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/v1/transfers/{id}/reversals", id = self.id));
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostTransfersIdReversalsRequest<'a> {
    type Output = httpclient::InMemoryResult<TransferReversal>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}