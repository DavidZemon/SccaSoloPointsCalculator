#[cfg(test)]
use mockall::automock;

use crate::enums::championship_type::ChampionshipType;
use crate::models::championship_driver::ChampionshipDriver;
use crate::models::championship_results::IndexedChampionshipResults;
use crate::services::calculators::tie_calculator::calculate_tie_offset;
use crate::services::calculators::trophy_calculator::{IndexTrophyCalculator, TrophyCalculator};
use crate::utilities::events_to_count;

#[cfg_attr(test, automock)]
pub trait IndexedCsvBuilder {
    fn create(
        &self,
        championship_type: ChampionshipType,
        results: IndexedChampionshipResults,
    ) -> Result<Option<String>, String>;
}

pub struct DefaultIndexedCsvBuilder {
    trophy_calculator: Box<dyn TrophyCalculator>,
}

impl IndexedCsvBuilder for DefaultIndexedCsvBuilder {
    fn create(
        &self,
        championship_type: ChampionshipType,
        results: IndexedChampionshipResults,
    ) -> Result<Option<String>, String> {
        let event_count = results
            .drivers
            .get(0)
            .ok_or("Expected at least one driver")?
            .event_count(true);
        let events_to_count = events_to_count(event_count);
        let header = Self::build_header(event_count);
        let trophy_count = self.trophy_calculator.calculate(results.drivers.len());

        let mut rows = vec![
            results.organization.clone(),
            format!(
                "{} {} Championship -- Best {} of {} Events",
                results.year,
                championship_type.name(),
                events_to_count,
                event_count
            ),
            "".to_string(),
            header,
        ];
        let mut sorted = results.drivers;
        sorted.sort_by_key(|d| d.best_of(events_to_count));
        sorted.reverse();

        let filtered_drivers = sorted
            .iter()
            .filter(|d| d.total_points() != 0)
            .collect::<Vec<&ChampionshipDriver>>();
        rows.extend(filtered_drivers.iter().enumerate().map(|(index, d)| {
            let tie_offset = calculate_tie_offset(&filtered_drivers, index, |d1, d2| {
                d1.total_points() == d2.total_points()
            });

            let mut driver_row = vec![
                if (index - tie_offset) < trophy_count {
                    "T".to_string()
                } else {
                    "".to_string()
                },
                format!("{}", index + 1 - tie_offset),
                d.name().clone(),
            ];
            d.points()
                .iter()
                .for_each(|points| driver_row.push(format!("{}", points)));
            driver_row.push(format!("{}", d.total_points()));
            driver_row.push(format!("{}", d.best_of(events_to_count)));
            driver_row.join(",")
        }));

        Ok(Some(rows.join("\n")))
    }
}

impl Default for DefaultIndexedCsvBuilder {
    fn default() -> Self {
        Self::from(None)
    }
}

impl DefaultIndexedCsvBuilder {
    pub fn from(trophy_calculator: Option<Box<dyn TrophyCalculator>>) -> DefaultIndexedCsvBuilder {
        Self {
            trophy_calculator: trophy_calculator
                .unwrap_or_else(|| Box::new(IndexTrophyCalculator {})),
        }
    }

    fn build_header(event_count: usize) -> String {
        let mut header = vec![
            "Trophy".to_string(),
            "Rank".to_string(),
            "Driver".to_string(),
        ];
        header.extend((0..event_count).map(|i| format!("Event #{}", i + 1)));
        header.push("Total Points".to_string());
        header.push(format!(
            "Best {} of {}",
            events_to_count(event_count),
            event_count
        ));
        header.join(",")
    }
}

#[cfg(test)]
mod test {
    use crate::enums::championship_type::ChampionshipType;
    use crate::models::championship_driver::ChampionshipDriver;
    use crate::models::championship_results::IndexedChampionshipResults;
    use crate::services::calculators::trophy_calculator::TrophyCalculator;
    use crate::services::csv::builder::championship::indexed_csv_builder::{
        DefaultIndexedCsvBuilder, IndexedCsvBuilder,
    };

    struct MockTrophyCalculator {}

    impl TrophyCalculator for MockTrophyCalculator {
        fn calculate(&self, _: usize) -> usize {
            2
        }
    }

    #[test]
    fn test_tie() {
        let testable = DefaultIndexedCsvBuilder::from(Some(Box::from(MockTrophyCalculator {})));

        let mut d1 = ChampionshipDriver::new("Name 1");
        let mut d2 = ChampionshipDriver::new("Name 2");
        let mut d3 = ChampionshipDriver::new("Name 3");

        d1.add_event(10);
        d2.add_event(10);
        d3.add_event(100);

        let actual = testable.create(
            ChampionshipType::PAX,
            IndexedChampionshipResults::new(2022, "SCCA".to_string(), vec![d1, d2, d3]),
        );

        assert!(actual.is_ok());
        let actual_option = actual.unwrap();
        assert!(actual_option.is_some());

        let unwrapped = actual_option.unwrap();

        assert_eq!(
            unwrapped,
            "SCCA\n\
2022 PAX Championship -- Best 1 of 1 Events\n\
\n\
Trophy,Rank,Driver,Event #1,Total Points,Best 1 of 1\n\
T,1,Name 3,100,100,100\n\
T,2,Name 2,10,10,10\n\
T,2,Name 1,10,10,10"
                .to_string()
        );
    }
}
