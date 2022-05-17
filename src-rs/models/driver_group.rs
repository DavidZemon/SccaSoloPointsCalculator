use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::enum_str;

enum_str! {
    enum DriverGroup {
        Ladies,
        Novice,
        PAX,
        Raw,
    }
}
