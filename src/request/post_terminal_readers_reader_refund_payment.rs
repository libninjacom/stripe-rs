use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostTerminalReadersReaderRefundPaymentRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub reader: String,
}
impl<'a> PostTerminalReadersReaderRefundPaymentRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<TerminalReader> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/terminal/readers/{reader}/refund_payment", reader = self.reader
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture
for PostTerminalReadersReaderRefundPaymentRequest<'a> {
    type Output = httpclient::InMemoryResult<TerminalReader>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}