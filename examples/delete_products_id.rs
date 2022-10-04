use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let id = "your id";
    let response = client.delete_products_id(id).send().await.unwrap();
    println!("{:#?}", response);
}
