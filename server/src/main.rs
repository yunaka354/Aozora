mod api;
mod model;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::routing::get;
use axum::Extension;
use axum::Router;
use http::Method;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // Create an instance of the API
    let api = api::API::new().await;

    // CORS setting
    // CAUTION: This is not secure. You should not use this in production.
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    // Create an instance of the web server
    let app = Router::new()
        .route("/", get(root))
        .route("/timeline", get(timeline))
        .layer(cors)
        .layer(Extension(api));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    println!("server is hosted at {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn timeline(Extension(api): Extension<api::API>) -> impl IntoResponse {
    let timeline = api.get_timeline().await;
    Json(timeline)
}
