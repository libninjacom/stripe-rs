use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let test_clock = "your test clock";
    let response = client
        .post_test_helpers_test_clocks_test_clock_advance(test_clock)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
