use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_reviews_review_approve`].

On request success, this will return a [`Review`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostReviewsReviewApproveRequest {
    pub review: String,
}
impl PostReviewsReviewApproveRequest {}
impl FluentRequest<'_, PostReviewsReviewApproveRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostReviewsReviewApproveRequest> {
    type Output = httpclient::InMemoryResult<Review>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/reviews/{review}/approve", review = self.params.review
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}