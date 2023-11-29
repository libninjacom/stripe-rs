use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIssuingCardholdersRequest {
    pub created: Option<serde_json::Value>,
    pub email: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub phone_number: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub type_: Option<String>,
}
impl GetIssuingCardholdersRequest {}
impl FluentRequest<'_, GetIssuingCardholdersRequest> {
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.params.created = Some(created);
        self
    }
    pub fn email(mut self, email: &str) -> Self {
        self.params.email = Some(email.to_owned());
        self
    }
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
    pub fn phone_number(mut self, phone_number: &str) -> Self {
        self.params.phone_number = Some(phone_number.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.params.status = Some(status.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.params.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetIssuingCardholdersRequest> {
    type Output = httpclient::InMemoryResult<IssuingCardholderList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/v1/issuing/cardholders";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}