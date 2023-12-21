use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_accounts_account_capabilities`].

On request success, this will return a [`GetAccountsAccountCapabilitiesResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountsAccountCapabilitiesRequest {
    pub account: String,
    pub expand: Option<Vec<String>>,
}
impl GetAccountsAccountCapabilitiesRequest {}
impl FluentRequest<'_, GetAccountsAccountCapabilitiesRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetAccountsAccountCapabilitiesRequest> {
    type Output = httpclient::InMemoryResult<GetAccountsAccountCapabilitiesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/accounts/{account}/capabilities", account = self.params.account
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}