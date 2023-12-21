use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_treasury_outbound_transfers_outbound_transfer_cancel`].

On request success, this will return a [`TreasuryOutboundTransfer`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTreasuryOutboundTransfersOutboundTransferCancelRequest {
    pub outbound_transfer: String,
}
impl PostTreasuryOutboundTransfersOutboundTransferCancelRequest {}
impl FluentRequest<'_, PostTreasuryOutboundTransfersOutboundTransferCancelRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTreasuryOutboundTransfersOutboundTransferCancelRequest> {
    type Output = httpclient::InMemoryResult<TreasuryOutboundTransfer>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/treasury/outbound_transfers/{outbound_transfer}/cancel",
                outbound_transfer = self.params.outbound_transfer
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}