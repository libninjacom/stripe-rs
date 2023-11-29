use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteTestHelpersTestClocksTestClockRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub test_clock: String,
}
impl<'a> DeleteTestHelpersTestClocksTestClockRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<DeletedTestHelpersTestClock> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/v1/test_helpers/test_clocks/{test_clock}", test_clock = self
                    .test_clock
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeleteTestHelpersTestClocksTestClockRequest<'a> {
    type Output = httpclient::InMemoryResult<DeletedTestHelpersTestClock>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}