use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_climate_orders_order`].

On request success, this will return a [`ClimateOrder`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostClimateOrdersOrderRequest {
    pub order: String,
}
impl PostClimateOrdersOrderRequest {}
impl FluentRequest<'_, PostClimateOrdersOrderRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostClimateOrdersOrderRequest> {
    type Output = httpclient::InMemoryResult<ClimateOrder>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/climate/orders/{order}", order = self.params.order);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}