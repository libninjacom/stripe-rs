#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let calculation = "your calculation";
    let response = client
        .get_tax_calculations_calculation_line_items(calculation)
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .await
        .unwrap();
    println!("{:#?}", response);
}