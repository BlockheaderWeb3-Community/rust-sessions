use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::join;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub city: String,
    pub temperature: Option<f32>,
    pub conditions: Option<String>,
    pub error: Option<String>,
}

pub struct AppState {
    pub client: Client,
    pub api_key: String,
}

pub async fn get_weather(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let cities = vec!["London", "Tokyo", "New York"];

    // Create futures for each city weather request
    let london_future = fetch_city_weather(&state, cities[0]);
    let tokyo_future = fetch_city_weather(&state, cities[1]);
    let newyork_future = fetch_city_weather(&state, cities[2]);

    //Execute all futures concurrently
    let (london, tokyo, newyor) = join!(london_future, tokyo_future, newyork_future);

    // Combine the results into a single response
    let results = vec![london, tokyo, newyor];
    Json(results)
}

async fn fetch_city_weather(state: &Arc<AppState>, city: &str) -> WeatherResponse {
    let url = format!(
        "https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
        state.api_key, city
    );

    match state.client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(json) => {
                        let temp = json["current"]["temp_c"].as_f64().map(|t| t as f32);
                        let cond = json["current"]["condition"]["text"]
                            .as_str()
                            .map(String::from);

                        WeatherResponse {
                            city: city.to_string(),
                            temperature: temp,
                            conditions: cond,
                            error: None,
                        }
                    }
                    Err(e) => WeatherResponse {
                        city: city.to_string(),
                        temperature: None,
                        conditions: None,
                        error: Some(format!("Failed to parse JSON: {}", e)),
                    },
                }
            } else {
                WeatherResponse {
                    city: city.to_string(),
                    temperature: None,
                    conditions: None,
                    error: Some(format!("API error: {}", response.status())),
                }
            }
        }
        Err(e) => WeatherResponse {
            city: city.to_string(),
            temperature: None,
            conditions: None,
            error: Some(format!("Request failed: {}", e)),
        },
    }
}
