use serde::Serialize;

use crate::models::car_class::CarClass;

pub trait ChampionshipDriver {
    fn id(&self) -> &String;

    fn name(&self) -> &String;

    fn points(&self) -> &Vec<i64>;

    fn total_points(&self) -> i64;

    fn event_count(&self) -> usize;

    fn add_event(&mut self, event_points: i64);

    fn best_of(&self, events_to_count: usize) -> i64 {
        let mut points = self.points().clone();
        points.sort();
        points.reverse();
        points[0..events_to_count].iter().sum()
    }
}

pub trait ClassChampionshipDriver {
    fn get_car_class(&self) -> &CarClass;
}

#[derive(Clone, Serialize)]
pub struct IndexedChampionshipDriver {
    id: String,
    name: String,
    points: Vec<i64>,
    total_points: i64,
}

#[derive(Clone, Serialize)]
pub struct ClassedChampionshipDriver {
    id: String,
    name: String,
    points: Vec<i64>,
    total_points: i64,
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
}

impl ChampionshipDriver for IndexedChampionshipDriver {
    fn id(&self) -> &String {
        &self.id
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn points(&self) -> &Vec<i64> {
        &self.points
    }

    fn total_points(&self) -> i64 {
        self.total_points
    }

    fn event_count(&self) -> usize {
        self.points.len()
    }

    fn add_event(&mut self, event_points: i64) {
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

    pub fn car_class(&self) -> &CarClass {
        &self.car_class
    }
}

impl ChampionshipDriver for ClassedChampionshipDriver {
    fn id(&self) -> &String {
        &self.id
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn points(&self) -> &Vec<i64> {
        &self.points
    }

    fn total_points(&self) -> i64 {
        self.total_points
    }

    fn event_count(&self) -> usize {
        self.points.len()
    }

    fn add_event(&mut self, event_points: i64) {
        self.points.push(event_points);
        self.total_points += event_points;
    }
}
