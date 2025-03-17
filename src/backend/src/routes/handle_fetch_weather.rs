// use axum::extract::Request as OtherRequest;
// use std::thread::Builder;
// use tokio::runtime::Builder;
// use axum::body::Body;

// pub type Request<T = Body> = Request<T>;

// pub fn handle_request<T>(uri: T) -> Builder {
//    let request = Request::get(uri)
//    .body(())
//    .unwrap();

//    println!("this is the data from the request {}", request)
// }


use reqwest::Error;
use serde_json::Value;

// #[derive(Debug)]
// pub struct WeatherDetails {
//     name: String,
//     country: String,
//     region: String,
//     lat: String,
//     lon: String,
//     timezone_id: String,
//     localtime: String,
//     localtime_epoch: u32,
// }

pub async fn handle_request() -> Result<Value, Error> {
    let weather = reqwest::get("https://api.weatherstack.com/current?access_key=ee70f0af3c7ac89490f38705bbfb87c0&query=kaduna")
    .await?
    .json::<Value>()
    .await?;
 
    println!("data from api_______ {:#?}", weather.get("location"));
    if let Some(data) = weather.get("location") {
        println!("__________ {:#?}", data);
        Ok(data.clone())
    } else {
        Ok(Value::Null)
    }
    // let weatherDet = 
 }