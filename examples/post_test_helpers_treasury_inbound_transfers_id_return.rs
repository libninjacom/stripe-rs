#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let id = "your id";
    let response = client
        .post_test_helpers_treasury_inbound_transfers_id_return(id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
