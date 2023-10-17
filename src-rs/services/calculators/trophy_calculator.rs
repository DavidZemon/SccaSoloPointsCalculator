use crate::enums::championship_type::ChampionshipType;
use std::cmp::{max, min};

pub trait TrophyCalculator {
    fn calculate(&self, driver_count: usize, championship_type: Option<ChampionshipType>) -> usize;
}

pub struct DefaultTrophyCalculator {}

impl TrophyCalculator for DefaultTrophyCalculator {
    fn calculate(
        &self,
        driver_count: usize,
        championship_type_opt: Option<ChampionshipType>,
    ) -> usize {
        match championship_type_opt {
            None => 0,
            Some(championship_type) => match championship_type {
                ChampionshipType::Class => {
                    if driver_count == 1 {
                        1
                    } else {
                        min(3, max(driver_count, 1) - 1)
                    }
                }
                ChampionshipType::PAX => min(10, driver_count),
                ChampionshipType::Novice => min(3, driver_count),
                ChampionshipType::Ladies => min(3, driver_count),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::enums::championship_type::ChampionshipType;
    use crate::services::calculators::trophy_calculator::{
        DefaultTrophyCalculator, TrophyCalculator,
    };

    #[test]
    fn class_calculate() {
        let testable = DefaultTrophyCalculator {};

        assert_eq!(testable.calculate(0, Some(ChampionshipType::Class)), 0);
        assert_eq!(testable.calculate(1, Some(ChampionshipType::Class)), 1);
        assert_eq!(testable.calculate(2, Some(ChampionshipType::Class)), 1);
        assert_eq!(testable.calculate(3, Some(ChampionshipType::Class)), 2);
        assert_eq!(testable.calculate(4, Some(ChampionshipType::Class)), 3);
        assert_eq!(testable.calculate(5, Some(ChampionshipType::Class)), 3);
        assert_eq!(testable.calculate(6, Some(ChampionshipType::Class)), 3);
        assert_eq!(testable.calculate(7, Some(ChampionshipType::Class)), 3);
    }

    #[test]
    fn pax_calculate() {
        let testable = DefaultTrophyCalculator {};

        assert_eq!(testable.calculate(0, Some(ChampionshipType::PAX)), 0);
        assert_eq!(testable.calculate(1, Some(ChampionshipType::PAX)), 1);
        assert_eq!(testable.calculate(2, Some(ChampionshipType::PAX)), 2);
        assert_eq!(testable.calculate(3, Some(ChampionshipType::PAX)), 3);
        assert_eq!(testable.calculate(4, Some(ChampionshipType::PAX)), 4);
        assert_eq!(testable.calculate(5, Some(ChampionshipType::PAX)), 5);
        assert_eq!(testable.calculate(6, Some(ChampionshipType::PAX)), 6);
        assert_eq!(testable.calculate(7, Some(ChampionshipType::PAX)), 7);
        assert_eq!(testable.calculate(8, Some(ChampionshipType::PAX)), 8);
        assert_eq!(testable.calculate(9, Some(ChampionshipType::PAX)), 9);
        assert_eq!(testable.calculate(10, Some(ChampionshipType::PAX)), 10);
        assert_eq!(testable.calculate(11, Some(ChampionshipType::PAX)), 10);
        assert_eq!(testable.calculate(12, Some(ChampionshipType::PAX)), 10);
        assert_eq!(testable.calculate(13, Some(ChampionshipType::PAX)), 10);
    }

    #[test]
    fn ladies_calculate() {
        let testable = DefaultTrophyCalculator {};

        assert_eq!(testable.calculate(0, Some(ChampionshipType::Ladies)), 0);
        assert_eq!(testable.calculate(1, Some(ChampionshipType::Ladies)), 1);
        assert_eq!(testable.calculate(2, Some(ChampionshipType::Ladies)), 2);
        assert_eq!(testable.calculate(3, Some(ChampionshipType::Ladies)), 3);
        assert_eq!(testable.calculate(4, Some(ChampionshipType::Ladies)), 3);
        assert_eq!(testable.calculate(5, Some(ChampionshipType::Ladies)), 3);
    }

    #[test]
    fn novice_calculate() {
        let testable = DefaultTrophyCalculator {};

        assert_eq!(testable.calculate(0, Some(ChampionshipType::Novice)), 0);
        assert_eq!(testable.calculate(1, Some(ChampionshipType::Novice)), 1);
        assert_eq!(testable.calculate(2, Some(ChampionshipType::Novice)), 2);
        assert_eq!(testable.calculate(3, Some(ChampionshipType::Novice)), 3);
        assert_eq!(testable.calculate(4, Some(ChampionshipType::Novice)), 3);
        assert_eq!(testable.calculate(5, Some(ChampionshipType::Novice)), 3);
    }
}
