use std::cmp::Ordering;

use crate::models::car_class::{get_car_class, CarClass};
use crate::models::exported_driver::ExportedDriver;
use crate::models::lap_time::LapTime;
use crate::models::type_aliases::{DriverId, Time};

#[derive(Copy, Clone, Debug)]
pub enum TimeSelection {
    Day1,
    Day2,
    Combined,
}

#[derive(Clone)]
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
    pub position: Option<usize>,
    pub dsq: bool,
    pub pax_multiplier: f64,

    pub day_1_times: Option<Vec<LapTime>>,
    pub day_2_times: Option<Vec<LapTime>>,
    pub combined: LapTime,

    /// For internal use only to help sort and compute combined time
    two_day_event: bool,
}

impl Driver {
    pub fn from(driver: ExportedDriver, two_day_event: bool) -> Driver {
        let best_run_is_truthy = driver
            .best_run
            .parse::<f64>()
            .map_or_else(|_| driver.best_run.trim().is_empty(), |f| f == 0.);

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

        let day_1_times = driver.day1.map(|mut times| {
            times.sort();
            times
        });
        let day_2_times = driver.day2.map(|mut times| {
            times.sort();
            times
        });

        let mut driver = Driver {
            error: driver.runs_day1.is_none() && driver.runs_day2.is_none() && best_run_is_truthy,
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
            dsq: driver.dsq.map(|dsq| dsq == 1).unwrap_or(false),
            pax_multiplier: driver.pax_multiplier,
            day_1_times: day_1_times.clone(),
            day_2_times: day_2_times.clone(),
            combined: LapTime::dns(),
            two_day_event,
        };
        driver.combined = driver.best_lap(Some(TimeSelection::Combined));
        driver
    }

    pub fn best_lap(&self, time_selection: Option<TimeSelection>) -> LapTime {
        if self.dsq {
            LapTime::dsq()
        } else {
            match time_selection.unwrap_or(TimeSelection::Day1) {
                TimeSelection::Day1 => self
                    .day_1_times
                    .clone()
                    .map(|times| times.get(0).map(|t| t.clone()))
                    .flatten()
                    .unwrap_or(LapTime::dns()),
                TimeSelection::Day2 => self
                    .day_2_times
                    .clone()
                    .map(|times| times.get(0).map(|t| t.clone()))
                    .flatten()
                    .unwrap_or(LapTime::dns()),
                TimeSelection::Combined => {
                    let day_1_empty = self
                        .day_1_times
                        .clone()
                        .map(|times| times.is_empty())
                        .unwrap_or(true);
                    let day_2_empty = self
                        .day_2_times
                        .clone()
                        .map(|times| times.is_empty())
                        .unwrap_or(true);

                    if self.two_day_event {
                        if day_1_empty || day_2_empty {
                            LapTime::dns()
                        } else {
                            self.best_lap(Some(TimeSelection::Day1))
                                .add(self.best_lap(Some(TimeSelection::Day2)))
                        }
                    } else {
                        if day_2_empty {
                            self.best_lap(Some(TimeSelection::Day1))
                        } else if day_1_empty {
                            self.best_lap(Some(TimeSelection::Day2))
                        } else {
                            panic!("Asking for combined time for a one-day event but driver {} has times for both days!", self.name)
                        }
                    }
                }
            }
        }
    }

    pub fn get_times(&self, time_selection: Option<TimeSelection>) -> Option<Vec<LapTime>> {
        let selection = match time_selection {
            Some(t) => t,
            None => TimeSelection::Day1,
        };
        match selection {
            TimeSelection::Day1 => self.day_1_times.clone(),
            TimeSelection::Day2 => self.day_2_times.clone(),
            TimeSelection::Combined => {
                panic!("Silly person! I can't give you an array of times for the 'combined' time!")
            }
        }
    }

    pub fn differences(
        &self,
        fastest_of_day: Option<Time>,
        use_pax: Option<bool>,
        time_selection: Option<TimeSelection>,
    ) -> String {
        let time_to_compare = self.best_lap(time_selection);
        match time_to_compare.time {
            Some(t) => {
                let multiplier = if use_pax.unwrap_or(false) {
                    self.pax_multiplier
                } else {
                    1.
                };
                let indexed_time = multiplier * t;
                match fastest_of_day {
                    Some(fastest) => {
                        if indexed_time == fastest {
                            String::from("")
                        } else {
                            String::from(format!("{:.3}", fastest - indexed_time))
                        }
                    }
                    None => panic!("Asking for time difference but no fastest given"),
                }
            }
            None => String::from("N/A"),
        }
    }
}

impl Ord for Driver {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd<Self> for Driver {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.combined.partial_cmp(&other.combined)
    }
}

impl PartialEq<Self> for Driver {
    fn eq(&self, other: &Self) -> bool {
        (self.error == other.error)
            && self.id == other.id
            && self.name == other.name
            && self.car_number == other.car_number
            && (self.car_class.short == other.car_class.short)
            && self.car_description == other.car_description
            && self.region == other.region
            && self.rookie == other.rookie
            && self.ladies_championship == other.ladies_championship
            && self.position == other.position
            && self.dsq == other.dsq
            && self.pax_multiplier == other.pax_multiplier
            && self.day_1_times == other.day_1_times
            && self.day_2_times == other.day_2_times
            && self.combined == other.combined
    }
}

impl Eq for Driver {}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::models::driver::{Driver, TimeSelection};
    use crate::models::exported_driver::ExportedDriver;
    use crate::models::lap_time::{LapTime, Penalty};
    use crate::models::short_car_class::ShortCarClass;

    fn build_driver(
        d1: Option<Vec<LapTime>>,
        d2: Option<Vec<LapTime>>,
        dsq: bool,
        two_day: bool,
    ) -> Driver {
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
                dsq: Some(if dsq { 1 } else { 0 }),
                region: None,
                best_run: "".to_string(),
                pax_multiplier: 0.0,
                pax_time: 0.0,
                runs_day1: None,
                runs_day2: None,
                day1: d1,
                day2: d2,
            },
            two_day,
        )
    }

    #[rstest]
    #[case(None, None)]
    #[case(None, Some(vec![LapTime::new(2., 0, None)]))]
    #[case(Some(vec![LapTime::new(2., 0, None)]), None)]
    #[case(Some(vec![LapTime::new(1., 0, None)]), Some(vec![LapTime::new(2., 0, None)]))]
    fn best_lap_should_return_dsq_for_dsq(
        #[case] d1: Option<Vec<LapTime>>,
        #[case] d2: Option<Vec<LapTime>>,
    ) {
        for ts in vec![
            None,
            Some(TimeSelection::Day1),
            Some(TimeSelection::Day2),
            Some(TimeSelection::Combined),
        ] {
            assert_eq!(
                build_driver(d1.clone(), d2.clone(), true, false).best_lap(ts),
                LapTime::dsq()
            );
            assert_eq!(
                build_driver(d1.clone(), d2.clone(), true, true).best_lap(ts),
                LapTime::dsq()
            );
        }
    }

    #[rstest]
    #[case(None, None, None)]
    #[case(None, Some(vec![]), None)]
    #[case(None, Some(vec![LapTime::new(2., 0, None)]), None)]
    #[case(None, None, Some(TimeSelection::Day1))]
    #[case(None, Some(vec![]), Some(TimeSelection::Day1))]
    #[case(None, Some(vec![LapTime::new(2., 0, None)]), Some(TimeSelection::Day1))]
    #[case(None, None, Some(TimeSelection::Day2))]
    #[case(Some(vec![]), None, Some(TimeSelection::Day2))]
    #[case(Some(vec![LapTime::new(2., 0, None)]), None, Some(TimeSelection::Day2))]
    #[case(None, None, Some(TimeSelection::Day2))]
    #[case(Some(vec![]), None, Some(TimeSelection::Day2))]
    #[case(None, Some(vec![]), Some(TimeSelection::Day2))]
    #[case(Some(vec![]), Some(vec![]), Some(TimeSelection::Day2))]
    #[case(None, None, Some(TimeSelection::Combined))]
    fn best_lap_should_return_dns_for_missing_times(
        #[case] d1: Option<Vec<LapTime>>,
        #[case] d2: Option<Vec<LapTime>>,
        #[case] ts: Option<TimeSelection>,
    ) {
        assert_eq!(
            build_driver(d1.clone(), d2.clone(), false, false).best_lap(ts),
            LapTime::dns()
        );
        assert_eq!(
            build_driver(d1.clone(), d2.clone(), false, true).best_lap(ts),
            LapTime::dns()
        );
    }

    #[rstest]
    #[case(Some(vec![LapTime::new(2., 0, None)]), None, Some(TimeSelection::Combined))]
    #[case(Some(vec![LapTime::new(2., 0, None)]), Some(vec![]), Some(TimeSelection::Combined))]
    #[case(None, Some(vec![LapTime::new(2., 0, None)]), Some(TimeSelection::Combined))]
    #[case(Some(vec![]), Some(vec![LapTime::new(2., 0, None)]), Some(TimeSelection::Combined))]
    fn best_lap_should_return_dns_for_special_two_day_event_cases(
        #[case] d1: Option<Vec<LapTime>>,
        #[case] d2: Option<Vec<LapTime>>,
        #[case] ts: Option<TimeSelection>,
    ) {
        assert_eq!(
            build_driver(d1.clone(), d2.clone(), false, true).best_lap(ts),
            LapTime::dns()
        );
    }

    #[rstest]
    #[case(
        Some(vec![
            LapTime::new(2., 0, Some(Penalty::DNF)),
            LapTime::new(6., 1, None),
            LapTime::new(9., 0, None)
        ]),
        None,
        false,
        None,
        LapTime::new(6., 1, None),
    )]
    #[case(
        Some(vec![
            LapTime::new(2., 0, Some(Penalty::DNF)),
            LapTime::new(6., 1, None),
            LapTime::new(9., 0, None)
        ]),
        None,
        false,
        Some(TimeSelection::Day1),
        LapTime::new(6., 1, None),
    )]
    #[case(
        Some(vec![
            LapTime::new(2., 0, Some(Penalty::DNF)),
            LapTime::new(6., 1, None),
            LapTime::new(9., 0, None)
        ]),
        Some(vec![]),
        false,
        Some(TimeSelection::Day1),
        LapTime::new(6., 1, None),
    )]
    #[case(
        Some(vec![
            LapTime::new(2., 0, Some(Penalty::DNF)),
            LapTime::new(6., 1, None),
            LapTime::new(9., 0, None)
        ]),
        None,
        true,
        Some(TimeSelection::Day1),
        LapTime::new(6., 1, None),
    )]
    #[case(
        Some(vec![
            LapTime::new(2., 0, Some(Penalty::DNF)),
            LapTime::new(6., 1, None),
            LapTime::new(9., 0, None)
        ]),
        Some(vec![]),
        true,
        Some(TimeSelection::Day1),
        LapTime::new(6., 1, None),
    )]
    #[case(
        Some(vec![
            LapTime::new(2., 0, Some(Penalty::DNF)),
            LapTime::new(6., 1, None),
            LapTime::new(9., 0, None)
        ]),
        Some(vec![LapTime::new(1., 0, None)]),
        true,
        Some(TimeSelection::Day1),
        LapTime::new(6., 1, None),
    )]
    fn best_lap_happy_path_day1(
        #[case] d1: Option<Vec<LapTime>>,
        #[case] d2: Option<Vec<LapTime>>,
        #[case] two_day: bool,
        #[case] ts: Option<TimeSelection>,
        #[case] expected: LapTime,
    ) {
        assert_eq!(build_driver(d1, d2, false, two_day).best_lap(ts), expected);
    }

    #[rstest]
    #[case(
        None,
        Some(vec![
            LapTime::new(2., 0, Some(Penalty::DNF)),
            LapTime::new(6., 1, None),
            LapTime::new(9., 0, None)
        ]),
        false,
        LapTime::new(6., 1, None),
    )]
    #[case(
        Some(vec![]),
        Some(vec![
            LapTime::new(2., 0, Some(Penalty::DNF)),
            LapTime::new(6., 1, None),
            LapTime::new(9., 0, None)
        ]),
        false,
        LapTime::new(6., 1, None),
    )]
    #[case(
        None,
        Some(vec![
            LapTime::new(2., 0, Some(Penalty::DNF)),
            LapTime::new(6., 1, None),
            LapTime::new(9., 0, None)
        ]),
        true,
        LapTime::new(6., 1, None),
    )]
    #[case(
        Some(vec![]),
        Some(vec![
            LapTime::new(2., 0, Some(Penalty::DNF)),
            LapTime::new(6., 1, None),
            LapTime::new(9., 0, None)
        ]),
        true,
        LapTime::new(6., 1, None),
    )]
    #[case(
        Some(vec![LapTime::new(1., 0, None)]),
        Some(vec![
            LapTime::new(2., 0, Some(Penalty::DNF)),
            LapTime::new(6., 1, None),
            LapTime::new(9., 0, None)
        ]),
        true,
        LapTime::new(6., 1, None),
    )]
    fn best_lap_happy_path_day2(
        #[case] d1: Option<Vec<LapTime>>,
        #[case] d2: Option<Vec<LapTime>>,
        #[case] two_day: bool,
        #[case] expected: LapTime,
    ) {
        assert_eq!(
            build_driver(d1, d2, false, two_day).best_lap(Some(TimeSelection::Day2)),
            expected
        );
    }

    #[test]
    fn best_lap_happy_path_combined() {
        assert_eq!(
            build_driver(
                Some(vec![
                    LapTime::new(20., 0, Some(Penalty::DNF)),
                    LapTime::new(60., 2, None),
                    LapTime::new(90., 0, None)
                ]),
                Some(vec![
                    LapTime::new(2., 0, Some(Penalty::DNF)),
                    LapTime::new(6., 1, None),
                    LapTime::new(9., 0, None)
                ]),
                false,
                true
            )
            .best_lap(Some(TimeSelection::Combined)),
            LapTime::new(66., 3, None)
        );
    }
}
