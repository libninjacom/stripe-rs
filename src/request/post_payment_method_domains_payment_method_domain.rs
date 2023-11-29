use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPaymentMethodDomainsPaymentMethodDomainRequest {
    pub payment_method_domain: String,
}
impl PostPaymentMethodDomainsPaymentMethodDomainRequest {}
impl FluentRequest<'_, PostPaymentMethodDomainsPaymentMethodDomainRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostPaymentMethodDomainsPaymentMethodDomainRequest> {
    type Output = httpclient::InMemoryResult<PaymentMethodDomain>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!(
                "/v1/payment_method_domains/{payment_method_domain}",
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