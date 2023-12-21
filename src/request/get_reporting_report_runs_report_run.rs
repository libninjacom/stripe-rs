use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_reporting_report_runs_report_run`].

On request success, this will return a [`ReportingReportRun`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReportingReportRunsReportRunRequest {
    pub expand: Option<Vec<String>>,
    pub report_run: String,
}
impl GetReportingReportRunsReportRunRequest {}
impl FluentRequest<'_, GetReportingReportRunsReportRunRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetReportingReportRunsReportRunRequest> {
    type Output = httpclient::InMemoryResult<ReportingReportRun>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/reporting/report_runs/{report_run}", report_run = self.params
                .report_run
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}