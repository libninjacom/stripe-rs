use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_climate_reservations_order_confirm`].

On request success, this will return a [`ClimateOrder`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostClimateReservationsOrderConfirmRequest {
    pub order: String,
}
impl PostClimateReservationsOrderConfirmRequest {}
impl FluentRequest<'_, PostClimateReservationsOrderConfirmRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostClimateReservationsOrderConfirmRequest> {
    type Output = httpclient::InMemoryResult<ClimateOrder>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/climate/reservations/{order}/confirm", order = self.params.order
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}