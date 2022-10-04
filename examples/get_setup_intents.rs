use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_setup_intents()
        .attach_to_self(true)
        .created(::serde_json::json!({}))
        .customer("your customer")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .payment_method("your payment method")
        .starting_after("your starting after")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
