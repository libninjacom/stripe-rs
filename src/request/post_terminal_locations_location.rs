use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_terminal_locations_location`].

On request success, this will return a [`serde_json::Value`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTerminalLocationsLocationRequest {
    pub location: String,
}
impl PostTerminalLocationsLocationRequest {}
impl FluentRequest<'_, PostTerminalLocationsLocationRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTerminalLocationsLocationRequest> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/terminal/locations/{location}", location = self.params.location
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}