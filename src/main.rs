use std::error::Error;
use weather::{group_by, make_network_call, Metar, TempData};

const CHICAGO: &str = "KMDW";
const RALEIGH: &str = "KRDU";

fn extract_most_recent_data(
    grouped: &std::collections::HashMap<&String, std::vec::Vec<&Metar>>,
    station: &str,
    error_message: &str,
) -> Result<TempData, Box<dyn std::error::Error>> {
    Ok(TempData::new(
        grouped
            .get(&station.to_string())
            .and_then(|arr| {
                arr.iter()
                    .max_by(|metar1, metar2| metar1.observation_time.cmp(&metar2.observation_time))
            })
            .ok_or(error_message)?,
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = make_network_call().await?.data;
    let grouped = group_by(&data.metar, |el| &el.station_id);
    let mdw = extract_most_recent_data(
        &grouped,
        CHICAGO,
        "Chicago weather data not in expected format",
    )?;
    let rdu = extract_most_recent_data(
        &grouped,
        RALEIGH,
        "Raleigh weather data not in expected format",
    )?;

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
