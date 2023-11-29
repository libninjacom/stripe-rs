#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let order = "your order";
    let response = client.post_climate_orders_order_cancel(order).await.unwrap();
    println!("{:#?}", response);
}