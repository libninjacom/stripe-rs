use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_charges_charge_refund`].

On request success, this will return a [`Charge`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostChargesChargeRefundRequest {
    pub charge: String,
}
impl PostChargesChargeRefundRequest {}
impl FluentRequest<'_, PostChargesChargeRefundRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostChargesChargeRefundRequest> {
    type Output = httpclient::InMemoryResult<Charge>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/charges/{charge}/refund", charge = self.params.charge
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}