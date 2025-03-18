use axum::{
    routing::{get, post},
    Router,
};
mod routes;

use routes::{
    handle_create_name::create_name, handle_formatted_name::formatted_name, handle_name::say_name, handle_weather::get_weather,
};
const BASE_URL: &str = "0.0.0.0:5000"; // base url for server

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind(BASE_URL).await.unwrap();
    axum::serve(listener, server()).await.unwrap();
}

// Router
pub fn server() -> Router {
    return Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/say-name", get(|| say_name()))
        .route("/json-name", get(|| formatted_name()))
        .route("/create-name", post(create_name))
        .route("/get-weather", get(get_weather));
}
