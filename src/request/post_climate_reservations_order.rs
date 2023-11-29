use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostClimateReservationsOrderRequest {
    pub order: String,
}
impl PostClimateReservationsOrderRequest {}
impl FluentRequest<'_, PostClimateReservationsOrderRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostClimateReservationsOrderRequest> {
    type Output = httpclient::InMemoryResult<ClimateOrder>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!(
                "/v1/climate/reservations/{order}", order = self.params.order
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}