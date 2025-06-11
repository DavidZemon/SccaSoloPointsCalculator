use crate::models::car_class::{get_car_class, CarClass};
use crate::models::driver_from_pronto::DriverFromPronto;
use crate::models::lap_time::{dns, dsq, LapTime};
use crate::models::type_aliases::{DriverId, PaxMultiplier};
use std::cmp::min;
use std::str::FromStr;

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
    pub expert: bool,
    pub position: Option<usize>,
    pub dsq: bool,
    pub pax_multiplier: PaxMultiplier,
    pub times: Vec<LapTime>,
}

impl From<DriverFromPronto> for Driver {
    fn from(driver: DriverFromPronto) -> Self {
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

        Driver {
            error: driver.runs.is_empty() && !best_run_is_falsy,
            rookie: driver.rookie.map_or(false, |value| value != 0),
            ladies_championship: driver.ladies.map_or(false, |value| value != "0" && !value.is_empty()),
            expert: driver.expert.map_or(false, |value| value != 0),
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
            times: driver.runs,
        }
    }
}

impl Driver {
    pub fn set_position(&mut self, position: usize) {
        self.position = Some(position);
    }

    pub fn best_standard_lap(&self) -> LapTime {
        self.best_lap_in_range(false)
    }

    pub fn best_expert_lap(&self) -> LapTime {
        self.best_lap_in_range(true)
    }

    pub fn best_lap(&self, expert: bool) -> LapTime {
        if expert {
            self.best_expert_lap()
        } else {
            self.best_standard_lap()
        }
    }

    fn best_lap_in_range(&self, best_of_three: bool) -> LapTime {
        if self.dsq {
            dsq()
        } else {
            let mut times = Self::lap_times(&self.times, best_of_three);
            times.sort();
            times.first().cloned().unwrap_or_else(dns)
        }
    }

    fn lap_times(times: &[LapTime], best_of_three: bool) -> Vec<LapTime> {
        #[cfg(not(test))]
        crate::console_log!("Giving lap times out now: {times:?}");
        if best_of_three {
            times[..min(times.len(), 3)].to_vec()
        } else {
            times.to_vec()
        }
    }

    pub fn difference(&self, comparison: LapTime, use_pax: bool, use_xpert: bool) -> String {
        let self_best_lap = if use_xpert {
            self.best_expert_lap()
        } else {
            self.best_standard_lap()
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
    use crate::models::driver::Driver;
    use crate::models::driver_from_pronto::DriverFromPronto;
    use crate::models::lap_time::{dns, dsq, LapTime, Penalty};
    use crate::models::type_aliases::{PaxMultiplier, Time};
    use rstest::rstest;
    use std::str::FromStr;

    fn build_driver(runs: Vec<LapTime>, dsq: bool) -> Driver {
        Driver::from(DriverFromPronto {
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
            expert: None,
            dsq: Some(if dsq { 1 } else { 0 }),
            region: None,
            best_run: "".to_string(),
            pax_multiplier: "0.5".to_string(),
            pax_time: "0.0".to_string(),
            runs,
        })
    }

    #[rstest]
    #[case(vec![])]
    #[case(vec![LapTime::new(2.into(), PaxMultiplier::from_str("0.9").unwrap(), 0, None)])]
    fn best_lap_should_return_dsq_for_dsq(#[case] times: Vec<LapTime>) {
        assert_eq!(build_driver(times, true).best_lap(false), dsq());
    }

    #[test]
    fn best_lap_should_return_dns_for_missing_times() {
        assert_eq!(build_driver(vec![], false).best_lap(false), dns());
    }

    #[rstest]
    #[case(
        vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ],
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ],
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ],
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ],
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ],
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    #[case(
        vec![
            LapTime::new(2.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, Some(Penalty::DNF)),
            LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
            LapTime::new(9.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None)
        ],
        LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 1, None),
    )]
    fn best_lap_happy_path_day1(#[case] times: Vec<LapTime>, #[case] expected: LapTime) {
        assert_eq!(build_driver(times, false).best_lap(false), expected);
    }

    #[rstest]
    #[case(LapTime::new(6.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None), true, "-2.000")]
    #[case(LapTime::new(3.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None), false, "-7.000")]
    #[case(LapTime::new(10.into(), PaxMultiplier::from_str("0.45").unwrap(), 0, None), true, "-0.500")]
    #[case(LapTime::new(Time::from_str("4.5").unwrap(), PaxMultiplier::from_str("0.45").unwrap(), 0, None), false, "-5.500")]
    #[case(LapTime::new(Time::from_str("23.335").unwrap(), PaxMultiplier::from_str("0.1").unwrap(), 0, None), true, "-2.666")]
    #[case(LapTime::new(Time::from_str("2.334").unwrap(), PaxMultiplier::from_str("0.1").unwrap(), 0, None), false, "-7.666")]
    #[case(LapTime::new(Time::from_str("2.3334").unwrap(), PaxMultiplier::from_str("0.1").unwrap(), 0, None), false, "-7.667")]
    #[case(LapTime::new(10.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None), true,  "")]
    #[case(LapTime::new(10.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None), false, "")]
    #[case(LapTime::new(16.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None), true, "3.000")] /**/
    #[case(LapTime::new(8.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None), false, "-2.000")]
    #[case(LapTime::new(26.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None), true, "8.000")]
    #[case(LapTime::new(13.into(), PaxMultiplier::from_str("0.5").unwrap(), 0, None), false, "3.000")]
    fn difference_happy_path(#[case] fastest: LapTime, #[case] use_pax: bool, #[case] expected: &str) {
        let testable = build_driver(
            vec![LapTime::new(
                10.into(),
                PaxMultiplier::from_str("0.5").unwrap(),
                0,
                None,
            )],
            false,
        );

        let actual = testable.difference(fastest.clone(), use_pax, false);
        assert_eq!(
            actual,
            expected.to_string(),
            "Expected {} - {} == {}, got {}",
            fastest.to_string(use_pax, false),
            testable.best_lap(false).to_string(use_pax, false),
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
        let testable = build_driver(vec![LapTime::new(driver_time, driver_pax, 0, None)], false);
        let fastest = LapTime::new(fastest_time, fastest_pax, 0, None);

        let actual = testable.difference(fastest.clone(), true, false);
        assert_eq!(
            actual,
            expected.to_string(),
            "Expected {} - {} == {}, got {}",
            fastest.to_string(true, false),
            testable.best_lap(false).to_string(true, false),
            expected,
            actual
        )
    }

    #[test]
    fn difference_no_best_lap() {
        let times = vec![LapTime::new(
            Time::from_str("1.8").unwrap(),
            PaxMultiplier::from_str("0.5").unwrap(),
            0,
            Some(Penalty::DNS),
        )];
        let baseline = LapTime::new(1.into(), PaxMultiplier::from_str("0.8").unwrap(), 0, None);
        assert_eq!(
            build_driver(times.clone(), false).difference(baseline.clone(), true, false),
            "N/A".to_string()
        );
        assert_eq!(
            build_driver(times.clone(), false).difference(baseline.clone(), false, false),
            "N/A".to_string()
        );
    }

    #[test]
    fn is_sortable() {
        let d1 = build_driver(
            vec![LapTime::new(
                100.into(),
                PaxMultiplier::from_str("0.2").unwrap(),
                0,
                None,
            )],
            false,
        );
        let d2 = build_driver(
            vec![LapTime::new(
                100.into(),
                PaxMultiplier::from_str("0.3").unwrap(),
                0,
                None,
            )],
            false,
        );
        let d3 = build_driver(
            vec![LapTime::new(
                100.into(),
                PaxMultiplier::from_str("0.4").unwrap(),
                0,
                None,
            )],
            false,
        );

        let mut actual = vec![d3.clone(), d1.clone(), d2.clone()];
        actual.sort_by_key(|driver| driver.best_lap(false));

        assert_eq!(actual, vec![d1, d2, d3]);
    }
}
