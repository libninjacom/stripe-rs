use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_plans_plan`].

On request success, this will return a [`Plan`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPlansPlanRequest {
    pub plan: String,
}
impl PostPlansPlanRequest {}
impl FluentRequest<'_, PostPlansPlanRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostPlansPlanRequest> {
    type Output = httpclient::InMemoryResult<Plan>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/plans/{plan}", plan = self.params.plan);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}