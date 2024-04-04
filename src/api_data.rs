use std::fmt;
use serde::Deserialize;

/// Geo related structs

#[derive(Debug, Deserialize)]
pub struct GeoData {
    pub name: String,
    pub lat: f32,
    pub lon: f32,
    pub country: String,
    pub state: Option<String>,
}

/// Weather related structs

#[derive(Debug, Deserialize)]
struct MainData {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: u32,
    humidity: u32,
    sea_level: u32,
    grnd_level: u32,
}

#[derive(Debug, Deserialize)]
struct WindData {
    speed: f32,
    deg: u32,
    gust: f32,
}

#[derive(Debug, Deserialize)]
struct CloudsData {
    all: u32,
}

#[derive(Debug, Deserialize)]
pub struct ForecastData {
    main: MainData,
    visibility: u32,
    wind: WindData,
    clouds: CloudsData,
}

impl fmt::Display for ForecastData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{: >8}|", self.main.temp)?;
        write!(f, "{: >8}|", self.main.feels_like)?;
        write!(f, "{: >8}|", self.main.temp_min)?;
        write!(f, "{: >8}|", self.main.temp_max)?;
        write!(f, "{: >8}|", self.main.pressure)?;
        write!(f, "{: >8}|", self.main.humidity)?;
        write!(f, "{: >8}|", self.wind.speed)
    }
}