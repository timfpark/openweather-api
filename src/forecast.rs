use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherForecast {
    pub dt: i64,
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: i32,
    pub humidity: i32,
    pub dew_point: f64,
    pub uvi: f64,
    pub clouds: i32,
    pub visibility: i32,
    pub wind_speed: f64,
    pub wind_deg: i32,
    pub wind_gust: f64,
}
