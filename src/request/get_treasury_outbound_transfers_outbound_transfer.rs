use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_treasury_outbound_transfers_outbound_transfer`].

On request success, this will return a [`TreasuryOutboundTransfer`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTreasuryOutboundTransfersOutboundTransferRequest {
    pub expand: Option<Vec<String>>,
    pub outbound_transfer: String,
}
impl GetTreasuryOutboundTransfersOutboundTransferRequest {}
impl FluentRequest<'_, GetTreasuryOutboundTransfersOutboundTransferRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetTreasuryOutboundTransfersOutboundTransferRequest> {
    type Output = httpclient::InMemoryResult<TreasuryOutboundTransfer>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/treasury/outbound_transfers/{outbound_transfer}", outbound_transfer
                = self.params.outbound_transfer
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}