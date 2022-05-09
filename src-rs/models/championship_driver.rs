use crate::models::car_class::CarClass;

pub enum ChampionshipDriver {
    Indexed {
        id: String,
        name: String,
        points: Vec<u32>,
        total_points: u32,
    },
    Classed {
        id: String,
        name: String,
        points: Vec<u32>,
        total_points: u32,
        car_class: CarClass,
    },
}

impl ChampionshipDriver {
    pub fn new_indexed(id: String, name: String) -> ChampionshipDriver {
        ChampionshipDriver::Indexed {
            id,
            name,
            points: Vec::new(),
            total_points: 0,
        }
    }

    pub fn new_classed(id: String, name: String, car_class: CarClass) -> ChampionshipDriver {
        ChampionshipDriver::Classed {
            id,
            name,
            points: Vec::new(),
            total_points: 0,
            car_class,
        }
    }

    pub fn get_id(&self) -> &String {
        match self {
            ChampionshipDriver::Indexed {
                id,
                name: _name,
                points: _points,
                total_points: _total_points,
            } => id,
            ChampionshipDriver::Classed {
                id,
                name: _name,
                points: _points,
                total_points: _total_points,
                car_class: _car_class,
            } => id,
        }
    }

    pub fn get_name(&self) -> &String {
        match self {
            ChampionshipDriver::Indexed {
                id: _id,
                name,
                points: _points,
                total_points: _total_points,
            } => name,
            ChampionshipDriver::Classed {
                id: _id,
                name,
                points: _points,
                total_points: _total_points,
                car_class: _car_class,
            } => name,
        }
    }

    pub fn get_points(&self) -> &Vec<u32> {
        match self {
            ChampionshipDriver::Indexed {
                id: _id,
                name: _name,
                points,
                total_points: _total_points,
            } => points,
            ChampionshipDriver::Classed {
                id: _id,
                name: _name,
                points,
                total_points: _total_points,
                car_class: _car_class,
            } => points,
        }
    }

    pub fn get_total_points(&self) -> u32 {
        match self {
            ChampionshipDriver::Indexed {
                id: _id,
                name: _name,
                points: _points,
                total_points,
            } => *total_points,
            ChampionshipDriver::Classed {
                id: _id,
                name: _name,
                points: _points,
                total_points,
                car_class: _car_class,
            } => *total_points,
        }
    }

    pub fn get_event_count(&self) -> usize {
        self.get_points().len()
    }

    pub fn add_event(&mut self, event_points: u32) {
        match self {
            ChampionshipDriver::Indexed {
                id: _id,
                name: _name,
                points,
                total_points: _total_points,
            } => {
                points.push(event_points);
            }
            ChampionshipDriver::Classed {
                id: _id,
                name: _name,
                points,
                total_points: _total_points,
                car_class: _car_class,
            } => {
                points.push(event_points);
            }
        }
    }
}
