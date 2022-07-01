use crate::models::driver::Driver;
use crate::models::type_aliases::Time;
use float_cmp::approx_eq;

pub trait ChampionshipPointsCalculator {
    fn calculate(&self, fastest: Time, driver: &Driver, pax_multiplier: Option<Time>) -> i64 {
        let pax_multiplier = pax_multiplier.unwrap_or(1.);
        let actual = driver.best_lap(None).time.unwrap_or(Time::INFINITY) * pax_multiplier;
        if approx_eq!(Time, fastest, actual) {
            10_000
        } else {
            ((fastest / actual) * 10_000.).round() as i64
        }
    }
}

pub struct DefaultChampionshipPointsCalculator {}

impl ChampionshipPointsCalculator for DefaultChampionshipPointsCalculator {}
