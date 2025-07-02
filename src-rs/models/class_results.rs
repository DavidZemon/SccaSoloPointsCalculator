use crate::enums::short_car_class::ShortCarClass;
use crate::models::car_class::{get_car_class, CarClass};
use crate::models::driver::Driver;
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

    pub fn get_best_in_class(&self) -> LapTime {
        // Drivers are sorted as they are added via add_driver(), so just take the first
        // on the list
        self.drivers
            .first()
            .map(|d| {
                if self.car_class.short == ShortCarClass::P {
                    d.best_pro_lap()
                } else {
                    d.best_standard_lap()
                }
            })
            .unwrap_or_else(dns)
    }

    pub fn add_driver(&mut self, driver: Driver) {
        self.drivers.push(driver);
        self.trophy_count = self.calculate_trophy_count();

        self.drivers.sort_by(|lhs, rhs| match self.car_class.short {
            ShortCarClass::P => lhs.best_pro_lap().cmp(&rhs.best_pro_lap()),
            _ => lhs.best_standard_lap().cmp(&rhs.best_standard_lap()),
        });

        for (index, driver) in self.drivers.iter_mut().enumerate() {
            driver.set_position(index + 1);
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
    use crate::models::driver::Driver;
    use crate::models::driver_from_pronto::DriverFromPronto;
    use crate::models::lap_time::LapTime;
    use crate::models::msr_driver::MsrDriver;
    use crate::models::type_aliases::PaxMultiplier;
    use std::str::FromStr;

    fn build_driver(runs: Vec<LapTime>) -> Driver {
        Driver::from((
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
                dsq: None,
                best_run: "".to_string(),
                pax_multiplier: "0.0".to_string(),
                pax_time: "0.0".to_string(),
                runs,
            },
            &MsrDriver {
                last_name: "".to_string(),
                first_name: "".to_string(),
                member_number: "".to_string(),
                class_and_pax: "SS".to_string(),
                car_number: 0,
                car: "".to_string(),
                region: None,
                medical: None,
                novice: Some(0),
                ladies: Some(0),
            },
        ))
    }

    #[test]
    fn trophy_count() {
        let mut testable = ClassResults::new(ShortCarClass::AS);

        assert_eq!(testable.trophy_count, 0);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 0);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 1);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 1);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 2);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 2);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 2);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 3);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 3);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 3);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 4);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 4);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 4);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 4);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 5);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 5);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 5);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 5);
        testable.add_driver(build_driver(vec![]));
        assert_eq!(testable.trophy_count, 6);
    }

    #[test]
    fn get_best_in_class() {
        let mut testable = ClassResults::new(ShortCarClass::AS);
        testable.add_driver(build_driver(vec![LapTime::new(
            4.into(),
            PaxMultiplier::from_str("0.8").unwrap(),
            0,
            None,
        )]));
        testable.add_driver(build_driver(vec![LapTime::new(
            3.into(),
            PaxMultiplier::from_str("0.9").unwrap(),
            0,
            None,
        )]));
        testable.add_driver(build_driver(vec![LapTime::new(
            5.into(),
            PaxMultiplier::from_str("0.7").unwrap(),
            0,
            None,
        )]));

        assert_eq!(
            testable.get_best_in_class(),
            LapTime::new(3.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)
        );
    }
}
