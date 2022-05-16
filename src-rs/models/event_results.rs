use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::models::class_results::ClassResults;
use crate::models::short_car_class::ShortCarClass;

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventResults {
    #[wasm_bindgen(skip)]
    pub results: HashMap<ShortCarClass, ClassResults>,
}

#[wasm_bindgen]
impl EventResults {
    pub fn get(&self, car_class: ShortCarClass) -> Option<ClassResults> {
        self.results.get(&car_class).map(|r| r.clone())
    }

    pub fn contains_key(&self, car_class: ShortCarClass) -> bool {
        self.results.contains_key(&car_class)
    }

    pub fn values(&self) -> Result<Vec<JsValue>, String> {
        Ok(self
            .results
            .iter()
            .map(|(_, v)| serde_wasm_bindgen::to_value(v).unwrap())
            .collect::<Vec<JsValue>>())
    }

    pub fn len(&self) -> usize {
        self.results.len()
    }
}
