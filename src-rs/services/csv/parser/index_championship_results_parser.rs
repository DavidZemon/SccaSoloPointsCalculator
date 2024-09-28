use std::collections::{HashMap, HashSet};

use calamine::{Data, DataType};
#[cfg(test)]
use mockall::automock;

use crate::models::championship_driver::ChampionshipDriver;
use crate::models::championship_results::IndexedChampionshipResults;
use crate::models::driver::Driver;
use crate::models::lap_time::LapTime;
use crate::models::type_aliases::DriverId;
use crate::services::calculators::championship_points_calculator::{
    ChampionshipPointsCalculator, DefaultChampionshipPointsCalculator,
};

struct CalculationContext<'a> {
    rows_by_driver_id: HashMap<DriverId, ChampionshipDriver>,
    new_event_drivers_by_id: HashMap<DriverId, &'a Driver>,
    past_event_count: usize,
}

#[cfg_attr(test, automock)]
pub trait IndexChampionshipResultsParser {
    #[allow(clippy::needless_lifetimes)]
    fn parse<'a>(
        &self,
        past_event_count: usize,
        header_map: HashMap<String, usize>,
        data: calamine::Range<Data>,
        event_drivers: HashMap<DriverId, &'a Driver>,
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
        data: calamine::Range<Data>,
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
            .split(' ')
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

impl Default for DefaultIndexChampionshipResultsParser {
    fn default() -> Self {
        Self {
            points_calculator: Box::new(DefaultChampionshipPointsCalculator {}),
        }
    }
}

impl DefaultIndexChampionshipResultsParser {
    fn parse_sheet(
        &self,
        header_map: HashMap<String, usize>,
        data: calamine::Range<Data>,
    ) -> Result<HashMap<DriverId, ChampionshipDriver>, String> {
        let name_index = *header_map
            .get("Driver")
            .ok_or_else(|| "Missing 'Driver' column".to_string())?;
        let total_points_index = *header_map
            .get("Total\nPoints")
            .ok_or_else(|| "Missing 'Total Points' column".to_string())?;

        Ok(data
            .rows()
            .filter(|r| !r[0].is_empty() && r[0].is_int())
            .map(|r| {
                let name = r[name_index].to_string();
                let mut driver = ChampionshipDriver::new(name.as_str());
                r[name_index + 1..total_points_index]
                    .iter()
                    .for_each(|cell| driver.add_event(cell.get_int().unwrap_or_default()));
                (name.to_lowercase(), driver)
            })
            .collect())
    }

    fn create_indexed_championship_driver(
        &self,
        ctx: &CalculationContext,
        best_lap_of_day: &LapTime,
        id: &DriverId,
    ) -> ChampionshipDriver {
        let driver_history_opt = ctx.rows_by_driver_id.get(id);
        let driver_new_results_opt = ctx.new_event_drivers_by_id.get(id);

        match (driver_history_opt, driver_new_results_opt) {
            (Some(driver_history), Some(driver_new_results)) => {
                let mut driver_history = driver_history.clone();
                driver_history.add_event(self.points_calculator.calculate(best_lap_of_day, driver_new_results));
                driver_history
            }
            (Some(driver_history), None) => {
                let mut driver_history = driver_history.clone();
                driver_history.add_event(0);
                driver_history
            }
            (None, Some(driver_new_results)) => {
                let mut new_driver = ChampionshipDriver::new(driver_new_results.name.as_str());
                (0..ctx.past_event_count).for_each(|_| new_driver.add_event(0));
                new_driver.add_event(self.points_calculator.calculate(best_lap_of_day, driver_new_results));
                new_driver
            }
            (None, None) => ChampionshipDriver::new("impossible"),
        }
    }
}
