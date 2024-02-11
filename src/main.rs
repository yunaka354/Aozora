mod model;
mod api;

#[tokio::main]
async fn main() {
    let api = api::API::new().await;
    let timeline = api.get_timeline().await;
    println!("{:?}", timeline);
}