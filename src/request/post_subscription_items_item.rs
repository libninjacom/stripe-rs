use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_subscription_items_item`].

On request success, this will return a [`SubscriptionItem`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSubscriptionItemsItemRequest {
    pub item: String,
}
impl PostSubscriptionItemsItemRequest {}
impl FluentRequest<'_, PostSubscriptionItemsItemRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostSubscriptionItemsItemRequest> {
    type Output = httpclient::InMemoryResult<SubscriptionItem>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/subscription_items/{item}", item = self.params.item);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}