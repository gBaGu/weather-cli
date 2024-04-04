#[allow(dead_code)]
mod api_data;
mod error;

use std::env;
use std::sync::Arc;
use tokio::task::JoinSet;

use api_data::{ForecastData, GeoData};
use error::WeatherError;

const OPEN_WEATHER_MAP_GEO: &str = "http://api.openweathermap.org/geo/1.0/direct";
const OPEN_WEATHER_MAP_WEATHER: &str = "https://api.openweathermap.org/data/2.5/weather";

async fn get_forecast(city: &str, api_key: &str) -> Result<Vec<ForecastData>, WeatherError> {
    let client = Arc::new(reqwest::Client::new());

    // get city location
    let query = serde_json::json!({ "q": city, "appid": api_key });
    let response = client
        .get(OPEN_WEATHER_MAP_GEO)
        .query(&query)
        .send()
        .await?;
    let cities: Vec<GeoData> = response.json().await?;

    // get weather for every city
    let mut tasks = JoinSet::new();
    for city in cities {
        println!("fetching weather for {},{}", city.name, city.country);
        let client_cloned = Arc::clone(&client);
        let query = serde_json::json!({ "lat": city.lat, "lon": city.lon, "appid": api_key });
        tasks.spawn(async move {
            let response = client_cloned
                .get(OPEN_WEATHER_MAP_WEATHER)
                .query(&query)
                .send()
                .await?;
            response.json().await
        });
    }

    let mut forecasts = Vec::new();
    while let Some(res) = tasks.join_next().await {
        let out = res?;
        forecasts.push(out?);
    }
    Ok(forecasts)
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 1 {
        println!("usage: weather-cli <CITY>");
        return;
    }

    let api_key = match env::var("API_KEY") {
        Ok(val) => val,
        Err(_) => {
            println!("API_KEY environment variable must be set to openweathermap.org api key");
            return;
        }
    };
    let res = get_forecast(&args[0], &api_key).await;
    match res {
        Ok(weather_data) => {
            println!("temp(K)|feelslike(K)|t min(K)|t max(K)|pressure|humidity|wind speed");
            for entry in weather_data {
                println!("{}", entry);
            }
        },
        Err(err) => println!("failed to fetch weather: {}", err),
    }
}
