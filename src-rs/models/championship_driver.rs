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
            ChampionshipDriver::Indexed { id, .. } => id,
            ChampionshipDriver::Classed { id, .. } => id,
        }
    }

    pub fn get_name(&self) -> &String {
        match self {
            ChampionshipDriver::Indexed { name, .. } => name,
            ChampionshipDriver::Classed { name, .. } => name,
        }
    }

    pub fn get_points(&self) -> &Vec<u32> {
        match self {
            ChampionshipDriver::Indexed { points, .. } => points,
            ChampionshipDriver::Classed { points, .. } => points,
        }
    }

    pub fn get_total_points(&self) -> u32 {
        match self {
            ChampionshipDriver::Indexed { total_points, .. } => *total_points,
            ChampionshipDriver::Classed { total_points, .. } => *total_points,
        }
    }

    pub fn get_event_count(&self) -> usize {
        self.get_points().len()
    }

    pub fn add_event(&mut self, event_points: u32) {
        match self {
            ChampionshipDriver::Indexed {
                points,
                total_points,
                ..
            } => {
                points.push(event_points);
                *total_points += event_points;
            }
            ChampionshipDriver::Classed {
                points,
                total_points,
                ..
            } => {
                points.push(event_points);
                *total_points += event_points;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::car_class::CarClass;
    use crate::models::championship_driver::ChampionshipDriver;
    use crate::models::class_category::ClassCategory;
    use crate::models::long_car_class::LongCarClass;
    use crate::models::short_car_class::ShortCarClass;

    fn rand_indexed() -> ChampionshipDriver {
        let mut result = ChampionshipDriver::new_indexed(
            String::from("the indexed ID"),
            String::from("the indexed name"),
        );
        result.add_event(1);
        result.add_event(2);
        result.add_event(3);
        result
    }

    fn rand_classed() -> ChampionshipDriver {
        let mut result = ChampionshipDriver::new_classed(
            String::from("the classed ID"),
            String::from("the classed name"),
            CarClass::new(
                ShortCarClass::SS,
                LongCarClass::Super_Street,
                ClassCategory::Street_Category,
            ),
        );
        result.add_event(4);
        result.add_event(5);
        result.add_event(6);
        result.add_event(7);
        result
    }

    #[test]
    fn new_indexed_should_create_indexed() {
        match ChampionshipDriver::new_indexed(String::from("the ID"), String::from("the name")) {
            ChampionshipDriver::Classed { .. } => {
                panic!("Expected ChampionshipDriver::Indexed but got ChampionshipDriver::Classed")
            }
            _ => {}
        }
    }

    #[test]
    fn new_classed_should_create_classed() {
        match ChampionshipDriver::new_classed(
            String::from("the ID"),
            String::from("the name"),
            CarClass::new(
                ShortCarClass::SS,
                LongCarClass::Super_Street,
                ClassCategory::Street_Category,
            ),
        ) {
            ChampionshipDriver::Indexed { .. } => {
                panic!("Expected ChampionshipDriver::Classed but got ChampionshipDriver::Indexed")
            }
            _ => {}
        }
    }

    #[test]
    fn get_id() {
        assert_eq!(rand_indexed().get_id(), "the indexed ID");
        assert_eq!(rand_classed().get_id(), "the classed ID");
    }

    #[test]
    fn get_name() {
        assert_eq!(rand_indexed().get_name(), "the indexed name");
        assert_eq!(rand_classed().get_name(), "the classed name");
    }

    #[test]
    fn get_points() {
        let actual_indexed = rand_indexed().get_points().clone();
        assert_eq!(actual_indexed.len(), 3);
        assert_eq!(*(actual_indexed.get(0).unwrap()), 1);
        assert_eq!(*(actual_indexed.get(1).unwrap()), 2);
        assert_eq!(*(actual_indexed.get(2).unwrap()), 3);

        let actual_classed = rand_classed().get_points().clone();
        assert_eq!(actual_classed.len(), 4);
        assert_eq!(*(actual_classed.get(0).unwrap()), 4);
        assert_eq!(*(actual_classed.get(1).unwrap()), 5);
        assert_eq!(*(actual_classed.get(2).unwrap()), 6);
        assert_eq!(*(actual_classed.get(3).unwrap()), 7);
    }

    #[test]
    fn get_total_points() {
        assert_eq!(rand_indexed().get_total_points(), 6);
        assert_eq!(rand_classed().get_total_points(), 22);
    }

    #[test]
    fn get_event_count() {
        assert_eq!(rand_indexed().get_event_count(), 3);
        assert_eq!(rand_classed().get_event_count(), 4);
    }
}
