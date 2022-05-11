use std::collections::HashMap;

use crate::models::car_class::{get_car_class, CarClass};
use crate::models::driver::{Driver, TimeSelection};
use crate::models::lap_time::LapTime;
use crate::models::short_car_class::ShortCarClass;
use crate::models::type_aliases::Time;

#[derive(Clone)]
pub struct ClassResults {
    pub trophy_count: u8,
    pub car_class: CarClass,
    drivers: Vec<Driver>,
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
        for (index, mut driver) in self.drivers.iter_mut().enumerate() {
            driver.position = Some(index + 1);
        }
    }

    pub fn get_drivers(&self) -> &Vec<Driver> {
        &self.drivers
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
            .map(|t| t.time)
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

pub type EventResults = HashMap<ShortCarClass, ClassResults>;

#[cfg(test)]
mod test {
    use crate::models::car_class::get_car_class;
    use crate::models::class_results::ClassResults;
    use crate::models::driver::Driver;
    use crate::models::lap_time::LapTime;
    use crate::models::short_car_class::ShortCarClass;

    fn build_driver() -> Driver {
        Driver {
            error: false,
            id: "".to_string(),
            name: "".to_string(),
            car_number: 0,
            car_class: get_car_class(ShortCarClass::AS).unwrap(),
            car_description: "".to_string(),
            region: "".to_string(),
            rookie: false,
            ladies_championship: false,
            position: None,
            dsq: false,
            pax_multiplier: 0.0,
            day_1_times: None,
            day_2_times: None,
            combined: LapTime::dsq(),
        }
    }

    #[test]
    fn trophy_count() {
        let mut testable = ClassResults::new(ShortCarClass::AS);

        assert_eq!(testable.trophy_count, 0);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 0);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 1);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 1);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 2);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 2);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 2);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 3);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 3);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 3);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 4);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 4);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 4);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 4);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 5);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 5);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 5);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 5);
        testable.add_driver(build_driver());
        assert_eq!(testable.trophy_count, 6);
    }

    #[test]
    fn get_best_in_class() {
        let mut testable = ClassResults::new(ShortCarClass::AS);
        testable.add_driver(build_driver());
    }
}
