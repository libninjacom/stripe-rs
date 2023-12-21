use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_subscription_items_subscription_item_usage_record_summaries`].

On request success, this will return a [`GetSubscriptionItemsSubscriptionItemUsageRecordSummariesResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRequest {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub subscription_item: String,
}
impl GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRequest {}
impl FluentRequest<'_, GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRequest> {
    pub fn ending_before(mut self, ending_before: &str) -> Self {
        self.params.ending_before = Some(ending_before.to_owned());
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRequest> {
    type Output = httpclient::InMemoryResult<
        GetSubscriptionItemsSubscriptionItemUsageRecordSummariesResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/subscription_items/{subscription_item}/usage_record_summaries",
                subscription_item = self.params.subscription_item
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}