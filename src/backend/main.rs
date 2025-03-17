use axum::{
    routing::{get, post},
    Router,
};
mod routes;

use dotenv::dotenv;
use reqwest::Client;
use routes::{
    handle_create_name::create_name,
    handle_formatted_name::formatted_name,
    handle_name::say_name,
    weather::{get_weather, AppState},
};
use std::env;
use std::sync::Arc;

const BASE_URL: &str = "0.0.0.0:5000"; // base url for server

#[tokio::main]
async fn main() {
    dotenv().ok();

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BASE_URL.to_string());
    let weather_api_key = env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY must be set");

    println!("Starting server on {}", base_url);
    println!("Weather API key: {}", weather_api_key);

    let state = Arc::new(AppState {
        client: Client::new(),
        api_key: weather_api_key,
    });

    let app = server(state);

    // run our server with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(base_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Router
pub fn server(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/say-name", get(|| say_name()))
        .route("/json-name", get(|| formatted_name()))
        .route("/create-name", post(create_name))
        .route("/weather", get(get_weather))
        .with_state(state)
}
