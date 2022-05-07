use crate::models::car_class::{get_car_class, CarClass};
use crate::models::exported_driver::{BestRun, ExportedDriver};
use crate::models::lap_time::LapTime;
use crate::models::type_aliases::DriverId;

pub enum TimeSelection {
    Day1,
    Day2,
    Combined,
}

pub struct Driver {
    pub error: bool,
    pub id: DriverId,
    pub name: String,
    pub car_number: u16,
    pub car_class: CarClass,
    pub car_description: String,
    pub region: String,
    pub rookie: bool,
    pub ladies_championship: bool,
    pub position: Option<u16>,
    pub dsq: bool,
    pub pax_multiplier: f64,

    pub day_1_times: Option<Vec<LapTime>>,
    pub day_2_times: Option<Vec<LapTime>>,
    pub combined: LapTime,
}

impl Driver {
    pub fn from(driver: &ExportedDriver) -> Driver {
        let first_name = driver
            .first_name
            .clone()
            .unwrap_or(String::from("<Missing First Name>"));
        let last_name = driver
            .last_name
            .clone()
            .unwrap_or(String::from("<Missing Last Name>"));
        let name = format!("{} {}", first_name, last_name);
        let car_class = match get_car_class(driver.car_class) {
            Some(c) => c,
            None => panic!("Unable to map class for driver {}", driver.car_class.name()),
        };

        Driver {
            error: driver.runs_day1.is_none()
                && driver.runs_day2.is_none()
                && match driver.best_run {
                    BestRun::Time(t) => t == 0.,
                    BestRun::Penalty(ref reason) => !reason.is_empty(),
                },
            rookie: driver.rookie.map_or(false, |value| value != 0),
            ladies_championship: driver.ladies.map_or(false, |value| value != 0),
            position: None,
            car_number: driver.car_number,
            car_class,
            name: name.clone(),
            id: String::from(name.to_lowercase().trim()),
            car_description: format!(
                "{} {} {}",
                driver.year.unwrap_or(0),
                driver.make.clone().unwrap_or(String::from("Unknown")),
                driver.model.clone().unwrap_or(String::from("Unknown"))
            ),
            region: driver.region.clone().unwrap_or(String::from("")),
            dsq: false,
            pax_multiplier: driver.pax_multiplier,
            day_1_times: driver.day1.clone(),
            day_2_times: driver.day2.clone(),
            combined: Driver::_best_lap(
                false,
                &driver.day1,
                &driver.day2,
                Some(TimeSelection::Combined),
            ),
        }
    }

    fn _best_lap(
        dsq: bool,
        day_1_times: &Option<Vec<LapTime>>,
        day_2_times: &Option<Vec<LapTime>>,
        time_selection: Option<TimeSelection>,
    ) -> LapTime {
        if dsq {
            LapTime::dsq()
        } else {
            match time_selection.unwrap_or(TimeSelection::Day1) {
                TimeSelection::Day1 => day_1_times
                    .as_ref()
                    .map_or(None, |times| times.get(0))
                    .unwrap_or(&LapTime::dns())
                    .clone(),
                TimeSelection::Day2 => day_2_times
                    .as_ref()
                    .map_or(None, |times| times.get(0))
                    .unwrap_or(&LapTime::dns())
                    .clone(),
                TimeSelection::Combined => {
                    if day_1_times
                        .as_ref()
                        .map(|times| !times.is_empty())
                        .unwrap_or(false)
                        && day_2_times
                            .as_ref()
                            .map(|times| !times.is_empty())
                            .unwrap_or(false)
                    {
                        Driver::_best_lap(dsq, day_1_times, day_2_times, Some(TimeSelection::Day1))
                            .add(Driver::_best_lap(
                                dsq,
                                day_1_times,
                                day_2_times,
                                Some(TimeSelection::Day2),
                            ))
                    } else {
                        LapTime::dns()
                    }
                }
            }
        }
    }

    pub fn best_lap(&self, time_selection: Option<TimeSelection>) -> LapTime {
        Driver::_best_lap(
            self.dsq,
            &self.day_1_times,
            &self.day_2_times,
            time_selection,
        )
    }
}
