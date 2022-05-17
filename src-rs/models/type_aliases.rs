use wasm_bindgen::prelude::*;

pub type Time = f64;
pub type PaxMultiplier = f64;
#[wasm_bindgen(typescript_custom_section)]
const DRIVER_ID_TYPE: &'static str = r#"export type DriverId = string;"#;
pub type DriverId = String;
