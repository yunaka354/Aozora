mod model;
mod api;
use axum::routing::get;
use axum::Router;
use axum::Extension;
use axum::response::IntoResponse;
use axum::response::Json;

#[tokio::main]
async fn main() {
    let api = api::API::new().await;
    let app = Router::new()
        .route("/", get(root))
        .route("/timeline", get(timeline))
        .layer(Extension(api));
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn timeline(Extension(api): Extension<api::API>) -> impl IntoResponse {
    let timeline = api.get_timeline().await;
    Json(timeline)  
}