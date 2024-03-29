extern crate core;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use enums::championship_type::ChampionshipType;
use enums::driver_group::DriverGroup;

use crate::models::event_results::EventResults;
use crate::services::csv::builder::event::class_results_builder::ClassResultsBuilder;
use crate::services::csv::builder::event::combined_results_builder::CombinedResultsBuilder;
use crate::services::csv::parser::championship_results_parser::ChampionshipResultsParser;
use crate::services::csv::parser::event_results_parser::parse;

pub mod enums;
mod models;
mod services;
mod utilities;

#[wasm_bindgen]
pub struct SccaSoloPointsEngine {
    event_results: EventResults,
    champ_parser: ChampionshipResultsParser,
    class_results_builder: ClassResultsBuilder,
    combined_results_builder: CombinedResultsBuilder,
}

/// Main entry point, serving as an interface for the disparate methods and functions needed by the
/// JS engine from the WASM engine
#[wasm_bindgen]
impl SccaSoloPointsEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(file_contents: String, two_day_event: bool) -> Result<SccaSoloPointsEngine, String> {
        let event_results = parse(file_contents, two_day_event)?;
        let champ_parser = ChampionshipResultsParser::new(event_results.clone());
        Ok(SccaSoloPointsEngine {
            event_results,
            champ_parser,
            class_results_builder: Default::default(),
            combined_results_builder: Default::default(),
        })
    }

    /// See [`crate::models::event_results::EventResults::js_drivers_in_error()`]
    pub fn js_drivers_in_error(&self) -> Vec<JsValue> {
        self.event_results.js_drivers_in_error()
    }

    pub fn get_header_for_event_class_results(&self) -> String {
        self.class_results_builder.get_header()
    }

    pub fn get_event_class_results_csvs(&self) -> Vec<JsValue> {
        self.class_results_builder.to_csvs(&self.event_results)
    }

    pub fn get_event_combined_csv(&self, driver_group: DriverGroup) -> Result<String, String> {
        self.combined_results_builder
            .to_combined_csv(&self.event_results, driver_group)
    }

    /// See [`crate::services::championship_results_parser::ChampionshipResultsParser::process_results()`]
    pub fn add_prior_championship_results(
        &self,
        new_results_type: ChampionshipType,
        new_results: &[u8],
        file_name: String,
    ) -> Result<String, JsValue> {
        self.champ_parser
            .process_results(new_results_type, new_results, file_name)
            .map_err(|e| JsValue::from_str(e.as_str()))
    }
}
