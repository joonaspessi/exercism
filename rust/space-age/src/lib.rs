// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR: f64 = 31557600.00;

fn calculate_duration(d: &Duration, orbital_period: f64) -> f64 {
    d.duration_s as f64 / (EARTH_YEAR * orbital_period)
}

#[derive(Debug)]
pub struct Duration {
    duration_s: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { duration_s: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
    }
}

macro_rules! planet {
    ($n: ident, $p: expr) => {
        pub struct $n;
        impl Planet for $n {
            fn years_during(d: &Duration) -> f64 {
                calculate_duration(d, $p)
            }
        }
    };
}

//    - Mercury: orbital period 0.2408467 Earth years
//    - Venus: orbital period 0.61519726 Earth years
//    - Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
//    - Mars: orbital period 1.8808158 Earth years
//    - Jupiter: orbital period 11.862615 Earth years
//    - Saturn: orbital period 29.447498 Earth years
//    - Uranus: orbital period 84.016846 Earth years
//    - Neptune: orbital period 164.79132 Earth years

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.00);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
