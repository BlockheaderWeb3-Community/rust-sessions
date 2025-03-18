use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
struct CityWeather {
    country: String,
    region: String,
    condition: String,
    last_updated: String,
    humidity: String,
    wind_dr: String,
    wind_chill_c: f64,
    wind_chill_f: f64,
    temp_c: f64,
    temp_f: f64,
    heat_index_f: f64,
    heat_index_c: f64,
    wind_degree: u64,
    status: u32,
}

pub async fn get_weather() -> impl IntoResponse {
    let api = String::from(
        "https://api.weatherapi.com/v1/current.json?key=bb2e5608bc574becad611405240207&q=",
    );
    match response_data(api, "london").await {
        Ok(data) => {
            let weather = format_data(data);
            (StatusCode::OK, Json(weather))
        }
        Err(_) => {
            let error_response = CityWeather {
                country: "".to_string(),
                region: "".to_string(),
                condition: "".to_string(),
                last_updated: "".to_string(),
                humidity: "".to_string(),
                wind_dr: "".to_string(),
                wind_chill_c: 0.0,
                wind_chill_f: 0.0,
                temp_c: 0.0,
                temp_f: 0.0,
                heat_index_f: 0.0,
                heat_index_c: 0.0,
                wind_degree: 0,
                status: 404,
            };
            (StatusCode::BAD_REQUEST, Json(error_response))
        }
    }
}

fn format_string(data: String) -> String {
    data.trim_matches('"').replace("/", " ").to_string()
}

async fn response_data(api: String, city: &str) -> Result<Json<Value>, Box<dyn std::error::Error>> {
    let url = format!("{}{}", api, city);
    let data = reqwest::get(&url).await?;

    if data.status() != 200 {
        return Err("city detail not found".into());
    }
    let response: Value = data.json().await?;
    Ok(Json(response))
}

fn format_data(response: Json<Value>) -> CityWeather {
    CityWeather {
        condition: format_string(response["current"]["condition"]["text"].to_string()),
        heat_index_c: response["current"]["heatindex_c"].as_f64().unwrap_or(0.0),
        heat_index_f: response["current"]["heatindex_f"].as_f64().unwrap_or(0.0),
        humidity: response["current"]["humidity"].to_string(),
        last_updated: format_string(response["current"]["last_updated"].to_string()),
        country: format_string(response["location"]["country"].to_string()),
        region: format_string(response["location"]["tz_id"].to_string()),
        temp_c: response["current"]["temp_c"].as_f64().unwrap_or(0.0),
        temp_f: response["current"]["temp_f"].as_f64().unwrap_or(0.0),
        wind_chill_c: response["current"]["windchill_c"].as_f64().unwrap_or(0.0),
        wind_chill_f: response["current"]["windchill_f"].as_f64().unwrap_or(0.0),
        wind_degree: response["current"]["wind_degree"].as_u64().unwrap_or(0),
        wind_dr: format_string(response["current"]["wind_dir"].to_string()),
        status: 200,
    }
}
