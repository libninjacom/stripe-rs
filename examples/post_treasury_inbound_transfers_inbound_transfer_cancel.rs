use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let inbound_transfer = "your inbound transfer";
    let response = client
        .post_treasury_inbound_transfers_inbound_transfer_cancel(inbound_transfer)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
