use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Cursor;

use crate::models::car_class::get_car_class;
use crate::models::championship_driver::{ChampionshipDriver, ClassedChampionshipDriver};
use calamine::{DataType, Range, Reader, Xls};
use getset::Setters;
use wasm_bindgen::prelude::*;

use crate::models::championship_results::{ClassChampionshipResults, IndexedChampionshipResults};
use crate::models::championship_type::ChampionshipType;
use crate::models::driver::Driver;
use crate::models::event_results::EventResults;
use crate::models::long_car_class::to_display_name;
use crate::models::short_car_class::ShortCarClass;
use crate::models::type_aliases::{DriverId, Time};
use crate::services::class_championship_results_parser::{
    ClassChampionshipResultsParser, DefaultClassChampionshipResultsParser,
};
use crate::services::index_championship_results_parser::{
    DefaultIndexChampionshipResultsParser, IndexChampionshipResultsParser,
};
use crate::utilities::events_to_count;

#[wasm_bindgen]
#[derive(Setters)]
pub struct ChampionshipResultsParser {
    class_results_parser: Box<dyn ClassChampionshipResultsParser>,
    index_results_parser: Box<dyn IndexChampionshipResultsParser>,

    event_results: EventResults,
    class: Option<ClassChampionshipResults>,
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
            event_results,
            class: None,
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
    ) -> Result<(), String> {
        let event_drivers_by_id = self
            .event_results
            .get_drivers(None)
            .iter()
            .filter(|d| d.car_class.short != ShortCarClass::FUN && !d.dsq)
            .map(|d| (d.id.clone(), d.clone()))
            .collect::<HashMap<DriverId, &Driver>>();

        let data = self.extract_sheet(file_name, new_results)?;
        match new_results_type {
            ChampionshipType::Class => {
                self.class = Some(self.class_results_parser.parse(data, &self.event_results)?);
            }
            ChampionshipType::PAX => {
                let fastest = Self::compute_fastest(&event_drivers_by_id);
                self.pax = Some(self.index_results_parser.parse(
                    data,
                    event_drivers_by_id,
                    fastest,
                )?);
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
            }
            ChampionshipType::Ladies => {
                let new_ladies = event_drivers_by_id
                    .iter()
                    .filter(|(_, d)| d.ladies_championship)
                    .map(|(id, d)| (id.clone(), d.clone()))
                    .collect::<HashMap<DriverId, &Driver>>();
                let fastest = Self::compute_fastest(&new_ladies);
                self.ladies = Some(self.index_results_parser.parse(data, new_ladies, fastest)?);
            }
        };
        Ok(())
    }

    pub fn get(&self, champ_type: ChampionshipType) -> Result<Option<String>, String> {
        match champ_type {
            ChampionshipType::Class => match &self.class {
                None => Ok(None),
                Some(class) => {
                    let event_count = class
                        .drivers_by_class
                        .values()
                        .next()
                        .expect("Expected at least one class")
                        .get(0)
                        .expect("Expected at least one driver in at least one class")
                        .event_count();
                    let header = Self::build_initial(event_count);

                    let mut results = vec![
                        class.organization.clone(),
                        format!(
                            "{} Class Championship -- Best {} of {} Events",
                            class.year,
                            events_to_count(event_count),
                            event_count
                        ),
                        "".to_string(),
                        header,
                    ];

                    let mut sorted = class
                        .drivers_by_class
                        .iter()
                        .map(|(k, v)| {
                            let mut v = v.clone();
                            v.sort_by(|lhs, rhs| lhs.total_points().cmp(&rhs.total_points()));
                            (k.clone(), v)
                        })
                        .collect::<Vec<(ShortCarClass, Vec<ClassedChampionshipDriver>)>>();
                    sorted.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));

                    sorted.iter().for_each(|(class, drivers)| {
                        results.push(format!(
                            "{} - {}",
                            class.name(),
                            to_display_name(get_car_class(class).unwrap().long)
                        ));

                        results.extend(drivers.iter().enumerate().map(|(index, d)| {
                            let mut driver_row = vec![format!("{}", index + 1), d.name().clone()];
                            d.points()
                                .iter()
                                .for_each(|points| driver_row.push(format!("{}", points)));
                            driver_row.push(format!("{}", d.total_points()));
                            driver_row.join(",")
                        }))
                    });

                    Ok(Some(results.join("\n")))
                }
            },
            _ => Ok(None),
        }
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

    fn build_initial(event_count: usize) -> String {
        let mut header = vec!["Rank".to_string(), "Driver".to_string()];
        header.extend(
            [0..event_count]
                .iter()
                .enumerate()
                .map(|(i, _)| format!("#{}", i + 1)),
        );
        header.push("Points".to_string());
        header.push(format!(
            "Best {} of {}",
            events_to_count(event_count),
            event_count
        ));

        header.join(",")
    }
}
