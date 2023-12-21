use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_payouts_payout_cancel`].

On request success, this will return a [`Payout`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPayoutsPayoutCancelRequest {
    pub payout: String,
}
impl PostPayoutsPayoutCancelRequest {}
impl FluentRequest<'_, PostPayoutsPayoutCancelRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostPayoutsPayoutCancelRequest> {
    type Output = httpclient::InMemoryResult<Payout>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/payouts/{payout}/cancel", payout = self.params.payout
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}