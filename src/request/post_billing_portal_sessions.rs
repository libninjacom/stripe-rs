use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_billing_portal_sessions`].

On request success, this will return a [`BillingPortalSession`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostBillingPortalSessionsRequest {}
impl PostBillingPortalSessionsRequest {}
impl FluentRequest<'_, PostBillingPortalSessionsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostBillingPortalSessionsRequest> {
    type Output = httpclient::InMemoryResult<BillingPortalSession>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/billing_portal/sessions";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}