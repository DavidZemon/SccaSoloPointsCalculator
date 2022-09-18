use crate::models::driver::Driver;
use crate::models::lap_time::LapTime;

pub trait ChampionshipPointsCalculator {
    fn calculate(&self, fastest: &LapTime, driver: &Driver) -> i64 {
        let actual = driver.best_lap(None);
        if fastest == &actual {
            10_000
        } else {
            ((fastest.with_pax() / actual.with_pax()) * 10_000.).round() as i64
        }
    }
}

pub struct DefaultChampionshipPointsCalculator {}

impl ChampionshipPointsCalculator for DefaultChampionshipPointsCalculator {}
