pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration((s as f64) / 31557600_f64)
    }
}

pub trait Planet {
    fn period() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::period()
    }
}

macro_rules! planetizer {
    ($p:ident, $d:expr) => {
        pub struct $p;
        impl Planet for $p {
            fn period() -> f64 {
                $d
            }
        }
    };
}

planetizer!(Earth, 1.0);
planetizer!(Mercury, 0.2408467);
planetizer!(Venus, 0.61519726);
planetizer!(Mars, 1.8808158);
planetizer!(Jupiter, 11.862615);
planetizer!(Saturn, 29.447498);
planetizer!(Uranus, 84.016846);
planetizer!(Neptune, 164.79132);
