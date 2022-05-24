use crate::models::driver::Driver;

pub trait TrophyCalculator {
    fn calculate(&self, driver_count: usize) -> usize {
        if driver_count <= 1 {
            0
        } else if driver_count >= 10 {
            3 + ((((driver_count - 9) as f64) / 4.).ceil() as usize)
        } else {
            ((driver_count as f64) / 3.).ceil() as usize
        }
    }

    fn calculate_vec(&self, drivers: Vec<&Driver>) -> usize {
        self.calculate(drivers.len())
    }
}

pub struct DefaultTrophyCalculator {}

impl TrophyCalculator for DefaultTrophyCalculator {}

#[cfg(test)]
mod test {
    use crate::services::trophy_calculator::{DefaultTrophyCalculator, TrophyCalculator};

    #[test]
    fn calculate() {
        let testable = DefaultTrophyCalculator {};

        assert_eq!(testable.calculate(0), 0);
        assert_eq!(testable.calculate(1), 0);
        assert_eq!(testable.calculate(2), 1);
        assert_eq!(testable.calculate(3), 1);
        assert_eq!(testable.calculate(4), 2);
        assert_eq!(testable.calculate(5), 2);
        assert_eq!(testable.calculate(6), 2);
        assert_eq!(testable.calculate(7), 3);
        assert_eq!(testable.calculate(8), 3);
        assert_eq!(testable.calculate(9), 3);
        assert_eq!(testable.calculate(10), 4);
        assert_eq!(testable.calculate(11), 4);
        assert_eq!(testable.calculate(12), 4);
        assert_eq!(testable.calculate(13), 4);
        assert_eq!(testable.calculate(14), 5);
        assert_eq!(testable.calculate(15), 5);
        assert_eq!(testable.calculate(16), 5);
    }
}
