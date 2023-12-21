use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_subscription_schedules_schedule`].

On request success, this will return a [`SubscriptionSchedule`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSubscriptionSchedulesScheduleRequest {
    pub schedule: String,
}
impl PostSubscriptionSchedulesScheduleRequest {}
impl FluentRequest<'_, PostSubscriptionSchedulesScheduleRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostSubscriptionSchedulesScheduleRequest> {
    type Output = httpclient::InMemoryResult<SubscriptionSchedule>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/subscription_schedules/{schedule}", schedule = self.params.schedule
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}