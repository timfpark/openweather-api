use serde::{Deserialize, Serialize};

use crate::{CurrentWeather, Units, WeatherForecast};

#[derive(Debug, Deserialize, Serialize)]
pub struct OneCallResponse {
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub timezone_offset: i32,
    pub current: CurrentWeather,
    pub hourly: Option<Vec<WeatherForecast>>,
}

pub async fn onecall(
    api_key: &str,
    latitude: f64,
    longitude: f64,
    exclude: &[&str],
    units: Option<Units>,
) -> anyhow::Result<OneCallResponse> {
    let units = match units {
        Some(Units::Standard) => "standard",
        Some(Units::Metric) => "metric",
        Some(Units::Imperial) => "imperial",
        None => "standard",
    };

    let url = format!("https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&exclude={}&units={}&appid={}", latitude, longitude, exclude.join(","), units, api_key);
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;

    let one_call_response: OneCallResponse = serde_json::from_str(&body)?;

    Ok(one_call_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_get_delete() -> anyhow::Result<()> {
        dotenvy::from_filename(".env.test").ok();

        let api_key = std::env::var("OPENWEATHER_API_KEY").unwrap();

        let one_call_response = onecall(
            &api_key,
            37.8267,
            -122.4233,
            &["minutely", "hourly", "daily"],
            Some(Units::Metric),
        )
        .await?;

        assert_eq!(one_call_response.lat, 37.8267);
        assert_eq!(one_call_response.lon, -122.4233);

        println!("{:#?}", one_call_response);
        Ok(())
    }
}
