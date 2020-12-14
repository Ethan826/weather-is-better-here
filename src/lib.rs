pub mod wind_chill;
pub mod heat_index;

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

pub fn convert_knots_to_mph(knots: usize) -> f64 {
    ((1.15078 * knots as f64) * 100.00).round() / 100.0
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
}
