use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let fee = "your fee";
    let id = "your id";
    let response = client
        .post_application_fees_fee_refunds_id(fee, id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
