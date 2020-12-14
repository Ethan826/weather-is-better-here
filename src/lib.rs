use std::collections::HashMap;

pub(in crate) mod api_call;
// pub(in crate) mod heat_index;
pub(in crate) mod wind_chill;

pub use api_call::{make_network_call, Metar};

use crate::wind_chill::compute_wind_chill;

pub(in crate) fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

pub(in crate) fn convert_knots_to_mph(knots: usize) -> f64 {
    ((1.15078 * knots as f64) * 100.00).round() / 100.0
}

pub fn group_by<'a, T, U>(to_group: &'a [T], group_fn: fn(&'a T) -> U) -> HashMap<U, Vec<&'a T>>
where
    U: std::cmp::Eq + std::hash::Hash,
{
    to_group.iter().fold(HashMap::new(), |mut acc, el| {
        let to_insert = group_fn(el);
        let entry = acc.entry(to_insert).or_insert_with(Vec::new);
        entry.push(el);
        acc
    })
}

#[derive(Debug)]
pub struct TempData {
    pub temp_c: f64,
    pub temp_f: f64,
    pub wind_chill_f: f64,
    // TODO: Finish implementing and add.
    // heat_index_f: f64,
}

impl TempData {
    pub fn new(metar: &Metar) -> Self {
        let temp_c = metar.temp_c as f64;
        let temp_f = celsius_to_fahrenheit(temp_c);

        TempData {
            temp_c,
            temp_f,
            wind_chill_f: compute_wind_chill(temp_f, metar.wind_speed_kt).unwrap_or(temp_f),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn test_convert_knots_to_mph() {
        assert_eq!(convert_knots_to_mph(25), 28.77);
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
