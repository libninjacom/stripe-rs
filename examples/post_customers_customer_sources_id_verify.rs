use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let customer = "your customer";
    let id = "your id";
    let response = client
        .post_customers_customer_sources_id_verify(customer, id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
