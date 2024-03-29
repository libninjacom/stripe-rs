use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_apple_pay_domains`].

On request success, this will return a [`ApplePayDomain`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostApplePayDomainsRequest {}
impl PostApplePayDomainsRequest {}
impl FluentRequest<'_, PostApplePayDomainsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostApplePayDomainsRequest> {
    type Output = httpclient::InMemoryResult<ApplePayDomain>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/apple_pay/domains";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}