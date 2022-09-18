use std::collections::{HashMap, HashSet};

use calamine::{DataType, Range};

use crate::models::championship_driver::{ChampionshipDriver, IndexedChampionshipDriver};
use crate::models::championship_results::IndexedChampionshipResults;
use crate::models::driver::Driver;
use crate::models::lap_time::LapTime;
use crate::models::type_aliases::DriverId;
use crate::services::calculators::championship_points_calculator::{
    ChampionshipPointsCalculator, DefaultChampionshipPointsCalculator,
};

struct CalculationContext<'a> {
    rows_by_driver_id: HashMap<DriverId, IndexedChampionshipDriver>,
    new_event_drivers_by_id: HashMap<DriverId, &'a Driver>,
    past_event_count: usize,
}

pub trait IndexChampionshipResultsParser {
    fn parse(
        &self,
        past_event_count: usize,
        header_map: HashMap<String, usize>,
        data: Range<DataType>,
        event_drivers: HashMap<DriverId, &Driver>,
        best_lap_of_day: &LapTime,
    ) -> Result<IndexedChampionshipResults, String>;
}

pub struct DefaultIndexChampionshipResultsParser {
    points_calculator: Box<dyn ChampionshipPointsCalculator>,
}

impl IndexChampionshipResultsParser for DefaultIndexChampionshipResultsParser {
    fn parse(
        &self,
        past_event_count: usize,
        header_map: HashMap<String, usize>,
        data: Range<DataType>,
        new_event_drivers_by_id: HashMap<DriverId, &Driver>,
        best_lap_of_day: &LapTime,
    ) -> Result<IndexedChampionshipResults, String> {
        let org = data
            .get((0, 0))
            .ok_or("Empty sheet - no value at 0,0 for indexed championship input XLS")?
            .to_string()
            .trim()
            .to_string();
        let year = data
            .get((1, 0))
            .ok_or("Invalid sheet - no value at 1,0 for indexed championship input XLS")?
            .to_string()
            .split(" ")
            .next()
            .ok_or("Invalid 'year' cell contents for indexed championship input XLS")?
            .parse::<u16>()
            .map_err(|e| e.to_string())?;

        let rows_by_driver_id = self.parse_sheet(header_map, data)?;
        let ctx = CalculationContext {
            past_event_count,
            rows_by_driver_id,
            new_event_drivers_by_id,
        };
        Ok(IndexedChampionshipResults::new(
            year,
            org,
            ctx.rows_by_driver_id
                .keys()
                .cloned()
                .chain(ctx.new_event_drivers_by_id.keys().cloned())
                .collect::<HashSet<DriverId>>()
                .iter()
                .map(|id| self.create_indexed_championship_driver(&ctx, best_lap_of_day, id))
                .collect(),
        ))
    }
}

impl DefaultIndexChampionshipResultsParser {
    pub fn new() -> Self {
        Self {
            points_calculator: Box::new(DefaultChampionshipPointsCalculator {}),
        }
    }

    fn parse_sheet(
        &self,
        header_map: HashMap<String, usize>,
        data: Range<DataType>,
    ) -> Result<HashMap<DriverId, IndexedChampionshipDriver>, String> {
        let name_index = header_map
            .get("Driver")
            .ok_or("Missing 'Driver' column".to_string())?
            .clone();
        let total_points_index = header_map
            .get("Total\nPoints")
            .ok_or("Missing 'Total Points' column".to_string())?
            .clone();

        Ok(data
            .rows()
            .filter(|r| !r[0].is_empty() && r[0].is_int())
            .map(|r| {
                let name = r[name_index].to_string();
                let id = name.to_lowercase();
                let mut driver = IndexedChampionshipDriver::new(&id, &name);
                r[name_index + 1..total_points_index]
                    .iter()
                    .for_each(|cell| driver.add_event(cell.get_int().unwrap_or_default()));
                (id, driver)
            })
            .collect())
    }

    fn create_indexed_championship_driver(
        &self,
        ctx: &CalculationContext,
        best_lap_of_day: &LapTime,
        id: &DriverId,
    ) -> IndexedChampionshipDriver {
        let driver_history_opt = ctx.rows_by_driver_id.get(id);
        let driver_new_results_opt = ctx.new_event_drivers_by_id.get(id);

        match (driver_history_opt, driver_new_results_opt) {
            (Some(driver_history), Some(driver_new_results)) => {
                let mut driver_history = driver_history.clone();
                driver_history.add_event(
                    self.points_calculator
                        .calculate(best_lap_of_day, driver_new_results),
                );
                driver_history
            }
            (Some(driver_history), None) => {
                let mut driver_history = driver_history.clone();
                driver_history.add_event(0);
                driver_history
            }
            (None, Some(driver_new_results)) => {
                let mut new_driver = IndexedChampionshipDriver::new(
                    &driver_new_results.name,
                    &driver_new_results.name,
                );
                (0..ctx.past_event_count).for_each(|_| new_driver.add_event(0));
                new_driver.add_event(
                    self.points_calculator
                        .calculate(best_lap_of_day, driver_new_results),
                );
                new_driver
            }
            (None, None) => {
                IndexedChampionshipDriver::new(&"impossible".to_string(), &"impossible".to_string())
            }
        }
    }
}
