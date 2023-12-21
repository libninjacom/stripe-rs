use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_apps_secrets_delete`].

On request success, this will return a [`AppsSecret`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostAppsSecretsDeleteRequest {}
impl PostAppsSecretsDeleteRequest {}
impl FluentRequest<'_, PostAppsSecretsDeleteRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostAppsSecretsDeleteRequest> {
    type Output = httpclient::InMemoryResult<AppsSecret>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/apps/secrets/delete";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}