use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetTerminalReadersRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub device_type: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub location: Option<String>,
    pub serial_number: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl<'a> GetTerminalReadersRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TerminalReaderRetrieveReader> {
        let mut r = self.http_client.client.get("/v1/terminal/readers");
        if let Some(ref unwrapped) = self.device_type {
            r = r.query("device_type", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.location {
            r = r.query("location", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.serial_number {
            r = r.query("serial_number", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.query("status", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn device_type(mut self, device_type: &str) -> Self {
        self.device_type = Some(device_type.to_owned());
        self
    }
    pub fn ending_before(mut self, ending_before: &str) -> Self {
        self.ending_before = Some(ending_before.to_owned());
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn location(mut self, location: &str) -> Self {
        self.location = Some(location.to_owned());
        self
    }
    pub fn serial_number(mut self, serial_number: &str) -> Self {
        self.serial_number = Some(serial_number.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetTerminalReadersRequest<'a> {
    type Output = httpclient::InMemoryResult<TerminalReaderRetrieveReader>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}