use std::collections::HashMap;

use calamine::{DataType, Range};

use crate::models::championship_results::IndexedChampionshipResults;
use crate::models::driver::Driver;
use crate::models::type_aliases::{DriverId, Time};
use crate::services::championship_points_calculator::{
    ChampionshipPointsCalculator, DefaultChampionshipPointsCalculator,
};

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
        event_drivers: HashMap<DriverId, &Driver>,
        best_index_time_of_day: Time,
    ) -> Result<IndexedChampionshipResults, String> {
        let previous_drivers = data.range()


        Err("Not implemented".to_string())
    }
}

impl DefaultIndexChampionshipResultsParser {
    pub fn new() -> Self {
        Self {
            points_calculator: Box::new(DefaultChampionshipPointsCalculator {}),
        }
    }
}
