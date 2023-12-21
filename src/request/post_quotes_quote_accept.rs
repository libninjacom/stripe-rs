use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_quotes_quote_accept`].

On request success, this will return a [`Quote`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostQuotesQuoteAcceptRequest {
    pub quote: String,
}
impl PostQuotesQuoteAcceptRequest {}
impl FluentRequest<'_, PostQuotesQuoteAcceptRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostQuotesQuoteAcceptRequest> {
    type Output = httpclient::InMemoryResult<Quote>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/quotes/{quote}/accept", quote = self.params.quote);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}