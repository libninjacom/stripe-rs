use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_charges_charge_dispute_close`].

On request success, this will return a [`Dispute`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostChargesChargeDisputeCloseRequest {
    pub charge: String,
}
impl PostChargesChargeDisputeCloseRequest {}
impl FluentRequest<'_, PostChargesChargeDisputeCloseRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostChargesChargeDisputeCloseRequest> {
    type Output = httpclient::InMemoryResult<Dispute>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/charges/{charge}/dispute/close", charge = self.params.charge
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}