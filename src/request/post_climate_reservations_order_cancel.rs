use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostClimateReservationsOrderCancelRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub order: String,
}
impl<'a> PostClimateReservationsOrderCancelRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ClimateOrder> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!("/v1/climate/reservations/{order}/cancel", order = self.order),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostClimateReservationsOrderCancelRequest<'a> {
    type Output = httpclient::InMemoryResult<ClimateOrder>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}