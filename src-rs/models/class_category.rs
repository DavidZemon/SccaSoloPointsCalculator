use crate::enum_str;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

enum_str! {
    enum ClassCategory {
        Street_Category,
        Street_Touring_Category,
        Street_Prepared_Category,
        Street_Modified_Category,
        Prepared_Category,
        Modified_Category,
        Classic_American_Muscle_Category,
        Xtreme_Street,
        Miscellaneous_Category,
    }
}
