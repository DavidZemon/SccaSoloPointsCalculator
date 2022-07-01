use std::collections::{HashMap, HashSet};

use calamine::{DataType, Range};

use crate::models::championship_driver::{ChampionshipDriver, IndexedChampionshipDriver};
use crate::models::championship_results::IndexedChampionshipResults;
use crate::models::driver::Driver;
use crate::models::type_aliases::{DriverId, Time};
use crate::services::championship_points_calculator::{
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
        data: Range<DataType>,
        event_drivers: HashMap<DriverId, &Driver>,
        best_index_time_of_day: Time,
    ) -> Result<IndexedChampionshipResults, String>;
}

pub struct DefaultIndexChampionshipResultsParser {
    points_calculator: Box<dyn ChampionshipPointsCalculator>,
}

impl IndexChampionshipResultsParser for DefaultIndexChampionshipResultsParser {
    fn parse(
        &self,
        data: Range<DataType>,
        new_event_drivers_by_id: HashMap<DriverId, &Driver>,
        best_index_time_of_day: Time,
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
        let past_event_count = data.width() - 4;

        let rows_by_driver_id = self.parse_sheet(data);
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
                .map(|id| self.create_indexed_championship_driver(&ctx, best_index_time_of_day, id))
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

    fn parse_sheet(&self, data: Range<DataType>) -> HashMap<DriverId, IndexedChampionshipDriver> {
        data.rows()
            .filter(|r| !r[0].is_empty() && r[0].is_int())
            .map(|r| {
                let name = r[1].to_string();
                let id = name.to_lowercase();
                let mut driver = IndexedChampionshipDriver::new(id.clone(), name);
                r[2..r.len() - 2]
                    .iter()
                    .for_each(|cell| driver.add_event(cell.get_int().unwrap_or_default()));
                (id, driver)
            })
            .collect()
    }

    fn create_indexed_championship_driver(
        &self,
        ctx: &CalculationContext,
        best_time_of_day: Time,
        id: &DriverId,
    ) -> IndexedChampionshipDriver {
        let driver_history_opt = ctx.rows_by_driver_id.get(id);
        let driver_new_results_opt = ctx.new_event_drivers_by_id.get(id);

        match (driver_history_opt, driver_new_results_opt) {
            (Some(driver_history), Some(driver_new_results)) => {
                let mut driver_history = driver_history.clone();
                driver_history.add_event(self.points_calculator.calculate(
                    best_time_of_day,
                    driver_new_results,
                    Some(driver_new_results.pax_multiplier),
                ));
                driver_history
            }
            (Some(mut driver_history), None) => {
                let mut driver_history = driver_history.clone();
                driver_history.add_event(0);
                driver_history
            }
            (None, Some(driver_new_results)) => {
                let mut new_driver = IndexedChampionshipDriver::new(
                    driver_new_results.get_name(),
                    driver_new_results.get_name(),
                );
                (0..ctx.past_event_count).for_each(|_| new_driver.add_event(0));
                new_driver.add_event(self.points_calculator.calculate(
                    best_time_of_day,
                    driver_new_results,
                    Some(driver_new_results.pax_multiplier),
                ));
                new_driver
            }
            (None, None) => {
                IndexedChampionshipDriver::new("impossible".to_string(), "impossible".to_string())
            }
        }
    }
}
