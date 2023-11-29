use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostTransfersTransferRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub transfer: String,
}
impl<'a> PostTransfersTransferRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Transfer> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/v1/transfers/{transfer}", transfer = self.transfer));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostTransfersTransferRequest<'a> {
    type Output = httpclient::InMemoryResult<Transfer>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}