use getset::Getters;
use serde::Serialize;

use crate::models::car_class::CarClass;

pub trait ChampionshipDriver {
    fn id(&self) -> &String;

    fn name(&self) -> &String;

    fn points(&self) -> &Vec<i64>;

    fn total_points(&self) -> i64;

    fn event_count(&self) -> usize;

    fn add_event(&mut self, event_points: i64);
}

pub trait ClassChampionshipDriver {
    fn get_car_class(&self) -> &CarClass;
}

#[derive(Clone, Getters, Serialize)]
pub struct IndexedChampionshipDriver {
    id: String,
    name: String,
    points: Vec<i64>,
    total_points: i64,
}

#[derive(Clone, Getters, Serialize)]
pub struct ClassedChampionshipDriver {
    #[get = "pub"]
    id: String,
    #[get = "pub"]
    name: String,
    #[get = "pub"]
    points: Vec<i64>,
    #[get = "pub"]
    total_points: i64,
    #[get = "pub"]
    car_class: CarClass,
}

impl IndexedChampionshipDriver {
    pub fn new(id: String, name: String) -> IndexedChampionshipDriver {
        IndexedChampionshipDriver {
            id,
            name,
            points: Vec::new(),
            total_points: 0,
        }
    }

    pub fn add_event(&mut self, event_points: i64) {
        self.points.push(event_points);
        self.total_points += event_points;
    }
}

impl ClassedChampionshipDriver {
    pub fn new(id: String, name: String, car_class: CarClass) -> ClassedChampionshipDriver {
        ClassedChampionshipDriver {
            id,
            name,
            points: Vec::new(),
            total_points: 0,
            car_class,
        }
    }

    pub fn add_event(&mut self, event_points: i64) {
        self.points.push(event_points);
        self.total_points += event_points;

        self.id();
    }
}
