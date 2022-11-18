#[derive(Debug, Clone)]
pub struct ChampionshipDriver {
    name: String,
    points: Vec<i64>,
    total_points: i64,
}

impl ChampionshipDriver {
    pub fn new(name: &str) -> ChampionshipDriver {
        ChampionshipDriver {
            name: name.to_string(),
            points: Vec::new(),
            total_points: 0,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn points(&self) -> &Vec<i64> {
        &self.points
    }

    pub fn total_points(&self) -> i64 {
        self.total_points
    }

    pub fn event_count(&self, include_zeroes: bool) -> usize {
        if include_zeroes {
            self.points.len()
        } else {
            self.points.iter().filter(|p| **p != 0).count()
        }
    }

    pub fn add_event(&mut self, event_points: i64) {
        self.points.push(event_points);
        self.total_points += event_points;
    }

    pub fn best_of(&self, events_to_count: usize) -> i64 {
        let mut points = self.points().clone();
        points.sort();
        points.reverse();
        points[0..events_to_count].iter().sum()
    }
}
