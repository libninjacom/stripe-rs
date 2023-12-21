use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_issuing_disputes_dispute_submit`].

On request success, this will return a [`IssuingDispute`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostIssuingDisputesDisputeSubmitRequest {
    pub dispute: String,
}
impl PostIssuingDisputesDisputeSubmitRequest {}
impl FluentRequest<'_, PostIssuingDisputesDisputeSubmitRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostIssuingDisputesDisputeSubmitRequest> {
    type Output = httpclient::InMemoryResult<IssuingDispute>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/issuing/disputes/{dispute}/submit", dispute = self.params.dispute
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}