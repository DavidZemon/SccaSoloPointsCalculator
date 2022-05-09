use crate::models::car_class::{get_car_class, CarClass};
use crate::models::driver::{Driver, TimeSelection};
use crate::models::lap_time::LapTime;
use crate::models::short_car_class::ShortCarClass;
use crate::models::type_aliases::Time;
use std::collections::HashMap;

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
    }

    pub fn get_drivers(&self) -> &Vec<Driver> {
        &self.drivers
    }

    pub fn get_best_in_class(&self, time_selection: Option<TimeSelection>) -> Time {
        let mut foo: Vec<LapTime> = self
            .drivers
            .iter()
            .map(|d| d.best_lap(time_selection.clone()))
            .collect();
        foo.sort();
        foo.get(0)
            .unwrap_or(&LapTime::dns())
            .time
            .unwrap_or(Time::INFINITY)
    }
}

pub type EventResults = HashMap<ShortCarClass, ClassResults>;
