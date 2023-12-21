use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_credit_notes`].

On request success, this will return a [`CreditNote`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCreditNotesRequest {}
impl PostCreditNotesRequest {}
impl FluentRequest<'_, PostCreditNotesRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostCreditNotesRequest> {
    type Output = httpclient::InMemoryResult<CreditNote>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/credit_notes";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}