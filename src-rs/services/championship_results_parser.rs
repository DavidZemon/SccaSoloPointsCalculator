use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Cursor;

use calamine::{DataType, Range, Reader, Xls};
use getset::Setters;
use wasm_bindgen::prelude::*;

use crate::console_log;
use crate::models::championship_type::ChampionshipType;
use crate::models::driver::Driver;
use crate::models::event_results::EventResults;
use crate::models::short_car_class::ShortCarClass;
use crate::models::type_aliases::{DriverId, Time};
use crate::services::class_championship_results_parser::{
    ClassChampionshipResultsParser, DefaultClassChampionshipResultsParser,
};
use crate::services::csv::class_csv_builder::{ClassCsvBuilder, DefaultClassCsvBuilder};
use crate::services::csv::indexed_csv_builder::{DefaultIndexedCsvBuilder, IndexedCsvBuilder};
use crate::services::index_championship_results_parser::{
    DefaultIndexChampionshipResultsParser, IndexChampionshipResultsParser,
};
use crate::utilities::log;

#[wasm_bindgen]
#[derive(Setters)]
pub struct ChampionshipResultsParser {
    class_results_parser: Box<dyn ClassChampionshipResultsParser>,
    index_results_parser: Box<dyn IndexChampionshipResultsParser>,
    class_csv_builder: Box<dyn ClassCsvBuilder>,
    indexed_csv_builder: Box<dyn IndexedCsvBuilder>,

    event_results: EventResults,
}

#[wasm_bindgen]
impl ChampionshipResultsParser {
    #[wasm_bindgen(constructor)]
    pub fn new(event_results: EventResults) -> ChampionshipResultsParser {
        ChampionshipResultsParser {
            class_results_parser: Box::new(DefaultClassChampionshipResultsParser::new()),
            index_results_parser: Box::new(DefaultIndexChampionshipResultsParser::new()),
            class_csv_builder: Box::new(DefaultClassCsvBuilder::new()),
            indexed_csv_builder: Box::new(DefaultIndexedCsvBuilder::new()),
            event_results,
        }
    }

    pub fn process_results(
        &self,
        new_results_type: ChampionshipType,
        new_results: &[u8],
        file_name: String,
    ) -> Result<String, JsValue> {
        let event_drivers_by_id = self
            .event_results
            .get_drivers(None)
            .iter()
            .filter(|d| d.car_class.short != ShortCarClass::FUN && !d.dsq)
            .map(|d| (d.id.clone(), d.clone()))
            .collect::<HashMap<DriverId, &Driver>>();

        let old_data = self.extract_sheet(file_name, new_results)?;

        let result = if new_results_type == ChampionshipType::Class {
            self.class_csv_builder.create(
                self.class_results_parser
                    .parse(old_data, &self.event_results)?,
            )
        } else {
            let new_drivers = event_drivers_by_id
                .iter()
                .filter(|(_, d)| match new_results_type {
                    ChampionshipType::Novice => d.rookie,
                    ChampionshipType::Ladies => d.ladies_championship,
                    _ => true,
                })
                .map(|(id, d)| (id.clone(), d.clone()))
                .collect::<HashMap<DriverId, &Driver>>();
            let fastest =
                Self::compute_fastest(&new_drivers, new_results_type != ChampionshipType::Class);
            self.indexed_csv_builder.create(
                new_results_type,
                self.index_results_parser
                    .parse(old_data, new_drivers, fastest)?,
            )
        };

        result
            .map(|csv_option| {
                csv_option.unwrap_or(format!("No results for {}", new_results_type.name()))
            })
            .map_err(|e| JsValue::from_str(e.as_str()))
    }
}

impl ChampionshipResultsParser {
    fn extract_sheet(
        &self,
        file_name: String,
        new_results: &[u8],
    ) -> Result<Range<DataType>, String> {
        let new_results = Cursor::new(new_results);
        let mut workbook = Xls::new(new_results).map_err(|e| format!("{}", e).to_string())?;
        let worksheets = workbook.worksheets();
        let mut sheets = worksheets
            .iter()
            .filter(|(name, _)| name.trim().to_lowercase() != "calculations")
            .collect::<Vec<&(String, Range<DataType>)>>();
        sheets.sort_by(|(lhs_name, ..), (rhs_name, ..)| lhs_name.cmp(rhs_name));
        sheets.reverse();

        self.find_sheet(file_name, sheets.as_slice())
    }

    fn find_sheet(
        &self,
        file_name: String,
        sheets: &[&(String, Range<DataType>)],
    ) -> Result<Range<DataType>, String> {
        let (sheet_name, sheet_data) = sheets
            .get(0)
            .ok_or("Unable to find sheet with with name dissimilar to 'calculations'")?;

        if sheet_data.rows().len() >= 5 {
            console_log!("Found sheet with name {}", sheet_name);
            Ok(sheet_data.clone())
        } else if sheets.len() > 1 {
            log(format!(
                "Sheet '{}' doesn't have enough rows, checking next",
                sheet_name
            )
            .as_str());
            self.find_sheet(file_name, &sheets[1..])
        } else {
            Err(format!("File {} contains no non-empty sheets", file_name))
        }
    }

    fn compute_fastest(drivers: &HashMap<DriverId, &Driver>, use_pax: bool) -> Time {
        let mut times = drivers
            .iter()
            .map(|(_, d)| {
                d.best_lap(None).time.unwrap_or(Time::INFINITY)
                    * (if use_pax { d.pax_multiplier } else { 1. })
            })
            .collect::<Vec<Time>>();
        times.sort_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap_or(Ordering::Equal));
        times.get(0).cloned().unwrap_or(Time::INFINITY)
    }
}
