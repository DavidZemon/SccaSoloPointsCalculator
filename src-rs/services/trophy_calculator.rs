use crate::models::driver::Driver;
use std::cmp::{max, min};

pub trait TrophyCalculator {
    fn calculate(&self, driver_count: usize) -> usize;

    fn calculate_vec(&self, drivers: Vec<&Driver>) -> usize;
}

pub struct ClassTrophyCalculator {}

impl TrophyCalculator for ClassTrophyCalculator {
    fn calculate(&self, driver_count: usize) -> usize {
        min(3, max(driver_count, 1) - 1)
    }

    fn calculate_vec(&self, drivers: Vec<&Driver>) -> usize {
        self.calculate(drivers.len())
    }
}

pub struct IndexTrophyCalculator {}

impl TrophyCalculator for IndexTrophyCalculator {
    fn calculate(&self, driver_count: usize) -> usize {
        min(10, driver_count)
    }

    fn calculate_vec(&self, drivers: Vec<&Driver>) -> usize {
        self.calculate(drivers.len())
    }
}

#[cfg(test)]
mod test {
    use crate::services::trophy_calculator::{
        ClassTrophyCalculator, IndexTrophyCalculator, TrophyCalculator,
    };

    #[test]
    fn class_calculate() {
        let testable = ClassTrophyCalculator {};

        assert_eq!(testable.calculate(0), 0);
        assert_eq!(testable.calculate(1), 0);
        assert_eq!(testable.calculate(2), 1);
        assert_eq!(testable.calculate(3), 2);
        assert_eq!(testable.calculate(4), 3);
        assert_eq!(testable.calculate(5), 3);
        assert_eq!(testable.calculate(6), 3);
        assert_eq!(testable.calculate(7), 3);
    }

    #[test]
    fn index_calculate() {
        let testable = IndexTrophyCalculator {};

        assert_eq!(testable.calculate(0), 0);
        assert_eq!(testable.calculate(1), 1);
        assert_eq!(testable.calculate(2), 2);
        assert_eq!(testable.calculate(3), 3);
        assert_eq!(testable.calculate(4), 4);
        assert_eq!(testable.calculate(5), 5);
        assert_eq!(testable.calculate(6), 6);
        assert_eq!(testable.calculate(7), 7);
        assert_eq!(testable.calculate(8), 8);
        assert_eq!(testable.calculate(9), 9);
        assert_eq!(testable.calculate(10), 10);
        assert_eq!(testable.calculate(11), 10);
        assert_eq!(testable.calculate(12), 10);
        assert_eq!(testable.calculate(13), 10);
    }
}
