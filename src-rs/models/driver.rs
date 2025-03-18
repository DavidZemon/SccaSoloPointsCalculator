use crate::models::car_class::{get_car_class, CarClass};
use crate::models::driver_from_pronto::DriverFromPronto;
use crate::models::lap_time::{dns, dsq, LapTime};
use crate::models::type_aliases::{DriverId, PaxMultiplier};
use std::cmp::min;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum TimeSelection {
    Day1,
    Day2,
    Combined,
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub xpert: bool,
    pub position: Option<usize>,
    pub dsq: bool,
    pub pax_multiplier: PaxMultiplier,
    pub day_1_times: Option<Vec<LapTime>>,
    pub day_2_times: Option<Vec<LapTime>>,
    pub combined: LapTime,

    /// For internal use only to help sort and compute combined time
    two_day_event: bool,
}

impl Driver {
    pub fn from(driver: DriverFromPronto, two_day_event: bool) -> Driver {
        let best_run_is_falsy = driver
            .best_run
            .parse::<f64>()
            .map_or_else(|_| driver.best_run.trim().is_empty(), |f| f == 0.);

        let first_name = driver
            .first_name
            .clone()
            .unwrap_or_else(|| "<Missing First Name>".to_string());
        let last_name = driver
            .last_name
            .clone()
            .unwrap_or_else(|| "<Missing Last Name>".to_string());
        let name = format!("{} {}", first_name, last_name);
        let car_class = match get_car_class(&driver.car_class) {
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
            error: driver.runs_day1.is_none() && driver.runs_day2.is_none() && !best_run_is_falsy,
            rookie: driver.rookie.map_or(false, |value| value != 0),
            ladies_championship: driver.ladies.map_or(false, |value| value != "0" && !value.is_empty()),
            xpert: driver.xpert.map_or(false, |value| value != 0),
            position: None,
            car_number: driver.car_number,
            car_class,
            name: name.clone(),
            id: name.to_lowercase().trim().to_string(),
            car_description: format!(
                "{} {} {}",
                driver.year.unwrap_or(0),
                driver.make.clone().unwrap_or_else(|| "Unknown".to_string()),
                driver.model.clone().unwrap_or_else(|| "Unknown".to_string())
            ),
            region: driver.region.clone().unwrap_or_default(),
            dsq: driver.dsq.map(|dsq| dsq == 1).unwrap_or(false),
            pax_multiplier: PaxMultiplier::from_str(&driver.pax_multiplier).unwrap(),
            day_1_times,
            day_2_times,
            combined: dns(),
            two_day_event,
        };
        driver.combined = driver.best_xpert_lap(Some(TimeSelection::Combined));
        driver
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = Some(position);
    }

    pub fn best_standard_lap(&self, time_selection: Option<TimeSelection>) -> LapTime {
        self.best_lap_in_range(false, time_selection)
    }

    pub fn best_xpert_lap(&self, time_selection: Option<TimeSelection>) -> LapTime {
        self.best_lap_in_range(true, time_selection)
    }

    pub fn best_lap(&self, xpert: bool, time_selection: Option<TimeSelection>) -> LapTime {
        if xpert {
            self.best_xpert_lap(time_selection)
        } else {
            self.best_standard_lap(time_selection)
        }
    }

    fn best_lap_in_range(&self, best_of_three: bool, time_selection: Option<TimeSelection>) -> LapTime {
        if self.dsq {
            dsq()
        } else {
            match time_selection.unwrap_or(TimeSelection::Day1) {
                TimeSelection::Day1 => self
                    .day_1_times
                    .as_ref()
                    .and_then(|times| {
                        let mut times = Self::lap_times_for_range(times, best_of_three);
                        times.sort();
                        times.first().cloned()
                    })
                    .unwrap_or_else(dns),
                TimeSelection::Day2 => self
                    .day_2_times
                    .as_ref()
                    .and_then(|times| {
                        let mut times = Self::lap_times_for_range(times, best_of_three);
                        times.sort();
                        times.first().cloned()
                    })
                    .unwrap_or_else(dns),
                TimeSelection::Combined => {
                    let day_1_empty = self.day_1_times.as_ref().map(|times| times.is_empty()).unwrap_or(true);
                    let day_2_empty = self.day_2_times.as_ref().map(|times| times.is_empty()).unwrap_or(true);

                    if self.two_day_event {
                        if day_1_empty || day_2_empty {
                            dns()
                        } else {
                            self.best_lap_in_range(best_of_three, Some(TimeSelection::Day1))
                                .add(self.best_lap_in_range(best_of_three, Some(TimeSelection::Day2)))
                        }
                    } else if day_2_empty {
                        self.best_lap_in_range(best_of_three, Some(TimeSelection::Day1))
                    } else if day_1_empty {
                        self.best_lap_in_range(best_of_three, Some(TimeSelection::Day2))
                    } else {
                        panic!(
                            "Asking for combined time for a one-day event but driver {} has times for both days!",
                            self.name
                        )
                    }
                }
            }
        }
    }

    fn lap_times_for_range(times: &[LapTime], best_of_three: bool) -> Vec<LapTime> {
        if best_of_three {
            times[..min(times.len(), 3)].to_vec()
        } else {
            times.to_vec()
        }
    }

    pub fn difference(
        &self,
        comparison: LapTime,
        use_pax: bool,
        use_xpert: bool,
        time_selection: Option<TimeSelection>,
    ) -> String {
        let self_best_lap = if use_xpert {
            self.best_xpert_lap(time_selection)
        } else {
            self.best_standard_lap(time_selection)
        };
        match (self_best_lap.time.clone(), comparison.time.clone()) {
            (Some(self_best_time), Some(comparison_time)) => {
                if use_pax {
                    if self_best_lap == comparison {
                        "".to_string()
                    } else {
                        match (comparison.with_pax(), self_best_lap.with_pax()) {
                            (Some(comparison), Some(self_best_lap)) => {
                                format!("{:.3}", comparison - self_best_lap)
                            }
                            _ => "".to_string(),
                        }
                    }
                } else if comparison_time == self_best_time {
                    "".to_string()
                } else {
                    format!("{:.3}", comparison_time - self_best_time)
                }
            }
            (_, _) => "N/A".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::enums::short_car_class::ShortCarClass;
    use crate::models::driver::{Driver, TimeSelection};
    use crate::models::driver_from_pronto::DriverFromPronto;
    use crate::models::lap_time::{dns, dsq, LapTime, Penalty};
    use crate::models::type_aliases::{PaxMultiplier, Time};
    use rstest::rstest;
    use std::str::FromStr;

    fn build_driver(day1: Option<Vec<LapTime>>, day2: Option<Vec<LapTime>>, dsq: bool, two_day: bool) -> Driver {
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
                xpert: None,
                dsq: Some(if dsq { 1 } else { 0 }),
                region: None,
                best_run: "".to_string(),
                pax_multiplier: "0.5".to_string(),
                pax_time: "0.0".to_string(),
                runs_day1: None,
                runs_day2: None,
                day1,
                day2,
            },
            two_day,
        )
    }

    #[rstest]
    #[case(None, None)]
    #[case(None, Some(vec![LapTime::new(2.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)]))]
    #[case(Some(vec![LapTime::new(2.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)]), None)]
    #[case(
        Some(vec![LapTime::new(1.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)]),
        Some(vec![LapTime::new(2.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)])
    )]
    fn best_lap_should_return_dsq_for_dsq(#[case] d1: Option<Vec<LapTime>>, #[case] d2: Option<Vec<LapTime>>) {
        for ts in &[
            None,
            Some(TimeSelection::Day1),
            Some(TimeSelection::Day2),
            Some(TimeSelection::Combined),
        ] {
            assert_eq!(
                build_driver(d1.clone(), d2.clone(), true, false).best_lap(false, *ts),
                dsq()
            );
            assert_eq!(
                build_driver(d1.clone(), d2.clone(), true, true).best_lap(false, *ts),
                dsq()
            );
        }
    }

    #[rstest]
    #[case(None, None, None)]
    #[case(None, Some(vec![]), None)]
    #[case(None, Some(vec![LapTime::new(2.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)]), None)]
    #[case(None, None, Some(TimeSelection::Day1))]
    #[case(None, Some(vec![]), Some(TimeSelection::Day1))]
    #[case(None, Some(vec![LapTime::new(2.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)]), Some(TimeSelection::Day1))]
    #[case(None, None, Some(TimeSelection::Day2))]
    #[case(Some(vec![]), None, Some(TimeSelection::Day2))]
    #[case(Some(vec![LapTime::new(2.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)]), None, Some(TimeSelection::Day2))]
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
            build_driver(d1.clone(), d2.clone(), false, false).best_lap(false, ts),
            dns()
        );
        assert_eq!(build_driver(d1, d2, false, true).best_lap(false, ts), dns());
    }

    #[rstest]
    #[case(Some(vec![LapTime::new(2.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)]), None, Some(TimeSelection::Combined))]
    #[case(Some(vec![LapTime::new(2.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)]), Some(vec![]), Some(TimeSelection::Combined))]
    #[case(None, Some(vec![LapTime::new(2.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)]), Some(TimeSelection::Combined))]
    #[case(Some(vec![]), Some(vec![LapTime::new(2.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)]), Some(TimeSelection::Combined))]
    fn best_lap_should_return_dns_for_special_two_day_event_cases(
        #[case] d1: Option<Vec<LapTime>>,
        #[case] d2: Option<Vec<LapTime>>,
        #[case] ts: Option<TimeSelection>,
    ) {
        assert_eq!(build_driver(d1, d2, false, true).best_lap(false, ts), dns());
    }

    #[rstest]
    #[case(
        Some(vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ]),
        None,
        false,
        None,
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        Some(vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ]),
        None,
        false,
        Some(TimeSelection::Day1),
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        Some(vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ]),
        Some(vec![]),
        false,
        Some(TimeSelection::Day1),
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        Some(vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ]),
        None,
        true,
        Some(TimeSelection::Day1),
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        Some(vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ]),
        Some(vec![]),
        true,
        Some(TimeSelection::Day1),
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        Some(vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ]),
        Some(vec![LapTime::new(1.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)]),
        true,
        Some(TimeSelection::Day1),
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    fn best_lap_happy_path_day1(
        #[case] d1: Option<Vec<LapTime>>,
        #[case] d2: Option<Vec<LapTime>>,
        #[case] two_day: bool,
        #[case] ts: Option<TimeSelection>,
        #[case] expected: LapTime,
    ) {
        assert_eq!(build_driver(d1, d2, false, two_day).best_lap(false, ts), expected);
    }

    #[rstest]
    #[case(
        None,
        Some(vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ]),
        false,
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        Some(vec![]),
        Some(vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ]),
        false,
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        None,
        Some(vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ]),
        true,
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        Some(vec![]),
        Some(vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ]),
        true,
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        Some(vec![LapTime::new(1.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)]),
        Some(vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ]),
        true,
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    fn best_lap_happy_path_day2(
        #[case] d1: Option<Vec<LapTime>>,
        #[case] d2: Option<Vec<LapTime>>,
        #[case] two_day: bool,
        #[case] expected: LapTime,
    ) {
        assert_eq!(
            build_driver(d1, d2, false, two_day).best_lap(false, Some(TimeSelection::Day2)),
            expected
        );
    }

    #[test]
    fn best_lap_happy_path_combined() {
        assert_eq!(
            build_driver(
                Some(vec![
                    LapTime::new(
                        20.into(),
                        PaxMultiplier::from_str("0.5").unwrap(),
                        0,
                        Some(Penalty::DNF)
                    ),
                    LapTime::new(60.into(), PaxMultiplier::from_str("0.5").unwrap(), 2, None),
                    LapTime::new(90.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
                ]),
                Some(vec![
                    LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
                    LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
                    LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
                ]),
                false,
                true
            )
            .best_xpert_lap(Some(TimeSelection::Combined)),
            LapTime::new(66.into(), PaxMultiplier::from_str("0.5").unwrap(), 3, None)
        );
    }

    #[rstest]
    #[case(LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None), true, None, "-2.000")]
    #[case(LapTime::new(3.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None), false, None, "-7.000")]
    #[case(
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None),
        true,
        Some(TimeSelection::Day1),
        "-2.000"
    )]
    #[case(
        LapTime::new(3.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None),
        false,
        Some(TimeSelection::Day1),
        "-7.000"
    )]
    #[case(
        LapTime::new(10.into(), PaxMultiplier::from_str("0.45").unwrap(), 0, None),
        true,
        Some(TimeSelection::Day1),
        "-0.500"
    )]
    #[case(
        LapTime::new(Time::from_str("4.5").unwrap(), PaxMultiplier::from_str("0.45").unwrap(), 0, None),
        false,
        Some(TimeSelection::Day1),
        "-5.500"
    )]
    #[case(
        LapTime::new(Time::from_str("23.335").unwrap(), PaxMultiplier::from_str("0.1").unwrap(), 0, None),
        true,
        Some(TimeSelection::Day1),
        "-2.666"
    )]
    #[case(
        LapTime::new(Time::from_str("2.334").unwrap(), PaxMultiplier::from_str("0.1").unwrap(), 0, None),
        false,
        Some(TimeSelection::Day1),
        "-7.666"
    )]
    #[case(
        LapTime::new(Time::from_str("23.334").unwrap(), PaxMultiplier::from_str("0.1").unwrap(), 0, None),
        true,
        Some(TimeSelection::Day1),
        "-2.667"
    )]
    #[case(
        LapTime::new(Time::from_str("2.3334").unwrap(), PaxMultiplier::from_str("0.1").unwrap(), 0, None),
        false,
        Some(TimeSelection::Day1),
        "-7.667"
    )]
    #[case(LapTime::new(10.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None), true, Some(TimeSelection::Day1), "")]
    #[case(LapTime::new(10.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None), false, Some(TimeSelection::Day1), "")]
    #[case(
        LapTime::new(16.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None),
        true,
        Some(TimeSelection::Day2),
        "-2.000"
    )]
    #[case(
        LapTime::new(8.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None),
        false,
        Some(TimeSelection::Day2),
        "-12.000"
    )]
    #[case(
        LapTime::new(26.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None),
        true,
        Some(TimeSelection::Combined),
        "-2.000"
    )]
    #[case(
        LapTime::new(13.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None),
        false,
        Some(TimeSelection::Combined),
        "-17.000"
    )]
    fn difference_happy_path(
        #[case] fastest: LapTime,
        #[case] use_pax: bool,
        #[case] ts: Option<TimeSelection>,
        #[case] expected: &str,
    ) {
        let testable = build_driver(
            Some(vec![LapTime::new(
                10.into(),
                PaxMultiplier::from_str("0.5").unwrap(),
                0,
                None,
            )]),
            Some(vec![LapTime::new(
                20.into(),
                PaxMultiplier::from_str("0.5").unwrap(),
                0,
                None,
            )]),
            false,
            true,
        );

        let actual = testable.difference(fastest.clone(), use_pax, false, ts);
        assert_eq!(
            actual,
            expected.to_string(),
            "Expected {} - {} == {}, got {}",
            fastest.to_string(use_pax, false),
            testable.best_lap(false, ts).to_string(use_pax, false),
            expected,
            actual
        );
    }

    #[rstest]
    #[case(Time::from_str("51.861").unwrap(), PaxMultiplier::from_str("0.821").unwrap(), Time::from_str("50.154").unwrap(), PaxMultiplier::from_str("0.821").unwrap(), "-1.402")]
    #[case(Time::from_str("48.274").unwrap(), PaxMultiplier::from_str("0.819").unwrap(), Time::from_str("48.547").unwrap(), PaxMultiplier::from_str("0.813").unwrap(), "-0.067")]
    fn difference_but_barely(
        #[case] driver_time: Time,
        #[case] driver_pax: PaxMultiplier,
        #[case] fastest_time: Time,
        #[case] fastest_pax: PaxMultiplier,
        #[case] expected: &str,
    ) {
        let testable = build_driver(
            Some(vec![LapTime::new(driver_time, driver_pax, 0, None)]),
            None,
            false,
            true,
        );
        let fastest = LapTime::new(fastest_time, fastest_pax, 0, None);

        let actual = testable.difference(fastest.clone(), true, false, None);
        assert_eq!(
            actual,
            expected.to_string(),
            "Expected {} - {} == {}, got {}",
            fastest.to_string(true, false),
            testable.best_lap(false, None).to_string(true, false),
            expected,
            actual
        )
    }

    #[test]
    fn difference_no_best_lap() {
        let d1 = Some(vec![LapTime::new(
            Time::from_str("1.8").unwrap(),
            PaxMultiplier::from_str("0.5").unwrap(),
            0,
            None,
        )]);
        let d2 = Some(vec![LapTime::new(
            3.into(),
            PaxMultiplier::from_str("0.5").unwrap(),
            0,
            None,
        )]);
        let baseline = LapTime::new(1.into(), PaxMultiplier::from_str("0.8").unwrap(), 0, None);
        assert_eq!(
            build_driver(d1.clone(), d2.clone(), true, false).difference(baseline.clone(), true, false, None),
            "N/A".to_string()
        );
        assert_eq!(
            build_driver(d1.clone(), d2.clone(), true, false).difference(baseline.clone(), false, false, None),
            "N/A".to_string()
        );
        assert_eq!(
            build_driver(None, d2.clone(), false, false).difference(baseline.clone(), true, false, None),
            "N/A".to_string()
        );
        assert_eq!(
            build_driver(None, d2.clone(), false, false).difference(baseline.clone(), false, false, None),
            "N/A".to_string()
        );
        assert_eq!(
            build_driver(None, d2.clone(), false, false).difference(
                baseline.clone(),
                true,
                false,
                Some(TimeSelection::Day1)
            ),
            "N/A".to_string()
        );
        assert_eq!(
            build_driver(None, d2.clone(), false, false).difference(
                baseline.clone(),
                false,
                false,
                Some(TimeSelection::Day1)
            ),
            "N/A".to_string()
        );
        assert_eq!(
            build_driver(d1.clone(), None, false, false).difference(
                baseline.clone(),
                true,
                false,
                Some(TimeSelection::Day2)
            ),
            "N/A".to_string()
        );
        assert_eq!(
            build_driver(d1.clone(), None, false, false).difference(
                baseline.clone(),
                false,
                false,
                Some(TimeSelection::Day2)
            ),
            "N/A".to_string()
        );
        assert_eq!(
            build_driver(d1.clone(), None, false, true).difference(
                baseline.clone(),
                true,
                false,
                Some(TimeSelection::Combined)
            ),
            "N/A".to_string()
        );
        assert_eq!(
            build_driver(d1, None, false, true).difference(
                baseline.clone(),
                false,
                false,
                Some(TimeSelection::Combined)
            ),
            "N/A".to_string()
        );
        assert_eq!(
            build_driver(None, d2.clone(), false, true).difference(
                baseline.clone(),
                true,
                false,
                Some(TimeSelection::Combined)
            ),
            "N/A".to_string()
        );
        assert_eq!(
            build_driver(None, d2, false, true).difference(
                baseline.clone(),
                false,
                false,
                Some(TimeSelection::Combined)
            ),
            "N/A".to_string()
        );
    }

    #[test]
    fn sortable_one_day_event_day1() {
        let d1 = build_driver(
            Some(vec![LapTime::new(
                100.into(),
                PaxMultiplier::from_str("0.2").unwrap(),
                0,
                None,
            )]),
            None,
            false,
            false,
        );
        let d2 = build_driver(
            Some(vec![LapTime::new(
                100.into(),
                PaxMultiplier::from_str("0.3").unwrap(),
                0,
                None,
            )]),
            None,
            false,
            false,
        );
        let d3 = build_driver(
            Some(vec![LapTime::new(
                100.into(),
                PaxMultiplier::from_str("0.4").unwrap(),
                0,
                None,
            )]),
            None,
            false,
            false,
        );

        let mut actual = vec![d3.clone(), d1.clone(), d2.clone()];
        actual.sort_by_key(|driver| driver.best_lap(false, None));

        assert_eq!(actual, vec![d1, d2, d3]);
    }

    #[test]
    fn sortable_two_day_event() {
        let d1 = build_driver(
            Some(vec![LapTime::new(
                100.into(),
                PaxMultiplier::from_str("0.10").unwrap(),
                0,
                None,
            )]),
            Some(vec![LapTime::new(
                100.into(),
                PaxMultiplier::from_str("0.11").unwrap(),
                0,
                None,
            )]),
            false,
            true,
        );
        let d2 = build_driver(
            Some(vec![LapTime::new(
                100.into(),
                PaxMultiplier::from_str("0.20").unwrap(),
                0,
                None,
            )]),
            Some(vec![LapTime::new(
                100.into(),
                PaxMultiplier::from_str("0.22").unwrap(),
                0,
                None,
            )]),
            false,
            true,
        );
        let d3 = build_driver(
            Some(vec![LapTime::new(
                100.into(),
                PaxMultiplier::from_str("0.30").unwrap(),
                0,
                None,
            )]),
            Some(vec![LapTime::new(
                100.into(),
                PaxMultiplier::from_str("0.33").unwrap(),
                0,
                None,
            )]),
            false,
            true,
        );

        let mut actual = vec![d3.clone(), d1.clone(), d2.clone()];
        actual.sort_by_key(|driver| driver.best_lap(false, None));

        assert_eq!(actual, vec![d1, d2, d3]);
    }
}
