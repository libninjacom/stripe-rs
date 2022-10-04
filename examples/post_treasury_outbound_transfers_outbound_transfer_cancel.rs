use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let outbound_transfer = "your outbound transfer";
    let response = client
        .post_treasury_outbound_transfers_outbound_transfer_cancel(outbound_transfer)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
