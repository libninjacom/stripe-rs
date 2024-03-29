use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_application_fees_id_refunds`].

On request success, this will return a [`FeeRefund`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostApplicationFeesIdRefundsRequest {
    pub id: String,
}
impl PostApplicationFeesIdRefundsRequest {}
impl FluentRequest<'_, PostApplicationFeesIdRefundsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostApplicationFeesIdRefundsRequest> {
    type Output = httpclient::InMemoryResult<FeeRefund>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/application_fees/{id}/refunds", id = self.params.id);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}