use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_charges_charge_capture`].

On request success, this will return a [`Charge`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostChargesChargeCaptureRequest {
    pub charge: String,
}
impl PostChargesChargeCaptureRequest {}
impl FluentRequest<'_, PostChargesChargeCaptureRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostChargesChargeCaptureRequest> {
    type Output = httpclient::InMemoryResult<Charge>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/charges/{charge}/capture", charge = self.params.charge
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}