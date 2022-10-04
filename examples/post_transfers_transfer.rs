use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let transfer = "your transfer";
    let response = client.post_transfers_transfer(transfer).send().await.unwrap();
    println!("{:#?}", response);
}
