use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_tax_registrations`].

On request success, this will return a [`TaxRegistration`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTaxRegistrationsRequest {}
impl PostTaxRegistrationsRequest {}
impl FluentRequest<'_, PostTaxRegistrationsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostTaxRegistrationsRequest> {
    type Output = httpclient::InMemoryResult<TaxRegistration>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/tax/registrations";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}