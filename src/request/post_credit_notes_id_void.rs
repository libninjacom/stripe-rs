use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_credit_notes_id_void`].

On request success, this will return a [`CreditNote`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCreditNotesIdVoidRequest {
    pub id: String,
}
impl PostCreditNotesIdVoidRequest {}
impl FluentRequest<'_, PostCreditNotesIdVoidRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostCreditNotesIdVoidRequest> {
    type Output = httpclient::InMemoryResult<CreditNote>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/credit_notes/{id}/void", id = self.params.id);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}