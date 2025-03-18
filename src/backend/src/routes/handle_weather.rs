use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reqwest;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://api.weatherapi.com/v1";

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    name: String,
    localtime: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Current {
    temp_c: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Weather {
    location: Location,
    current: Current,
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherResponse {
    location: String,
    time: String,
    temperature: f32,
}

// Return proper Axum response types
pub async fn get_weather() -> impl IntoResponse {
    let client = reqwest::Client::new();
    
    match client
        .get("http://api.weatherapi.com/v1/current.json?key=e51ffecacdb14c65b0b82259240207&q=London&aqi=no")
        .send()
        .await {
            Ok(response) => {
                match response.text().await {
                    Ok(text) => {
                        let weather_result: Result<Weather, _> = serde_json::from_str(&text);
                        match weather_result {
                            Ok(parsed) => {
                                // Create a simplified response structure
                                let weather_response = WeatherResponse {
                                    location: parsed.location.name,
                                    time: parsed.location.localtime,
                                    temperature: parsed.current.temp_c,
                                };
                                
                                // Return as JSON with 200 status
                                (StatusCode::OK, Json(weather_response)).into_response()
                            },
                            Err(e) => {
                                (StatusCode::INTERNAL_SERVER_ERROR, format!("Error parsing JSON: {}", e)).into_response()
                            },
                        }
                    },
                    Err(e) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, format!("Error reading response: {}", e)).into_response()
                    },
                }
            },
            Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Error fetching data: {}", e)).into_response()
            },
        }
}