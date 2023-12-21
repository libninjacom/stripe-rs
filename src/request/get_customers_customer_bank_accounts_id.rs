use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_customers_customer_bank_accounts_id`].

On request success, this will return a [`BankAccount`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomersCustomerBankAccountsIdRequest {
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl GetCustomersCustomerBankAccountsIdRequest {}
impl FluentRequest<'_, GetCustomersCustomerBankAccountsIdRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetCustomersCustomerBankAccountsIdRequest> {
    type Output = httpclient::InMemoryResult<BankAccount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/customers/{customer}/bank_accounts/{id}", customer = self.params
                .customer, id = self.params.id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}