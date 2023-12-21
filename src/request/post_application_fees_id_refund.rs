use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_application_fees_id_refund`].

On request success, this will return a [`ApplicationFee`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostApplicationFeesIdRefundRequest {
    pub id: String,
}
impl PostApplicationFeesIdRefundRequest {}
impl FluentRequest<'_, PostApplicationFeesIdRefundRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostApplicationFeesIdRefundRequest> {
    type Output = httpclient::InMemoryResult<ApplicationFee>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/application_fees/{id}/refund", id = self.params.id);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}