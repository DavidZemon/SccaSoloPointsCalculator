use std::collections::HashMap;
use std::io::Cursor;

use calamine::{DataType, Range, Reader, Xls};
use regex::Regex;
use wasm_bindgen::JsValue;

use crate::console_log;
use crate::enums::championship_type::ChampionshipType;
use crate::enums::short_car_class::ShortCarClass;
use crate::models::driver::Driver;
use crate::models::event_results::EventResults;
use crate::models::lap_time::{dns, LapTime};
use crate::models::type_aliases::DriverId;
use crate::services::csv::builder::class_csv_builder::{ClassCsvBuilder, DefaultClassCsvBuilder};
use crate::services::csv::builder::indexed_csv_builder::{
    DefaultIndexedCsvBuilder, IndexedCsvBuilder,
};
use crate::services::csv::parser::class_championship_results_parser::{
    ClassChampionshipResultsParser, DefaultClassChampionshipResultsParser,
};
use crate::services::csv::parser::index_championship_results_parser::{
    DefaultIndexChampionshipResultsParser, IndexChampionshipResultsParser,
};
use crate::utilities::log;

pub struct ChampionshipResultsParser {
    class_results_parser: Box<dyn ClassChampionshipResultsParser>,
    index_results_parser: Box<dyn IndexChampionshipResultsParser>,
    class_csv_builder: Box<dyn ClassCsvBuilder>,
    indexed_csv_builder: Box<dyn IndexedCsvBuilder>,

    event_results: EventResults,
}

impl ChampionshipResultsParser {
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
        let header_map = self
            .get_header_map(&old_data)
            .map_err(|e| JsValue::from_str(e.as_str()))?;
        let past_event_count = self
            .get_past_event_count(&header_map)
            .map_err(|e| JsValue::from_str(e.as_str()))?;

        let result = if new_results_type == ChampionshipType::Class {
            self.class_csv_builder
                .create(self.class_results_parser.parse(
                    past_event_count,
                    header_map,
                    old_data,
                    &self.event_results,
                )?)
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
            let fastest = Self::compute_fastest(&new_drivers);
            self.indexed_csv_builder.create(
                new_results_type,
                self.index_results_parser.parse(
                    past_event_count,
                    header_map,
                    old_data,
                    new_drivers,
                    &fastest,
                )?,
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

    fn get_header_map(&self, data: &Range<DataType>) -> Result<HashMap<String, usize>, String> {
        let re = Regex::new(r"^\s*best\s+\d+\s+of\s+\d+\s*$").map_err(|e| e.to_string())?;
        Ok(data
            .rows()
            .find(|row| match row.last() {
                Some(last) => re.is_match(last.to_string().to_lowercase().as_str()),
                None => false,
            })
            .ok_or("Unable to find header".to_string())?
            .iter()
            .enumerate()
            .map(|(index, header)| (header.to_string(), index))
            .collect())
    }

    fn get_past_event_count(&self, header_map: &HashMap<String, usize>) -> Result<usize, String> {
        let re = Regex::new(r"^(Trophy|Rank|Driver|Total\s+Points|Best\s+\d+\s+of\s+\d+)$")
            .map_err(|e| e.to_string())?;
        Ok(header_map
            .keys()
            .filter(|header| !re.is_match(header))
            .collect::<Vec<&String>>()
            .len())
    }

    fn compute_fastest(drivers: &HashMap<DriverId, &Driver>) -> LapTime {
        drivers
            .iter()
            .map(|(_, d)| d.best_lap(None))
            .min()
            .unwrap_or(dns())
    }
}