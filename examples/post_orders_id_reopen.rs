use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let id = "your id";
    let response = client.post_orders_id_reopen(id).send().await.unwrap();
    println!("{:#?}", response);
}
