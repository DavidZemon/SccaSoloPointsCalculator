use std::collections::HashMap;

use serde::Serialize;

use crate::enums::short_car_class::ShortCarClass;
use crate::models::championship_driver::{ClassedChampionshipDriver, IndexedChampionshipDriver};

#[derive(Serialize)]
pub struct IndexedChampionshipResults {
    pub year: u16,
    pub organization: String,
    pub drivers: Vec<IndexedChampionshipDriver>,
}

impl IndexedChampionshipResults {
    pub fn new(
        year: u16,
        organization: String,
        drivers: Vec<IndexedChampionshipDriver>,
    ) -> IndexedChampionshipResults {
        IndexedChampionshipResults {
            year,
            organization,
            drivers,
        }
    }
}

#[derive(Serialize)]
pub struct ClassChampionshipResults {
    pub year: u16,
    pub organization: String,
    pub drivers_by_class: HashMap<ShortCarClass, Vec<ClassedChampionshipDriver>>,
}

impl ClassChampionshipResults {
    pub fn new(
        year: u16,
        organization: String,
        drivers_by_class: HashMap<ShortCarClass, Vec<ClassedChampionshipDriver>>,
    ) -> ClassChampionshipResults {
        ClassChampionshipResults {
            year,
            organization,
            drivers_by_class,
        }
    }
}
