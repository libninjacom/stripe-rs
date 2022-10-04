use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let intent = "your intent";
    let response = client.post_payment_intents_intent(intent).send().await.unwrap();
    println!("{:#?}", response);
}
