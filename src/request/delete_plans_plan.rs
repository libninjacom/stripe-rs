use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_plans_plan`].

On request success, this will return a [`DeletedPlan`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePlansPlanRequest {
    pub plan: String,
}
impl DeletePlansPlanRequest {}
impl FluentRequest<'_, DeletePlansPlanRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeletePlansPlanRequest> {
    type Output = httpclient::InMemoryResult<DeletedPlan>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/plans/{plan}", plan = self.params.plan);
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}