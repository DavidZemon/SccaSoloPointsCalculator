use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::models::class_results::ClassResults;
use crate::models::driver::Driver;
use crate::models::driver_group::DriverGroup;
use crate::models::short_car_class::ShortCarClass;

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventResults {
    #[wasm_bindgen(skip)]
    pub results: HashMap<ShortCarClass, ClassResults>,
}

#[wasm_bindgen]
impl EventResults {
    /// Driver descriptors (string consisting of name + number + class) of any driver that we
    /// found to be in an error state during import
    pub fn js_drivers_in_error(&self) -> Vec<JsValue> {
        self.results
            .values()
            .map(|class_results| class_results.drivers.iter().filter(|d| d.error))
            .flatten()
            .map(|driver| {
                JsValue::from_str(
                    format!(
                        "{} ({} {})",
                        driver.name,
                        driver.car_number,
                        driver.car_class.short.name()
                    )
                    .as_str(),
                )
            })
            .collect()
    }

    pub fn get_all_driver_js_names(&self) -> Vec<JsValue> {
        self.results
            .values()
            .map(|class_results| {
                class_results
                    .drivers
                    .iter()
                    .map(|d| JsValue::from_str(d.name.as_str()))
            })
            .flatten()
            .collect()
    }

    pub fn len(&self) -> usize {
        self.results.len()
    }
}

impl EventResults {
    /// Get a sorted list of drivers
    pub fn get_drivers(&self, filter: Option<DriverGroup>) -> Vec<&Driver> {
        let mut drivers = self
            .results
            .values()
            .map(|r| {
                r.drivers
                    .iter()
                    .filter(|d| match filter.unwrap_or(DriverGroup::PAX) {
                        DriverGroup::Ladies => d.ladies_championship,
                        DriverGroup::Novice => d.rookie,
                        _ => true,
                    })
            })
            .flatten()
            .collect::<Vec<&Driver>>();

        drivers.sort();
        drivers
    }
}
