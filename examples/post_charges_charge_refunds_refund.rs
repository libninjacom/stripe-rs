use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let charge = "your charge";
    let refund = "your refund";
    let response = client
        .post_charges_charge_refunds_refund(charge, refund)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
