mod model;
mod api;

#[tokio::main]
async fn main() {
    let api = api::API::new().await;
}