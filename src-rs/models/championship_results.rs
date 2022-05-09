use crate::models::championship_driver::ChampionshipDriver;
use crate::models::short_car_class::ShortCarClass;
use getset::Setters;
use std::collections::HashMap;

pub struct IndexedChampionshipResults {
    pub year: u16,
    pub organization: String,
    pub drivers: Vec<ChampionshipDriver>,
}

impl IndexedChampionshipResults {
    pub fn new(
        year: u16,
        organization: String,
        drivers: Vec<ChampionshipDriver>,
    ) -> IndexedChampionshipResults {
        IndexedChampionshipResults {
            year,
            organization,
            drivers,
        }
    }
}

pub struct ClassChampionshipResults {
    pub year: u16,
    pub organization: String,
    pub drivers_by_class: HashMap<ShortCarClass, Vec<ChampionshipDriver>>,
}

impl ClassChampionshipResults {
    pub fn new(
        year: u16,
        organization: String,
        drivers_by_class: HashMap<ShortCarClass, Vec<ChampionshipDriver>>,
    ) -> ClassChampionshipResults {
        ClassChampionshipResults {
            year,
            organization,
            drivers_by_class,
        }
    }
}

#[derive(Setters)]
pub struct ChampionshipResults {
    pub class: Option<ClassChampionshipResults>,
    pub pax: Option<IndexedChampionshipResults>,
    pub novice: Option<IndexedChampionshipResults>,
    pub ladies: Option<IndexedChampionshipResults>,
}

impl ChampionshipResults {
    pub fn new() -> ChampionshipResults {
        ChampionshipResults {
            class: None,
            pax: None,
            novice: None,
            ladies: None,
        }
    }
}
