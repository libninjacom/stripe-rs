use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostTestHelpersTreasuryOutboundTransfersOutboundTransferPostRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub outbound_transfer: String,
}
impl<'a> PostTestHelpersTreasuryOutboundTransfersOutboundTransferPostRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<TreasuryOutboundTransfer> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/treasury/outbound_transfers/{outbound_transfer}/post",
                    outbound_transfer = self.outbound_transfer
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture
for PostTestHelpersTreasuryOutboundTransfersOutboundTransferPostRequest<'a> {
    type Output = httpclient::InMemoryResult<TreasuryOutboundTransfer>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}