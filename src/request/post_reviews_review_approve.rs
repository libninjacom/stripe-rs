use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostReviewsReviewApproveRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub review: String,
}
impl<'a> PostReviewsReviewApproveRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Review> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/v1/reviews/{review}/approve", review = self.review));
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostReviewsReviewApproveRequest<'a> {
    type Output = httpclient::InMemoryResult<Review>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}