use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_payment_intents_intent_confirm`].

On request success, this will return a [`PaymentIntent`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPaymentIntentsIntentConfirmRequest {
    pub intent: String,
}
impl PostPaymentIntentsIntentConfirmRequest {}
impl FluentRequest<'_, PostPaymentIntentsIntentConfirmRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostPaymentIntentsIntentConfirmRequest> {
    type Output = httpclient::InMemoryResult<PaymentIntent>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/payment_intents/{intent}/confirm", intent = self.params.intent
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}