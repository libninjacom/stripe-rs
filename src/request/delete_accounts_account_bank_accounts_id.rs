use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_accounts_account_bank_accounts_id`].

On request success, this will return a [`DeletedExternalAccount`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAccountsAccountBankAccountsIdRequest {
    pub account: String,
    pub id: String,
}
impl DeleteAccountsAccountBankAccountsIdRequest {}
impl FluentRequest<'_, DeleteAccountsAccountBankAccountsIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteAccountsAccountBankAccountsIdRequest> {
    type Output = httpclient::InMemoryResult<DeletedExternalAccount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/accounts/{account}/bank_accounts/{id}", account = self.params
                .account, id = self.params.id
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}