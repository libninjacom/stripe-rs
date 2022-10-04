use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let payment_method = "your payment method";
    let response = client
        .post_payment_methods_payment_method_attach(payment_method)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
