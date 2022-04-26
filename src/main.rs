use std::fs;
use reqwest;

#[tokio::main]
async fn main() {
    let body = reqwest::get("https://www.rust-lang.org")
    .await
    .unwrap()
    .text()
    .await;

    //println!("body = {:?}", body);

    fs::write("rust.html", body.unwrap()).unwrap();

}