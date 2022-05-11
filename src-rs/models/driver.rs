use crate::models::car_class::{get_car_class, CarClass};
use crate::models::exported_driver::ExportedDriver;
use crate::models::lap_time::LapTime;
use crate::models::type_aliases::{DriverId, Time};
use std::cmp::Ordering;

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
}

impl Driver {
    pub fn from(driver: ExportedDriver) -> Driver {
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

        Driver {
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
            dsq: false,
            pax_multiplier: driver.pax_multiplier,
            day_1_times: day_1_times.clone(),
            day_2_times: day_2_times.clone(),
            combined: compute_best_lap(
                false,
                &day_1_times.clone().as_ref(),
                &day_2_times.clone().as_ref(),
                Some(TimeSelection::Combined),
            ),
        }
    }

    pub fn best_lap(&self, time_selection: Option<TimeSelection>) -> LapTime {
        compute_best_lap(
            self.dsq,
            &self.day_1_times.as_ref(),
            &self.day_2_times.as_ref(),
            time_selection,
        )
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

/// Find the best lap for the given time selection
/// NOTE: Times MUST already be sorted with the fastest lap at the beginning of each vector
fn compute_best_lap(
    dsq: bool,
    day_1_times: &Option<&Vec<LapTime>>,
    day_2_times: &Option<&Vec<LapTime>>,
    time_selection: Option<TimeSelection>,
) -> LapTime {
    if dsq {
        LapTime::dsq()
    } else {
        match time_selection.unwrap_or(TimeSelection::Day1) {
            TimeSelection::Day1 => day_1_times
                .map_or(None, |times| times.get(0))
                .unwrap_or(&LapTime::dns())
                .clone(),
            TimeSelection::Day2 => day_2_times
                .map_or(None, |times| times.get(0))
                .unwrap_or(&LapTime::dns())
                .clone(),
            TimeSelection::Combined => {
                if day_1_times.map(|times| !times.is_empty()).unwrap_or(false)
                    && day_2_times.map(|times| !times.is_empty()).unwrap_or(false)
                {
                    compute_best_lap(dsq, day_1_times, day_2_times, Some(TimeSelection::Day1)).add(
                        compute_best_lap(dsq, day_1_times, day_2_times, Some(TimeSelection::Day2)),
                    )
                } else {
                    LapTime::dns()
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::models::driver::{compute_best_lap, TimeSelection};
    use crate::models::lap_time::LapTime;

    #[test]
    fn compute_best_lap_should_return_dsq_for_dsq() {
        assert_eq!(compute_best_lap(true, &None, &None, None), LapTime::dsq());
    }

    #[test]
    fn compute_best_lap_should_return_dns_if_no_times() {
        let no_times: Vec<LapTime> = Vec::new();
        let with_times = vec![LapTime::new(1., 0, None)];

        assert_eq!(compute_best_lap(false, &None, &None, None), LapTime::dns());

        assert_eq!(
            compute_best_lap(false, &None, &None, Some(TimeSelection::Day1)),
            LapTime::dns()
        );
        assert_eq!(
            compute_best_lap(false, &Some(&no_times), &None, Some(TimeSelection::Day1)),
            LapTime::dns()
        );
        assert_eq!(
            compute_best_lap(false, &None, &Some(&with_times), Some(TimeSelection::Day1)),
            LapTime::dns()
        );
        assert_eq!(
            compute_best_lap(
                false,
                &Some(&no_times),
                &Some(&with_times),
                Some(TimeSelection::Day1)
            ),
            LapTime::dns()
        );

        assert_eq!(
            compute_best_lap(false, &None, &None, Some(TimeSelection::Day2)),
            LapTime::dns()
        );
        assert_eq!(
            compute_best_lap(false, &None, &Some(&no_times), Some(TimeSelection::Day2)),
            LapTime::dns()
        );
        assert_eq!(
            compute_best_lap(false, &Some(&with_times), &None, Some(TimeSelection::Day2)),
            LapTime::dns()
        );
        assert_eq!(
            compute_best_lap(
                false,
                &Some(&with_times),
                &Some(&no_times),
                Some(TimeSelection::Day2)
            ),
            LapTime::dns()
        );

        assert_eq!(
            compute_best_lap(false, &None, &None, Some(TimeSelection::Combined)),
            LapTime::dns()
        );
        assert_eq!(
            compute_best_lap(
                false,
                &Some(&no_times),
                &None,
                Some(TimeSelection::Combined)
            ),
            LapTime::dns()
        );
        assert_eq!(
            compute_best_lap(
                false,
                &None,
                &Some(&no_times),
                Some(TimeSelection::Combined)
            ),
            LapTime::dns()
        );
        assert_eq!(
            compute_best_lap(
                false,
                &Some(&no_times),
                &Some(&no_times),
                Some(TimeSelection::Combined)
            ),
            LapTime::dns()
        );
        assert_eq!(
            compute_best_lap(
                false,
                &Some(&with_times),
                &Some(&no_times),
                Some(TimeSelection::Combined)
            ),
            LapTime::dns()
        );
        assert_eq!(
            compute_best_lap(
                false,
                &Some(&no_times),
                &Some(&with_times),
                Some(TimeSelection::Combined)
            ),
            LapTime::dns()
        );
    }

    #[test]
    fn compute_best_lap_happy_path() {
        // Intentionally use not the fastest lap as the expected as a showcase and reminder that
        // this function doesn't actually compute the best lap
        let expected1 = LapTime::new(3., 2, None);
        let expected2 = LapTime::new(20., 1, None);
        let expected_combined = LapTime::new(23., 3, None);

        let times1 = vec![
            expected1.clone(),
            LapTime::new(1., 0, None),
            LapTime::new(2., 0, None),
        ];
        let times2 = vec![
            expected2.clone(),
            LapTime::new(10., 0, None),
            LapTime::new(30., 0, None),
        ];

        assert_eq!(
            compute_best_lap(false, &Some(&times1), &None, None),
            expected1.clone()
        );
        assert_eq!(
            compute_best_lap(false, &Some(&times1), &Some(&times2), None),
            expected1.clone()
        );
        assert_eq!(
            compute_best_lap(
                false,
                &Some(&times1),
                &Some(&times2),
                Some(TimeSelection::Day1)
            ),
            expected1.clone()
        );

        assert_eq!(
            compute_best_lap(false, &None, &Some(&times2), Some(TimeSelection::Day2)),
            expected2.clone()
        );
        assert_eq!(
            compute_best_lap(
                false,
                &Some(&times1),
                &Some(&times2),
                Some(TimeSelection::Day2)
            ),
            expected2.clone()
        );

        assert_eq!(
            compute_best_lap(
                false,
                &Some(&times1),
                &Some(&times2),
                Some(TimeSelection::Combined)
            ),
            expected_combined.clone()
        );
    }
}
