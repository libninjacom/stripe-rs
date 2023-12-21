use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_disputes_dispute`].

On request success, this will return a [`Dispute`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostDisputesDisputeRequest {
    pub dispute: String,
}
impl PostDisputesDisputeRequest {}
impl FluentRequest<'_, PostDisputesDisputeRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostDisputesDisputeRequest> {
    type Output = httpclient::InMemoryResult<Dispute>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/disputes/{dispute}", dispute = self.params.dispute);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}