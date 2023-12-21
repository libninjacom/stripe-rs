use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_quotes_quote_finalize`].

On request success, this will return a [`Quote`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostQuotesQuoteFinalizeRequest {
    pub quote: String,
}
impl PostQuotesQuoteFinalizeRequest {}
impl FluentRequest<'_, PostQuotesQuoteFinalizeRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostQuotesQuoteFinalizeRequest> {
    type Output = httpclient::InMemoryResult<Quote>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/quotes/{quote}/finalize", quote = self.params.quote);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}