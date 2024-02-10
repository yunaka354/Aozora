use dotenv::dotenv;
use std::env;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bluesky_handle = env::var("BLUESKY_HANDLE").expect("BLUESKY_HANDLE must be set");
    let bluesky_password = env::var("BLUESKY_PASSWORD").expect("BLUESKY_PASSWORD must be set");
    let bsky_url = "https://bsky.social/xrpc/com.atproto.server.createSession";
    let mut map = HashMap::new();
    map.insert("identifier", bluesky_handle);
    map.insert("password", bluesky_password);

    let client = reqwest::Client::new();
    let res = client.post(bsky_url)
        .json(&map)
        .send()
        .await;
    println!("{:?}", res);
    println!("{:?}", res.unwrap().text().await.unwrap());
}