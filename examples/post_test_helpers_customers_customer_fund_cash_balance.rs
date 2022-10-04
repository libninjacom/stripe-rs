use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let customer = "your customer";
    let response = client
        .post_test_helpers_customers_customer_fund_cash_balance(customer)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
