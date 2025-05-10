use axum::{
    routing::{get, post},
    Router,
    response::Json
};
use serde_json::Value;
mod routes;

use routes::{
    handle_create_name::create_name, handle_formatted_name::formatted_name, handle_name::say_name, handle_fetch_weather::handle_request
};
const BASE_URL: &str = "0.0.0.0:5000"; // base url for server

#[tokio::main]
async fn main() {
    // run our server with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(BASE_URL).await.unwrap();
    axum::serve(listener, server()).await.unwrap();
}

async fn weather_handler() -> Json<Value> {
    match handle_request().await {
        Ok(data) => Json(data),  // Return JSON response
        Err(_) => Json("failed to fetch data".into()),
    }
}

// Router
pub fn server() -> Router {
    return Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/say-name", get(|| say_name()))
        .route("/json-name", get(|| formatted_name()))
        .route("/create-name", post(create_name))
        .route("/weather", get(weather_handler));
}
