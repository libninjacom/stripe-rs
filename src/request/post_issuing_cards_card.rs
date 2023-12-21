use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_issuing_cards_card`].

On request success, this will return a [`IssuingCard`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostIssuingCardsCardRequest {
    pub card: String,
}
impl PostIssuingCardsCardRequest {}
impl FluentRequest<'_, PostIssuingCardsCardRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostIssuingCardsCardRequest> {
    type Output = httpclient::InMemoryResult<IssuingCard>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/issuing/cards/{card}", card = self.params.card);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}