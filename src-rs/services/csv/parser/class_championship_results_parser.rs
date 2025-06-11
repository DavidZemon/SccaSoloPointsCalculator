use std::collections::{HashMap, HashSet};

use calamine::{Data, DataType};
#[cfg(test)]
use mockall::automock;

use crate::enums::short_car_class::ShortCarClass;
use crate::models::championship_driver::ChampionshipDriver;
use crate::models::championship_results::ClassChampionshipResults;
use crate::models::driver::Driver;
use crate::models::event_results::EventResults;
use crate::models::lap_time::{dns, LapTime};
use crate::models::type_aliases::DriverId;
use crate::services::calculators::championship_points_calculator::{
    ChampionshipPointsCalculator, DefaultChampionshipPointsCalculator,
};

struct CalculationContext {
    rows_by_class_and_driver_id: HashMap<ShortCarClass, HashMap<DriverId, ChampionshipDriver>>,
    new_event_drivers_by_class_and_id: HashMap<ShortCarClass, HashMap<DriverId, Driver>>,
    past_event_count: usize,
}

#[cfg_attr(test, automock)]
pub trait ClassChampionshipResultsParser {
    fn parse(
        &self,
        past_event_count: usize,
        header_map: HashMap<String, usize>,
        data: calamine::Range<Data>,
        event_results: &EventResults,
    ) -> Result<ClassChampionshipResults, String>;
}

pub struct DefaultClassChampionshipResultsParser {
    points_calculator: Box<dyn ChampionshipPointsCalculator>,
}

impl ClassChampionshipResultsParser for DefaultClassChampionshipResultsParser {
    fn parse(
        &self,
        past_event_count: usize,
        header_map: HashMap<String, usize>,
        data: calamine::Range<Data>,
        event_results: &EventResults,
    ) -> Result<ClassChampionshipResults, String> {
        let org = data
            .get((0, 0))
            .ok_or("Empty sheet - no value at 0,0 for class championship input XLS")?
            .to_string()
            .trim()
            .to_string();
        let year = data
            .get((1, 0))
            .ok_or("Invalid sheet - no value at 1,0 for class championship input XLS")?
            .to_string()
            .split(' ')
            .next()
            .ok_or("Invalid 'year' cell contents for class championship input XLS")?
            .parse::<u16>()
            .map_err(|e| e.to_string())?;

        Ok(ClassChampionshipResults::new(
            year,
            org,
            self.calculate_results(&CalculationContext {
                past_event_count,
                rows_by_class_and_driver_id: self.parse_sheet(header_map, data)?,
                new_event_drivers_by_class_and_id: self.get_new_event_drivers(event_results),
            }),
        ))
    }
}

impl Default for DefaultClassChampionshipResultsParser {
    fn default() -> Self {
        Self {
            points_calculator: Box::new(DefaultChampionshipPointsCalculator {}),
        }
    }
}

impl DefaultClassChampionshipResultsParser {
    fn parse_sheet(
        &self,
        header_map: HashMap<String, usize>,
        data: calamine::Range<Data>,
    ) -> Result<HashMap<ShortCarClass, HashMap<DriverId, ChampionshipDriver>>, String> {
        let mut rows_by_class_and_driver_id: HashMap<ShortCarClass, HashMap<DriverId, ChampionshipDriver>> =
            HashMap::new();

        let mut current_class: Option<ShortCarClass> = None;
        let name_index = *header_map
            .get("Driver")
            .ok_or_else(|| "Missing 'Driver' column".to_string())?;
        let total_points_index = *header_map
            .get("Total\nPoints")
            .ok_or_else(|| "Missing 'Total Points' column".to_string())?;
        for r in data.rows() {
            if !r.is_empty() {
                let cell_str = r[0].to_string();
                let delimeter = if cell_str.contains(" - ") { " - " } else { " – " };
                let pieces = cell_str.split(delimeter).collect::<Vec<&str>>();
                if let Some(short_class) = ShortCarClass::parse(pieces.first().unwrap_or(&"")) {
                    current_class = Some(short_class);
                    rows_by_class_and_driver_id.insert(short_class, HashMap::new());
                } else {
                    match current_class {
                        None => {
                            // Skip this row, it's an early header row
                        }
                        Some(current_class) => Self::add_driver(
                            &mut rows_by_class_and_driver_id,
                            &current_class,
                            name_index,
                            total_points_index,
                            r,
                        ),
                    };
                }
            }
        }

        Ok(rows_by_class_and_driver_id)
    }

    fn add_driver(
        rows_by_class_and_driver_id: &mut HashMap<ShortCarClass, HashMap<DriverId, ChampionshipDriver>>,
        current_class: &ShortCarClass,
        name_index: usize,
        total_points_index: usize,
        r: &[Data],
    ) {
        let rows_for_one_class = rows_by_class_and_driver_id.get_mut(current_class).unwrap_or_else(|| {
            panic!(
                "Attempted to retrieve driver map for class {:?} but map was not found",
                current_class
            )
        });
        let name = r[name_index].to_string();

        let mut driver = ChampionshipDriver::new(name.as_str());

        r[name_index + 1..total_points_index].iter().for_each(|cell| {
            driver.add_event(cell.get_int().unwrap_or_default());
        });

        rows_for_one_class.insert(name.to_lowercase(), driver);
    }

    fn get_new_event_drivers(&self, event_results: &EventResults) -> HashMap<ShortCarClass, HashMap<DriverId, Driver>> {
        event_results
            .results
            .iter()
            .filter(|(class, _)| class != &&ShortCarClass::FUN)
            .map(|(class, results)| {
                (
                    *class,
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

    fn calculate_results(&self, ctx: &CalculationContext) -> HashMap<ShortCarClass, Vec<ChampionshipDriver>> {
        self.get_all_driver_ids_by_class(&ctx.rows_by_class_and_driver_id, &ctx.new_event_drivers_by_class_and_id)
            .iter()
            .map(|(class, driver_ids)| self.calculate_results_for_class(ctx, class, driver_ids))
            .collect()
    }

    fn calculate_results_for_class(
        &self,
        ctx: &CalculationContext,
        class: &ShortCarClass,
        driver_ids: &HashSet<DriverId>,
    ) -> (ShortCarClass, Vec<ChampionshipDriver>) {
        let empty_class_history: HashMap<DriverId, ChampionshipDriver> = HashMap::new();
        let empty_driver_list: HashMap<DriverId, Driver> = HashMap::new();

        let class_history = ctx
            .rows_by_class_and_driver_id
            .get(class)
            .unwrap_or(&empty_class_history);
        let new_event_drivers_by_id = ctx
            .new_event_drivers_by_class_and_id
            .get(class)
            .unwrap_or(&empty_driver_list);

        let best_time_of_day = new_event_drivers_by_id
            .values()
            .map(|d| d.best_lap(*class == ShortCarClass::X))
            .filter(|lap| lap.time.is_some())
            .min()
            .unwrap_or_else(dns);

        (
            *class,
            driver_ids
                .iter()
                .map(|id| {
                    self.create_classed_championship_driver(
                        ctx,
                        &best_time_of_day,
                        id,
                        class_history,
                        new_event_drivers_by_id,
                        *class == ShortCarClass::X,
                    )
                })
                .collect(),
        )
    }

    fn create_classed_championship_driver(
        &self,
        ctx: &CalculationContext,
        best_time_of_day: &LapTime,
        id: &DriverId,
        class_history: &HashMap<DriverId, ChampionshipDriver>,
        new_event_drivers_by_id: &HashMap<DriverId, Driver>,
        expert: bool,
    ) -> ChampionshipDriver {
        let driver_history_opt = class_history.get(id);
        let driver_new_results_opt = new_event_drivers_by_id.get(id);

        match (driver_history_opt, driver_new_results_opt) {
            (Some(driver_history), Some(driver_new_results)) => {
                let mut driver_history = driver_history.clone();
                driver_history.add_event(self.points_calculator.calculate(
                    best_time_of_day,
                    driver_new_results,
                    expert,
                ));

                driver_history
            }
            (Some(driver_history), None) => {
                let mut driver_history = driver_history.clone();
                driver_history.add_event(0);
                driver_history
            }
            (None, Some(driver_new_results)) => {
                let mut new_driver = ChampionshipDriver::new(driver_new_results.name.as_str());
                (0..ctx.past_event_count).for_each(|_| {
                    new_driver.add_event(0);
                });
                new_driver.add_event(
                    self.points_calculator
                        .calculate(best_time_of_day, driver_new_results, expert),
                );
                new_driver
            }
            (None, None) => ChampionshipDriver::new("impossible"),
        }
    }

    fn get_all_driver_ids_by_class(
        &self,
        rows_by_class_and_driver_id: &HashMap<ShortCarClass, HashMap<DriverId, ChampionshipDriver>>,
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
                    .unwrap_or_default();

                let new_driver_ids_for_class = new_event_drivers_by_class_and_id
                    .get(class)
                    .map(|drivers| drivers.keys().cloned());
                if let Some(driver_ids) = new_driver_ids_for_class {
                    driver_ids_for_class.extend(driver_ids)
                }

                (*class, driver_ids_for_class)
            })
            .collect()
    }
}
