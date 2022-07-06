use wasm_bindgen::prelude::*;

use crate::models::championship_type::ChampionshipType;
use crate::models::driver_group::DriverGroup;
use crate::models::event_results::EventResults;
use crate::services::championship_results_parser::ChampionshipResultsParser;
use crate::services::class_results_builder::ClassResultsBuilder;
use crate::services::combined_results_builder::CombinedResultsBuilder;
use crate::services::event_results_parser::parse;

#[wasm_bindgen]
pub struct Rusty {
    event_results: EventResults,
    champ_parser: ChampionshipResultsParser,
    class_results_builder: ClassResultsBuilder,
    combined_results_builder: CombinedResultsBuilder,
}

#[wasm_bindgen]
impl Rusty {
    #[wasm_bindgen(constructor)]
    pub fn new(file_contents: String, two_day_event: bool) -> Result<Rusty, String> {
        let event_results = parse(file_contents, two_day_event)?;
        let champ_parser = ChampionshipResultsParser::new(event_results.clone());
        Ok(Rusty {
            event_results,
            champ_parser,
            class_results_builder: ClassResultsBuilder::new(),
            combined_results_builder: CombinedResultsBuilder::new(),
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
    }
}
