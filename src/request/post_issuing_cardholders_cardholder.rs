use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_issuing_cardholders_cardholder`].

On request success, this will return a [`IssuingCardholder`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostIssuingCardholdersCardholderRequest {
    pub cardholder: String,
}
impl PostIssuingCardholdersCardholderRequest {}
impl FluentRequest<'_, PostIssuingCardholdersCardholderRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostIssuingCardholdersCardholderRequest> {
    type Output = httpclient::InMemoryResult<IssuingCardholder>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/issuing/cardholders/{cardholder}", cardholder = self.params
                .cardholder
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}