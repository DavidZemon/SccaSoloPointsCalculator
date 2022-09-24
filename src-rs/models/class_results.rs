use crate::enums::short_car_class::ShortCarClass;
use crate::models::car_class::{get_car_class, CarClass};
use crate::models::driver::{Driver, TimeSelection};
use crate::models::lap_time::{dns, LapTime};

#[derive(Clone, Debug)]
pub struct ClassResults {
    pub trophy_count: u8,
    pub car_class: CarClass,
    pub drivers: Vec<Driver>,
}

impl ClassResults {
    pub fn new(car_class: ShortCarClass) -> ClassResults {
        ClassResults {
            trophy_count: 0,
            car_class: get_car_class(&car_class).unwrap(),
            drivers: Vec::new(),
        }
    }

    pub fn get_best_in_class(&self, time_selection: Option<TimeSelection>) -> LapTime {
        // Drivers are sorted as they are added via add_driver(), so just take the first
        // on the list
        self.drivers
            .get(0)
            .map(|d| d.best_lap(time_selection))
            .unwrap_or_else(dns)
    }

    pub fn add_driver(&mut self, driver: Driver) {
        self.drivers.push(driver);
        self.trophy_count = self.calculate_trophy_count();
        self.drivers.sort();
        for (index, driver) in self.drivers.iter_mut().enumerate() {
            driver.position = Some(index + 1);
        }
    }

    fn calculate_trophy_count(&self) -> u8 {
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
    use crate::enums::short_car_class::ShortCarClass;
    use crate::models::class_results::ClassResults;
    use crate::models::driver::{Driver, TimeSelection};
    use crate::models::driver_from_pronto::DriverFromPronto;
    use crate::models::lap_time::LapTime;

    fn build_driver(day1: Option<Vec<LapTime>>, day2: Option<Vec<LapTime>>) -> Driver {
        Driver::from(
            DriverFromPronto {
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
            Some(vec![LapTime::new(4., 0.8, 0, None)]),
            Some(vec![LapTime::new(40., 0.8, 0, None)]),
        ));
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(3., 0.9, 0, None)]),
            Some(vec![LapTime::new(30., 0.9, 0, None)]),
        ));
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(5., 0.7, 0, None)]),
            Some(vec![LapTime::new(500., 0.7, 0, None)]),
        ));

        assert_eq!(
            testable.get_best_in_class(None),
            LapTime::new(3., 0.9, 0, None)
        );
        assert_eq!(
            testable.get_best_in_class(Some(TimeSelection::Day1)),
            LapTime::new(3., 0.9, 0, None)
        );
    }

    #[test]
    fn get_best_in_class_day2() {
        let mut testable = ClassResults::new(ShortCarClass::AS);
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(4., 0.8, 0, None)]),
            Some(vec![LapTime::new(40., 0.8, 0, None)]),
        ));
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(3., 0.9, 0, None)]),
            Some(vec![LapTime::new(30., 0.9, 0, None)]),
        ));
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(5., 0.7, 0, None)]),
            Some(vec![LapTime::new(500., 0.7, 0, None)]),
        ));

        assert_eq!(
            testable.get_best_in_class(Some(TimeSelection::Day2)),
            LapTime::new(30., 0.9, 0, None)
        );
    }

    #[test]
    fn get_best_in_class_combined() {
        let mut testable = ClassResults::new(ShortCarClass::AS);
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(4., 0.8, 0, None)]),
            Some(vec![LapTime::new(40., 0.8, 0, None)]),
        ));
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(3., 0.9, 0, None)]),
            Some(vec![LapTime::new(30., 0.9, 0, None)]),
        ));
        testable.add_driver(build_driver(
            Some(vec![LapTime::new(5., 0.7, 0, None)]),
            Some(vec![LapTime::new(500., 0.7, 0, None)]),
        ));

        assert_eq!(
            testable.get_best_in_class(Some(TimeSelection::Combined)),
            LapTime::new(33., 0.9, 0, None)
        );
    }
}
