use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_charges_charge_refunds_refund`].

On request success, this will return a [`Refund`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostChargesChargeRefundsRefundRequest {
    pub charge: String,
    pub refund: String,
}
impl PostChargesChargeRefundsRefundRequest {}
impl FluentRequest<'_, PostChargesChargeRefundsRefundRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostChargesChargeRefundsRefundRequest> {
    type Output = httpclient::InMemoryResult<Refund>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/charges/{charge}/refunds/{refund}", charge = self.params.charge,
                refund = self.params.refund
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}