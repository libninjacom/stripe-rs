use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_payment_intents_intent_verify_microdeposits`].

On request success, this will return a [`PaymentIntent`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPaymentIntentsIntentVerifyMicrodepositsRequest {
    pub intent: String,
}
impl PostPaymentIntentsIntentVerifyMicrodepositsRequest {}
impl FluentRequest<'_, PostPaymentIntentsIntentVerifyMicrodepositsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostPaymentIntentsIntentVerifyMicrodepositsRequest> {
    type Output = httpclient::InMemoryResult<PaymentIntent>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/payment_intents/{intent}/verify_microdeposits", intent = self.params
                .intent
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}