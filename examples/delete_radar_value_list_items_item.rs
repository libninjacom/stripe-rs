use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let item = "your item";
    let response = client.delete_radar_value_list_items_item(item).send().await.unwrap();
    println!("{:#?}", response);
}
