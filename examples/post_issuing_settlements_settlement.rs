use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let settlement = "your settlement";
    let response = client
        .post_issuing_settlements_settlement(settlement)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
