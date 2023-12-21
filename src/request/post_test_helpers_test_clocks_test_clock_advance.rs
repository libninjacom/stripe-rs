use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_test_helpers_test_clocks_test_clock_advance`].

On request success, this will return a [`TestHelpersTestClock`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTestHelpersTestClocksTestClockAdvanceRequest {
    pub test_clock: String,
}
impl PostTestHelpersTestClocksTestClockAdvanceRequest {}
impl FluentRequest<'_, PostTestHelpersTestClocksTestClockAdvanceRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTestHelpersTestClocksTestClockAdvanceRequest> {
    type Output = httpclient::InMemoryResult<TestHelpersTestClock>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/test_helpers/test_clocks/{test_clock}/advance", test_clock = self
                .params.test_clock
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}