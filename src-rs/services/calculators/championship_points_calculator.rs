use crate::models::driver::Driver;
use crate::models::lap_time::LapTime;
use bigdecimal::{BigDecimal, ToPrimitive};

pub trait ChampionshipPointsCalculator {
    fn calculate(&self, fastest: &LapTime, driver: &Driver, pro: bool) -> i64 {
        let actual = if pro {
            driver.best_pro_lap()
        } else {
            driver.best_standard_lap()
        };
        if fastest == &actual {
            10_000
        } else {
            match (fastest.with_pax(), actual.with_pax()) {
                (Some(fastest), Some(actual)) => ((fastest * BigDecimal::from(10_000)) / actual).to_i64().unwrap(),
                (None, Some(_)) => 10_000,
                _ => 0,
            }
        }
    }
}

pub struct DefaultChampionshipPointsCalculator {}

impl ChampionshipPointsCalculator for DefaultChampionshipPointsCalculator {}
