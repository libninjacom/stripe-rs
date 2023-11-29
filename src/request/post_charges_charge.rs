use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostChargesChargeRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub charge: String,
}
impl<'a> PostChargesChargeRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Charge> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/v1/charges/{charge}", charge = self.charge));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostChargesChargeRequest<'a> {
    type Output = httpclient::InMemoryResult<Charge>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}