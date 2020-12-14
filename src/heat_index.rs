// TODO: Finish implementing

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

pub fn compute_heat_index(celsius: f64, dewpoint: f64) -> f64 {
    let rh = compute_relative_humidity(celsius, dewpoint);

    if celsius < 27.0 || rh.is_err() || rh.unwrap() < 40.0 {
        return celsius;
    }
    let rh = rh.unwrap();

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
        // assert!(within_heat_index_error(
        //   compute_heat_index(31.0, 55.0),
        //   34.0
        // ));
        // assert!(within_heat_index_error(
        //   compute_heat_index(31.0, 85.0),
        //   43.0
        // ));
    }
}
