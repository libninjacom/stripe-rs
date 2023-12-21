use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_payouts`].

On request success, this will return a [`Payout`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPayoutsRequest {}
impl PostPayoutsRequest {}
impl FluentRequest<'_, PostPayoutsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostPayoutsRequest> {
    type Output = httpclient::InMemoryResult<Payout>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/payouts";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}