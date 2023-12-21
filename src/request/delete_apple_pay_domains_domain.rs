use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_apple_pay_domains_domain`].

On request success, this will return a [`DeletedApplePayDomain`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteApplePayDomainsDomainRequest {
    pub domain: String,
}
impl DeleteApplePayDomainsDomainRequest {}
impl FluentRequest<'_, DeleteApplePayDomainsDomainRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteApplePayDomainsDomainRequest> {
    type Output = httpclient::InMemoryResult<DeletedApplePayDomain>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/apple_pay/domains/{domain}", domain = self.params.domain
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}