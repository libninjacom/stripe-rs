use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTreasuryInboundTransfersInboundTransferCancelRequest {
    pub inbound_transfer: String,
}
impl PostTreasuryInboundTransfersInboundTransferCancelRequest {}
impl FluentRequest<'_, PostTreasuryInboundTransfersInboundTransferCancelRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTreasuryInboundTransfersInboundTransferCancelRequest> {
    type Output = httpclient::InMemoryResult<TreasuryInboundTransfer>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!(
                "/v1/treasury/inbound_transfers/{inbound_transfer}/cancel",
                inbound_transfer = self.params.inbound_transfer
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}