use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPaymentMethodsPaymentMethodAttachRequest {
    pub payment_method: String,
}
impl PostPaymentMethodsPaymentMethodAttachRequest {}
impl FluentRequest<'_, PostPaymentMethodsPaymentMethodAttachRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostPaymentMethodsPaymentMethodAttachRequest> {
    type Output = httpclient::InMemoryResult<PaymentMethod>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!(
                "/v1/payment_methods/{payment_method}/attach", payment_method = self
                .params.payment_method
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}