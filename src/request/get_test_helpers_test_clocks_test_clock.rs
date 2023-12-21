use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_test_helpers_test_clocks_test_clock`].

On request success, this will return a [`TestHelpersTestClock`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTestHelpersTestClocksTestClockRequest {
    pub expand: Option<Vec<String>>,
    pub test_clock: String,
}
impl GetTestHelpersTestClocksTestClockRequest {}
impl FluentRequest<'_, GetTestHelpersTestClocksTestClockRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetTestHelpersTestClocksTestClockRequest> {
    type Output = httpclient::InMemoryResult<TestHelpersTestClock>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/test_helpers/test_clocks/{test_clock}", test_clock = self.params
                .test_clock
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}