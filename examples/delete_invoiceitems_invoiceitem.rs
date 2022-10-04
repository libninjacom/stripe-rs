use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let invoiceitem = "your invoiceitem";
    let response = client
        .delete_invoiceitems_invoiceitem(invoiceitem)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
