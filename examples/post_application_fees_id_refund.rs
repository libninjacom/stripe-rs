use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let id = "your id";
    let response = client.post_application_fees_id_refund(id).send().await.unwrap();
    println!("{:#?}", response);
}
