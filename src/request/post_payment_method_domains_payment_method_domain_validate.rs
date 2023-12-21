use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_payment_method_domains_payment_method_domain_validate`].

On request success, this will return a [`PaymentMethodDomain`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPaymentMethodDomainsPaymentMethodDomainValidateRequest {
    pub payment_method_domain: String,
}
impl PostPaymentMethodDomainsPaymentMethodDomainValidateRequest {}
impl FluentRequest<'_, PostPaymentMethodDomainsPaymentMethodDomainValidateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostPaymentMethodDomainsPaymentMethodDomainValidateRequest> {
    type Output = httpclient::InMemoryResult<PaymentMethodDomain>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/payment_method_domains/{payment_method_domain}/validate",
                payment_method_domain = self.params.payment_method_domain
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}