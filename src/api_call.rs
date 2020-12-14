use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

const URL: &str = r#"https://aviationweather.gov/adds/dataserver_current/httpparam?dataSource=metars&requestType=retrieve&format=xml&stationString=KMDW%20KRDU&hoursBeforeNow=2"#;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metar {
    pub dewpoint_c: f32,
    pub observation_time: String,
    pub station_id: String,
    pub temp_c: f32,
    pub wind_speed_kt: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "METAR")]
    pub metar: Vec<Metar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub data: Data,
}

pub async fn make_network_call() -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(URL)
        .header(reqwest::header::USER_AGENT, "experimenting")
        .send()
        .await?
        .text()
        .await?;

    Ok(from_str::<Response>(&response)?)
}
