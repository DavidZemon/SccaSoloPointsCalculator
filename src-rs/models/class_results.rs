use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::models::car_class::{get_car_class, CarClass};
use crate::models::driver::{Driver, TimeSelection};
use crate::models::lap_time::LapTime;
use crate::models::short_car_class::ShortCarClass;
use crate::models::type_aliases::Time;

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClassResults {
    pub trophy_count: u8,
    pub car_class: CarClass,
    #[wasm_bindgen(skip)]
    pub drivers: Vec<Driver>,
}

#[wasm_bindgen]
impl ClassResults {
    pub fn get_drivers(&self) -> Vec<JsValue> {
        self.drivers
            .iter()
            .map(|d| serde_wasm_bindgen::to_value(d).unwrap())
            .collect()
    }
}

impl ClassResults {
    pub fn new(car_class: ShortCarClass) -> ClassResults {
        ClassResults {
            trophy_count: 0,
            car_class: get_car_class(car_class).unwrap(),
            drivers: Vec::new(),
        }
    }

    pub fn add_driver(&mut self, driver: Driver) {
        self.drivers.push(driver);
        self.trophy_count = self.calculate_trophies();
        self.drivers.sort();
        for (index, driver) in self.drivers.iter_mut().enumerate() {
            driver.position = Some(index + 1);
        }
    }

    pub fn get_best_in_class(&self, time_selection: Option<TimeSelection>) -> Time {
        let mut best_laps: Vec<LapTime> = self
            .drivers
            .iter()
            .map(|d| d.best_lap(time_selection))
            .collect();
        best_laps.sort();
        best_laps
            .get(0)
            .map(|t| t.time.clone())
            .flatten()
            .unwrap_or(Time::INFINITY)
    }

    fn calculate_trophies(&self) -> u8 {
        let driver_count = self.drivers.len();
        if driver_count <= 1 {
            0
        } else if driver_count >= 10 {
            (3. + ((driver_count as f32) - 9.) / 4.).ceil() as u8
        } else {
            ((driver_count as f32) / 3.).ceil() as u8
        }
    }
}

#[cfg(test)]
mod test {
    use crate::models::class_results::ClassResults;
    use crate::models::driver::{Driver, TimeSelection};
    use crate::models::exported_driver::ExportedDriver;
    use crate::models::lap_time::LapTime;
    use crate::models::short_car_class::ShortCarClass;

    fn build_driver(day1: Option<Vec<LapTime>>, day2: Option<Vec<LapTime>>) -> Driver {
        Driver::from(
            ExportedDriver {
                position: None,
                car_class: ShortCarClass::SS,
                car_number: 0,
                first_name: None,
                last_name: None,
                year: None,
                make: None,
                model: None,
                color: None,
                member_number: None,
                rookie: None,
                ladies: None,
                dsq: None,
                region: None,
                best_run: "".to_string(),
                pax_multiplier: 0.0,
                pax_time: 0.0,
                runs_day1: None,
                runs_day2: None,
                day1,
                day2,
            },
            true,
        )
    }

    #[test]
    fn trophy_count() {
        let mut testable = ClassResults::new(ShortCarClass::AS);

        assert_eq!(testable.trophy_count, 0);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 0);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 1);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 1);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 2);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 2);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 2);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 3);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 3);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 3);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 4);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 4);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 4);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 4);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 5);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 5);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 5);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 5);
        testable.add_driver(build_driver(None, None));
        assert_eq!(testable.trophy_count, 6);
    }

    #[test]
    fn get_best_in_class_day1() {
        let mut testable = ClassResults::new(ShortCarClass::AS);
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(4., 0, None)]),
            Some(vec![LapTime::new(40., 0, None)]),
        ));
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(3., 0, None)]),
            Some(vec![LapTime::new(30., 0, None)]),
        ));
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(5., 0, None)]),
            Some(vec![LapTime::new(500., 0, None)]),
        ));

        assert_eq!(testable.get_best_in_class(None), 3.);
        assert_eq!(testable.get_best_in_class(Some(TimeSelection::Day1)), 3.);
    }

    #[test]
    fn get_best_in_class_day2() {
        let mut testable = ClassResults::new(ShortCarClass::AS);
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(4., 0, None)]),
            Some(vec![LapTime::new(40., 0, None)]),
        ));
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(3., 0, None)]),
            Some(vec![LapTime::new(30., 0, None)]),
        ));
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(5., 0, None)]),
            Some(vec![LapTime::new(500., 0, None)]),
        ));

        assert_eq!(testable.get_best_in_class(Some(TimeSelection::Day2)), 30.);
    }

    #[test]
    fn get_best_in_class_combined() {
        let mut testable = ClassResults::new(ShortCarClass::AS);
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(4., 0, None)]),
            Some(vec![LapTime::new(40., 0, None)]),
        ));
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(3., 0, None)]),
            Some(vec![LapTime::new(30., 0, None)]),
        ));
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(5., 0, None)]),
            Some(vec![LapTime::new(500., 0, None)]),
        ));

        assert_eq!(
            testable.get_best_in_class(Some(TimeSelection::Combined)),
            33.
        );
    }
}
