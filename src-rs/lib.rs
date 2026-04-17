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
use crate::utilities::errors::{install_panic_hook, to_js_error_string};

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

/// Main entry point, serving as an interface for the disparate methods and
/// functions needed by the JS engine from the WASM engine.
///
/// All public methods return `Result<_, String>` so the React layer can
/// display the error in a `react-toastify` toast. Internally the Rust code
/// uses `anyhow::Result` with rich `.context(...)` chains; errors are
/// flattened to a compact single-line string here and the full debug chain
/// is also logged to the browser console via
/// [`crate::utilities::errors::log_and_stringify`].
#[wasm_bindgen]
impl SccaSoloPointsEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(msr_export: String, pronto_export: String) -> Result<SccaSoloPointsEngine, String> {
        install_panic_hook();
        let event_results = parse(msr_export, pronto_export).map_err(to_js_error_string)?;
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

    pub fn get_event_class_results_csvs(&self) -> Result<Vec<JsValue>, String> {
        self.class_results_builder
            .to_csvs(&self.event_results)
            .map_err(to_js_error_string)
    }

    pub fn get_event_combined_csv(&self, driver_group: DriverGroup) -> Result<String, String> {
        self.combined_results_builder
            .to_combined_csv(&self.event_results, driver_group)
            .map_err(to_js_error_string)
    }

    /// See [`crate::services::csv::parser::championship_results_parser::ChampionshipResultsParser::process_results()`]
    pub fn add_prior_championship_results(
        &self,
        new_results_type: ChampionshipType,
        new_results: &[u8],
        file_name: String,
    ) -> Result<String, String> {
        self.champ_parser
            .process_results(new_results_type, new_results, file_name)
            .map_err(to_js_error_string)
    }
}
