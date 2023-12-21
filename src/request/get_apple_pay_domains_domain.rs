use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_apple_pay_domains_domain`].

On request success, this will return a [`ApplePayDomain`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplePayDomainsDomainRequest {
    pub domain: String,
    pub expand: Option<Vec<String>>,
}
impl GetApplePayDomainsDomainRequest {}
impl FluentRequest<'_, GetApplePayDomainsDomainRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetApplePayDomainsDomainRequest> {
    type Output = httpclient::InMemoryResult<ApplePayDomain>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/apple_pay/domains/{domain}", domain = self.params.domain
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}