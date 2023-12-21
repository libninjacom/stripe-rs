use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_terminal_locations_location`].

On request success, this will return a [`DeletedTerminalLocation`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTerminalLocationsLocationRequest {
    pub location: String,
}
impl DeleteTerminalLocationsLocationRequest {}
impl FluentRequest<'_, DeleteTerminalLocationsLocationRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteTerminalLocationsLocationRequest> {
    type Output = httpclient::InMemoryResult<DeletedTerminalLocation>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/terminal/locations/{location}", location = self.params.location
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}