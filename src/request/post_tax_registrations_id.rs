use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_tax_registrations_id`].

On request success, this will return a [`TaxRegistration`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTaxRegistrationsIdRequest {
    pub id: String,
}
impl PostTaxRegistrationsIdRequest {}
impl FluentRequest<'_, PostTaxRegistrationsIdRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostTaxRegistrationsIdRequest> {
    type Output = httpclient::InMemoryResult<TaxRegistration>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/tax/registrations/{id}", id = self.params.id);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}