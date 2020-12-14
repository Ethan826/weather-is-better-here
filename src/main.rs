use reqwest;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use std::collections::HashMap;
use std::error::Error;

use weather::celsius_to_fahrenheit;
use weather::wind_chill::compute_wind_chill;

#[derive(Debug)]
struct TempData {
    temp_c: f64,
    temp_f: f64,
    wind_chill_f: f64,
    heat_index_f: f64,
}

impl TempData {
    pub fn new(metar: &Metar) -> Self {
        let temp_c = metar.temp_c as f64;
        let temp_f = celsius_to_fahrenheit(temp_c);
        let rh = compute_relative_humidity(temp_c, metar.dewpoint_c as f64);
        let heat_index_f = rh
            .and_then(|rh| Ok(compute_heat_index(temp_c, rh)))
            .unwrap_or(temp_f);

        TempData {
            temp_c,
            temp_f,
            heat_index_f,
            wind_chill_f: compute_wind_chill(temp_f, metar.wind_speed_kt).unwrap_or(temp_f),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metar {
    dewpoint_c: f32,
    observation_time: String,
    station_id: String,
    temp_c: f32,
    wind_speed_kt: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "METAR")]
    metar: Vec<Metar>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub data: Data,
}

fn compute_relative_humidity(celsius: f64, dewpoint: f64) -> Result<f64, ()> {
    if dewpoint > celsius {
        Err(())
    } else {
        Ok((10000.0 * ((17.625 * dewpoint) / (243.04 + dewpoint)).exp()
            / ((17.625 * celsius) / (243.04 + celsius)).exp())
        .round()
            / 100.0)
    }
}

fn compute_heat_index(celsius: f64, rh: f64) -> f64 {
    if celsius < 27.0 || rh < 40.0 {
        return celsius;
    }

    let c1 = -8.78469475556;
    let c2 = 1.61139411;
    let c3 = 2.33854883889;
    let c4 = -0.14611605;
    let c5 = -0.012308094;
    let c6 = -0.0164248277778;
    let c7 = 0.002211732;
    let c8 = 0.00072546;
    let c9 = -0.000003582;

    c1 + c2 * celsius
        + c3 * rh
        + c4 * celsius * rh
        + c5 * celsius * celsius
        + c6 * rh * rh
        + c7 * celsius * celsius * rh
        + c8 * celsius * rh * rh
        + c9 * celsius * celsius * rh * rh
}

fn group_by<'a, T, U>(to_group: &'a Vec<T>, group_fn: fn(&'a T) -> U) -> HashMap<U, Vec<&'a T>>
where
    U: std::cmp::Eq + std::hash::Hash,
{
    to_group.iter().fold(HashMap::new(), |mut acc, el| {
        let to_insert = group_fn(el);
        let entry = acc.entry(to_insert).or_insert(Vec::new());
        entry.push(el);
        acc
    })
}

const CHICAGO: &'static str = "KMDW";
const RALEIGH: &'static str = "KRDU";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(r#"https://aviationweather.gov/adds/dataserver_current/httpparam?dataSource=metars&requestType=retrieve&format=xml&stationString=KMDW%20KRDU&hoursBeforeNow=1"#)
        .header(reqwest::header::USER_AGENT, "experimenting")
        .send()
        .await?
        .text()
        .await?;

    let deserialized: Response = from_str(&response)?;
    let grouped = group_by(&deserialized.data.metar, |el| &el.station_id);
    let mdw = TempData::new(
        grouped
            .get(&CHICAGO.to_string())
            .and_then(|arr| {
                arr.iter()
                    .max_by(|metar1, metar2| metar1.observation_time.cmp(&metar2.observation_time))
            })
            .ok_or("Chicago weather data not in expected format")?,
    );
    let rdu = TempData::new(
        grouped
            .get(&RALEIGH.to_string())
            .and_then(|arr| {
                arr.iter()
                    .max_by(|metar1, metar2| metar1.observation_time.cmp(&metar2.observation_time))
            })
            .ok_or("Raleigh weather data not in expected format")?,
    );

    let amount_colder_chicago = rdu.temp_f - mdw.temp_f;
    let amount_wind_chill_colder_chicago = rdu.wind_chill_f - mdw.wind_chill_f;

    println!(
        "Right now it's {:.1}ºF {} in Oak Park",
        amount_colder_chicago.abs(),
        if amount_colder_chicago > 0.0 {
            "colder"
        } else {
            "warmer"
        }
    );
    println!(
        "Right now the wind chill is {:.1}ºF {} in Oak Park",
        amount_wind_chill_colder_chicago.abs(),
        if amount_wind_chill_colder_chicago > 0.0 {
            "colder"
        } else {
            "warmer"
        }
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compute_relative_humidity() {
        assert_eq!(compute_relative_humidity(22.0, 18.0).unwrap(), 78.06);
    }

    fn within_heat_index_error(actual: f64, expected: f64) -> bool {
        (actual - expected).abs() <= 0.7
    }

    #[test]
    fn test_compute_heat_index() {
        assert!(within_heat_index_error(
            compute_heat_index(27.0, 40.0),
            27.0
        ));
        assert!(within_heat_index_error(
            compute_heat_index(31.0, 55.0),
            34.0
        ));
        assert!(within_heat_index_error(
            compute_heat_index(31.0, 85.0),
            43.0
        ));
    }

    #[test]
    fn test_group_by() {
        let mut expected = HashMap::new();
        expected.insert(false, vec![&3, &7, &5, &19, &33]);
        expected.insert(true, vec![&2, &14]);

        assert_eq!(
            group_by(&vec![3, 7, 2, 5, 19, 33, 14], |el| el % 2 == 0),
            expected
        );
    }
}
