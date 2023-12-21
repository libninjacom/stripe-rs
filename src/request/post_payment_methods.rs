use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_payment_methods`].

On request success, this will return a [`PaymentMethod`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPaymentMethodsRequest {}
impl PostPaymentMethodsRequest {}
impl FluentRequest<'_, PostPaymentMethodsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostPaymentMethodsRequest> {
    type Output = httpclient::InMemoryResult<PaymentMethod>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/payment_methods";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}