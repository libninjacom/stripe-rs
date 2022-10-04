use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let intent = "your intent";
    let response = client
        .post_setup_intents_intent_verify_microdeposits(intent)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
