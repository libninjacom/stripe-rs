use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_reporting_report_runs`].

On request success, this will return a [`ReportingReportRun`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostReportingReportRunsRequest {}
impl PostReportingReportRunsRequest {}
impl FluentRequest<'_, PostReportingReportRunsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostReportingReportRunsRequest> {
    type Output = httpclient::InMemoryResult<ReportingReportRun>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/reporting/report_runs";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}