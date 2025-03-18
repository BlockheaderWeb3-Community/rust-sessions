use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reqwest;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://api.weatherapi.com/v1";
const API_KEY: &str = "e51ffecacdb14c65b0b82259240207";

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

#[derive(Debug, Deserialize)]
pub struct WeatherParams {
    q: Option<String>,
}

// Updated to accept query parameters
pub async fn get_weather(params: Query<WeatherParams>) -> impl IntoResponse {
    let client = reqwest::Client::new();
    
    let location = params.q.clone().unwrap_or_else(|| "London".to_string());
    
    let url = format!(
        "{}/current.json?key={}&q={}&aqi=no",
        BASE_URL, API_KEY, location
    );
    
    match client
        .get(url)
        .send()
        .await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text().await {
                        Ok(text) => {
                            let weather_result: Result<Weather, _> = serde_json::from_str(&text);
                            match weather_result {
                                Ok(parsed) => {
                                    let weather_response = WeatherResponse {
                                        location: parsed.location.name,
                                        time: parsed.location.localtime,
                                        temperature: parsed.current.temp_c,
                                    };
                                    
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
                } else {
                    (
                        StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::BAD_REQUEST),
                        format!("API Error: {}", response.status())
                    ).into_response()
                }
            },
            Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Error fetching data: {}", e)).into_response()
            },
        }
}