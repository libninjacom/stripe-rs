use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let scheduled_query_run = "your scheduled query run";
    let response = client
        .get_sigma_scheduled_query_runs_scheduled_query_run(scheduled_query_run)
        .expand(&["your expand"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
