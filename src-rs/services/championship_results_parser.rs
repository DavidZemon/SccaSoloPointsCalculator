use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Cursor;

use calamine::{DataType, Range, Reader, Xls};
use getset::Setters;
use wasm_bindgen::prelude::*;

use crate::models::championship_results::IndexedChampionshipResults;
use crate::models::championship_type::ChampionshipType;
use crate::models::driver::Driver;
use crate::models::event_results::EventResults;
use crate::models::short_car_class::ShortCarClass;
use crate::models::type_aliases::{DriverId, Time};
use crate::services::class_championship_results_parser::{
    ClassChampionshipResultsParser, DefaultClassChampionshipResultsParser,
};
use crate::services::csv::class_csv_builder::{ClassCsvBuilder, DefaultClassCsvBuilder};
use crate::services::index_championship_results_parser::{
    DefaultIndexChampionshipResultsParser, IndexChampionshipResultsParser,
};

#[wasm_bindgen]
#[derive(Setters)]
pub struct ChampionshipResultsParser {
    class_results_parser: Box<dyn ClassChampionshipResultsParser>,
    index_results_parser: Box<dyn IndexChampionshipResultsParser>,
    class_csv_builder: Box<dyn ClassCsvBuilder>,

    event_results: EventResults,
    pax: Option<IndexedChampionshipResults>,
    novice: Option<IndexedChampionshipResults>,
    ladies: Option<IndexedChampionshipResults>,
}

#[wasm_bindgen]
impl ChampionshipResultsParser {
    #[wasm_bindgen(constructor)]
    pub fn new(event_results: EventResults) -> ChampionshipResultsParser {
        ChampionshipResultsParser {
            class_results_parser: Box::new(DefaultClassChampionshipResultsParser::new()),
            index_results_parser: Box::new(DefaultIndexChampionshipResultsParser::new()),
            class_csv_builder: Box::new(DefaultClassCsvBuilder::new()),
            event_results,
            pax: None,
            novice: None,
            ladies: None,
        }
    }

    pub fn process_results(
        &mut self,
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

        let data = self.extract_sheet(file_name, new_results)?;
        let csv = match new_results_type {
            ChampionshipType::Class => self
                .class_csv_builder
                .create(self.class_results_parser.parse(data, &self.event_results)?),
            ChampionshipType::PAX => {
                let fastest = Self::compute_fastest(&event_drivers_by_id);
                self.pax = Some(self.index_results_parser.parse(
                    data,
                    event_drivers_by_id,
                    fastest,
                )?);
                Ok(None)
            }
            ChampionshipType::Novice => {
                let new_novices = event_drivers_by_id
                    .iter()
                    .filter(|(_, d)| d.rookie)
                    .map(|(id, d)| (id.clone(), d.clone()))
                    .collect::<HashMap<DriverId, &Driver>>();
                let fastest = Self::compute_fastest(&new_novices);
                self.novice = Some(
                    self.index_results_parser
                        .parse(data, new_novices, fastest)?,
                );
                Ok(None)
            }
            ChampionshipType::Ladies => {
                let new_ladies = event_drivers_by_id
                    .iter()
                    .filter(|(_, d)| d.ladies_championship)
                    .map(|(id, d)| (id.clone(), d.clone()))
                    .collect::<HashMap<DriverId, &Driver>>();
                let fastest = Self::compute_fastest(&new_ladies);
                self.ladies = Some(self.index_results_parser.parse(data, new_ladies, fastest)?);
                Ok(None)
            }
        };
        csv.map(|results| match results {
            Some(results) => results,
            None => format!("No results for {}", new_results_type.name()),
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
        sheets.reverse();

        self.find_sheet(file_name, sheets.as_slice())
    }

    fn find_sheet(
        &self,
        file_name: String,
        sheets: &[&(String, Range<DataType>)],
    ) -> Result<Range<DataType>, String> {
        let (.., sheet_data) = sheets
            .get(0)
            .ok_or("Unable to find sheet with with name dissimilar to 'calculations'")?;

        if sheet_data.rows().len() >= 5 {
            Ok(sheet_data.clone())
        } else if sheets.len() > 1 {
            self.find_sheet(file_name, &sheets[1..])
        } else {
            Err(format!("File {} contains no non-empty sheets", file_name))
        }
    }

    fn compute_fastest(drivers: &HashMap<DriverId, &Driver>) -> Time {
        let mut times = drivers
            .iter()
            .map(|(_, d)| d.best_lap(None).time.unwrap_or(Time::INFINITY))
            .collect::<Vec<Time>>();
        times.sort_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap_or(Ordering::Equal));
        times.get(0).cloned().unwrap_or(Time::INFINITY)
    }
}
