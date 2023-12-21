use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_payment_method_domains`].

On request success, this will return a [`PaymentMethodDomain`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPaymentMethodDomainsRequest {}
impl PostPaymentMethodDomainsRequest {}
impl FluentRequest<'_, PostPaymentMethodDomainsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostPaymentMethodDomainsRequest> {
    type Output = httpclient::InMemoryResult<PaymentMethodDomain>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/payment_method_domains";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}