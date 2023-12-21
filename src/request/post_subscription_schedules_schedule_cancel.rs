use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_subscription_schedules_schedule_cancel`].

On request success, this will return a [`SubscriptionSchedule`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSubscriptionSchedulesScheduleCancelRequest {
    pub schedule: String,
}
impl PostSubscriptionSchedulesScheduleCancelRequest {}
impl FluentRequest<'_, PostSubscriptionSchedulesScheduleCancelRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostSubscriptionSchedulesScheduleCancelRequest> {
    type Output = httpclient::InMemoryResult<SubscriptionSchedule>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/subscription_schedules/{schedule}/cancel", schedule = self.params
                .schedule
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}