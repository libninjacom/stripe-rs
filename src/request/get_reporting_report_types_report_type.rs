use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetReportingReportTypesReportTypeRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub report_type: String,
}
impl<'a> GetReportingReportTypesReportTypeRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ReportingReportType> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v1/reporting/report_types/{report_type}", report_type = self
                    .report_type
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetReportingReportTypesReportTypeRequest<'a> {
    type Output = httpclient::InMemoryResult<ReportingReportType>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}