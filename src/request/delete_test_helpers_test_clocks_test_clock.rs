use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_test_helpers_test_clocks_test_clock`].

On request success, this will return a [`DeletedTestHelpersTestClock`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTestHelpersTestClocksTestClockRequest {
    pub test_clock: String,
}
impl DeleteTestHelpersTestClocksTestClockRequest {}
impl FluentRequest<'_, DeleteTestHelpersTestClocksTestClockRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteTestHelpersTestClocksTestClockRequest> {
    type Output = httpclient::InMemoryResult<DeletedTestHelpersTestClock>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/test_helpers/test_clocks/{test_clock}", test_clock = self.params
                .test_clock
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}