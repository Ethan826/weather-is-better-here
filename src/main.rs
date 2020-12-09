use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome {
    properties: Properties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    temperature: Temperature,
    #[serde(rename = "apparentTemperature")]
    apparent_temperature: Temperature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    uom: String,
    values: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    #[serde(rename = "validTime")]
    valid_time: String,
    value: f64,
}

const CHAPEL_HILL: &'static str = "https://api.weather.gov/gridpoints/RAH/56,61";
const OAK_PARK: &'static str = "https://api.weather.gov/gridpoints/LOT/69,71";

async fn get_temperature(client: &reqwest::Client, uri: &str) -> Result<f64, Box<dyn Error>> {
    Ok(client
        .get(uri)
        .header(reqwest::header::USER_AGENT, "experimenting")
        .send()
        .await?
        .json::<Welcome>()
        .await?
        .properties
        .temperature
        .values[0]
        .value)
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let chapel_hill_temp = convert_to_fahrenheit(get_temperature(&client, &CHAPEL_HILL).await?);
    let oak_park_temp = convert_to_fahrenheit(get_temperature(&client, &OAK_PARK).await?);
    println!(
        "It is {}º warmer in Chapel Hill",
        chapel_hill_temp - oak_park_temp
    );
    Ok(())
}
