#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let early_fraud_warning = "your early fraud warning";
    let response = client
        .get_radar_early_fraud_warnings_early_fraud_warning(early_fraud_warning)
        .expand(&["your expand"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
