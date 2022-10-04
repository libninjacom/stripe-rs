use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_billing_portal_configurations()
        .active(true)
        .ending_before("your ending before")
        .expand(&["your expand"])
        .is_default(true)
        .limit(1)
        .starting_after("your starting after")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
