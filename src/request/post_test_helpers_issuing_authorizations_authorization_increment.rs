use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_test_helpers_issuing_authorizations_authorization_increment`].

On request success, this will return a [`IssuingAuthorization`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTestHelpersIssuingAuthorizationsAuthorizationIncrementRequest {
    pub authorization: String,
}
impl PostTestHelpersIssuingAuthorizationsAuthorizationIncrementRequest {}
impl FluentRequest<
    '_,
    PostTestHelpersIssuingAuthorizationsAuthorizationIncrementRequest,
> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<
    'a,
    PostTestHelpersIssuingAuthorizationsAuthorizationIncrementRequest,
> {
    type Output = httpclient::InMemoryResult<IssuingAuthorization>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/test_helpers/issuing/authorizations/{authorization}/increment",
                authorization = self.params.authorization
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}