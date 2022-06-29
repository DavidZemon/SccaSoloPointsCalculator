use crate::models::car_class::get_car_class;
use crate::models::championship_driver::{ChampionshipDriver, ClassedChampionshipDriver};
use crate::models::championship_results::ClassChampionshipResults;
use crate::models::driver::Driver;
use crate::models::event_results::EventResults;
use crate::models::short_car_class::ShortCarClass;
use crate::models::type_aliases::DriverId;
use crate::services::championship_points_calculator::{
    ChampionshipPointsCalculator, DefaultChampionshipPointsCalculator,
};
use calamine::{DataType, Range};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub trait ClassChampionshipResultsParser {
    fn parse(
        &self,
        data: Range<DataType>,
        event_results: &EventResults,
    ) -> Result<ClassChampionshipResults, String>;
}

pub struct DefaultClassChampionshipResultsParser {
    points_calculator: Box<dyn ChampionshipPointsCalculator>,
}

impl ClassChampionshipResultsParser for DefaultClassChampionshipResultsParser {
    fn parse(
        &self,
        data: Range<DataType>,
        event_results: &EventResults,
    ) -> Result<ClassChampionshipResults, String> {
        let org = data
            .get((0, 0))
            .expect("Empty sheet - no value at 0,0")
            .to_string()
            .trim()
            .to_string();
        let year = data
            .get((1, 0))
            .expect("Invalid sheet - no value at 1,0")
            .to_string()
            .split(" ")
            .next()
            .expect("Invalid year cell contents")
            .parse::<u16>()
            .map_err(|e| e.to_string())?;
        let past_event_count = data.width() - 4;

        Ok(ClassChampionshipResults::new(
            year,
            org,
            self.calculate_results(
                past_event_count,
                &self.parse_sheet(data),
                &self.get_new_event_drivers(event_results),
            ),
        ))
    }
}

impl DefaultClassChampionshipResultsParser {
    pub fn new() -> Self {
        Self {
            points_calculator: Box::new(DefaultChampionshipPointsCalculator {}),
        }
    }

    fn parse_sheet(
        &self,
        data: Range<DataType>,
    ) -> HashMap<ShortCarClass, HashMap<DriverId, ClassedChampionshipDriver>> {
        let mut rows_by_class_and_driver_id: HashMap<
            ShortCarClass,
            HashMap<DriverId, ClassedChampionshipDriver>,
        > = HashMap::new();

        let mut current_class: Option<ShortCarClass> = None;
        data.rows().for_each(|r| {
            if r.len() != 0 {
                let cell_str = r[0].to_string();
                let delimeter = if cell_str.contains(" - ") {
                    " - "
                } else {
                    " – "
                };
                let pieces = cell_str.split(delimeter).collect::<Vec<&str>>();
                if let Some(short_class) = ShortCarClass::parse(pieces.get(0).unwrap_or(&"")) {
                    current_class = Some(short_class);
                    rows_by_class_and_driver_id.insert(short_class, HashMap::new());
                } else {
                    match current_class {
                        None => {
                            // Skip this row, it's an early header row
                        }
                        Some(current_class) => {
                            Self::add_driver(&mut rows_by_class_and_driver_id, &current_class, r)
                        }
                    };
                }
            }
        });

        rows_by_class_and_driver_id
    }

    fn add_driver(
        rows_by_class_and_driver_id: &mut HashMap<
            ShortCarClass,
            HashMap<DriverId, ClassedChampionshipDriver>,
        >,
        current_class: &ShortCarClass,
        r: &[DataType],
    ) {
        let rows_for_one_class = rows_by_class_and_driver_id.get_mut(current_class).expect(
            format!(
                "Attempted to retrieve driver map for class {:?} but map was not found",
                current_class
            )
            .as_str(),
        );

        let id = r[1].to_string();

        let mut driver = ClassedChampionshipDriver::new(
            id.clone(),
            id.clone(),
            get_car_class(&current_class).expect(
                format!(
                    "Expected to find full CarClass struct for short class {:?} but did not",
                    current_class
                )
                .as_str(),
            ),
        );

        r[2..r.len() - 2].iter().for_each(|cell| {
            driver.add_event(cell.get_int().unwrap_or_default());
        });

        rows_for_one_class.insert(id.clone(), driver);
    }

    fn get_new_event_drivers(
        &self,
        event_results: &EventResults,
    ) -> HashMap<ShortCarClass, HashMap<DriverId, Driver>> {
        event_results
            .results
            .iter()
            .filter(|(class, _)| class != &&ShortCarClass::FUN)
            .map(|(class, results)| {
                (
                    class.clone(),
                    results
                        .drivers
                        .iter()
                        .filter(|d| !d.dsq)
                        .map(|d| (d.id.clone(), d.clone()))
                        .collect::<HashMap<DriverId, Driver>>(),
                )
            })
            .collect()
    }

    fn calculate_results(
        &self,
        past_event_count: usize,
        rows_by_class_and_driver_id: &HashMap<
            ShortCarClass,
            HashMap<DriverId, ClassedChampionshipDriver>,
        >,
        new_event_drivers_by_class_and_id: &HashMap<ShortCarClass, HashMap<DriverId, Driver>>,
    ) -> HashMap<ShortCarClass, Vec<ClassedChampionshipDriver>> {
        self.get_all_driver_ids_by_class(
            rows_by_class_and_driver_id,
            new_event_drivers_by_class_and_id,
        )
        .iter()
        .map(|(class, driver_ids)| {
            let empty_class_history: HashMap<DriverId, ClassedChampionshipDriver> = HashMap::new();
            let empty_driver_list: HashMap<DriverId, Driver> = HashMap::new();

            let class_history = rows_by_class_and_driver_id
                .get(class)
                .unwrap_or(&empty_class_history);
            let new_event_drivers_by_id = new_event_drivers_by_class_and_id
                .get(class)
                .unwrap_or(&empty_driver_list);

            let best_time_of_day = new_event_drivers_by_id
                .values()
                .map(|d| (d.best_lap(None).time, d.pax_multiplier))
                .filter(|(t, _)| t.is_some())
                .map(|(t, pax)| t.unwrap() * pax)
                .min_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap_or(Ordering::Equal))
                .expect("Apparently no drivers got a valid time all day long?!");

            (
                class.clone(),
                driver_ids
                    .iter()
                    .map(|id| {
                        let driver_history = class_history.get(id);
                        let driver_new_results = new_event_drivers_by_id.get(id);

                        if driver_history.is_some() && driver_new_results.is_some() {
                            let mut driver_history = driver_history.unwrap().clone();
                            let driver_new_results = driver_new_results.unwrap();

                            driver_history.add_event(self.points_calculator.calculate(
                                best_time_of_day,
                                driver_new_results,
                                Some(driver_new_results.pax_multiplier),
                            ) as i64);

                            driver_history
                        } else if driver_history.is_some() {
                            let mut driver_history = driver_history.unwrap().clone();
                            driver_history.add_event(0);
                            driver_history
                        } else {
                            let driver_new_results = driver_new_results.unwrap();

                            let mut new_driver = ClassedChampionshipDriver::new(
                                id.clone(),
                                driver_new_results.name.clone(),
                                get_car_class(class).unwrap(),
                            );
                            [0..past_event_count]
                                .iter()
                                .for_each(|_| new_driver.add_event(0));
                            new_driver.add_event(self.points_calculator.calculate(
                                best_time_of_day,
                                driver_new_results,
                                Some(driver_new_results.pax_multiplier),
                            ) as i64);
                            new_driver
                        }
                    })
                    .collect::<Vec<ClassedChampionshipDriver>>(),
            )
        })
        .collect()
    }

    fn get_all_driver_ids_by_class(
        &self,
        rows_by_class_and_driver_id: &HashMap<
            ShortCarClass,
            HashMap<DriverId, ClassedChampionshipDriver>,
        >,
        new_event_drivers_by_class_and_id: &HashMap<ShortCarClass, HashMap<DriverId, Driver>>,
    ) -> HashMap<ShortCarClass, HashSet<DriverId>> {
        let mut all_classes = rows_by_class_and_driver_id
            .keys()
            .cloned()
            .collect::<HashSet<ShortCarClass>>();
        all_classes.extend(new_event_drivers_by_class_and_id.keys().cloned());

        all_classes
            .iter()
            .map(|class| {
                let mut driver_ids_for_class = rows_by_class_and_driver_id
                    .get(class)
                    .map(|drivers| drivers.keys().cloned().collect::<HashSet<DriverId>>())
                    .unwrap_or(HashSet::new());

                let new_driver_ids_for_class = new_event_drivers_by_class_and_id
                    .get(class)
                    .map(|drivers| drivers.keys().cloned());
                match new_driver_ids_for_class {
                    Some(driver_ids) => driver_ids_for_class.extend(driver_ids),
                    _ => {}
                }

                (class.clone(), driver_ids_for_class)
            })
            .collect()
    }
}