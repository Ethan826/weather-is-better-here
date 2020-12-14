use crate::convert_knots_to_mph;

pub fn compute_wind_chill(fahrenheit: f64, knots: usize) -> Option<f64> {
    let mph = convert_knots_to_mph(knots);
    if mph >= 3.0 && fahrenheit < 50.0 {
        Some(
            35.74 + (0.6215 * fahrenheit) - (35.75 * mph.powf(0.16))
                + (0.4275 * fahrenheit * mph.powf(0.16)),
        )
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compute_wind_chill() {
        assert_eq!(compute_wind_chill(20.0, 5).unwrap().round(), 12.0);

        assert!(compute_wind_chill(50.0, 5).is_none());
        assert!(compute_wind_chill(49.9, 5).is_some());

        assert!(compute_wind_chill(40.0, 2).is_none());
        assert!(compute_wind_chill(40.0, 3).is_some());
    }
}
