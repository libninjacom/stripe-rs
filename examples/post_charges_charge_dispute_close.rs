use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let charge = "your charge";
    let response = client
        .post_charges_charge_dispute_close(charge)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
