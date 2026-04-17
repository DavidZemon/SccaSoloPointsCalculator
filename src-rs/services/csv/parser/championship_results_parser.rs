use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;
use std::rc::Rc;

use anyhow::{Context, Result};
use calamine::{Data, Range, Reader, Xls, Xlsx};
use regex::Regex;

use crate::console_log;
use crate::enums::championship_type::ChampionshipType;
use crate::enums::short_car_class::ShortCarClass;
use crate::models::driver::Driver;
use crate::models::event_results::EventResults;
use crate::models::lap_time::{dns, LapTime};
use crate::models::type_aliases::DriverId;
use crate::services::csv::builder::championship::class_csv_builder::{ClassCsvBuilder, DefaultClassCsvBuilder};
use crate::services::csv::builder::championship::indexed_csv_builder::{DefaultIndexedCsvBuilder, IndexedCsvBuilder};
use crate::services::csv::parser::class_championship_results_parser::{
    ClassChampionshipResultsParser, DefaultClassChampionshipResultsParser,
};
use crate::services::csv::parser::errors::ChampionshipParseError;
use crate::services::csv::parser::index_championship_results_parser::{
    DefaultIndexChampionshipResultsParser, IndexChampionshipResultsParser,
};
use crate::utilities::log;

pub struct ChampionshipResultsParser {
    class_results_parser: Rc<RefCell<dyn ClassChampionshipResultsParser>>,
    index_results_parser: Rc<RefCell<dyn IndexChampionshipResultsParser>>,
    class_csv_builder: Rc<RefCell<dyn ClassCsvBuilder>>,
    indexed_csv_builder: Rc<RefCell<dyn IndexedCsvBuilder>>,

    event_results: EventResults,
}

impl ChampionshipResultsParser {
    pub fn new(event_results: EventResults) -> ChampionshipResultsParser {
        ChampionshipResultsParser {
            class_results_parser: Rc::new(RefCell::new(DefaultClassChampionshipResultsParser::default())),
            index_results_parser: Rc::new(RefCell::new(DefaultIndexChampionshipResultsParser::default())),
            class_csv_builder: Rc::new(RefCell::new(DefaultClassCsvBuilder::default())),
            indexed_csv_builder: Rc::new(RefCell::new(DefaultIndexedCsvBuilder::default())),
            event_results,
        }
    }

    pub fn process_results(
        &self,
        new_results_type: ChampionshipType,
        new_results: &[u8],
        file_name: String,
    ) -> Result<String> {
        self.process_results_inner(new_results_type, new_results, file_name.clone())
            .context(format!(
                "processing {new_results_type:?} championship from `{file_name}`"
            ))
    }

    fn process_results_inner(
        &self,
        new_results_type: ChampionshipType,
        new_results: &[u8],
        file_name: String,
    ) -> Result<String> {
        let event_drivers_by_id = self
            .event_results
            .get_drivers(None)
            .iter()
            .filter(|d| d.car_class.short != ShortCarClass::FUN && !d.dsq)
            .map(|d| (d.id.clone(), *d))
            .collect::<HashMap<DriverId, &Driver>>();

        let old_data = self
            .extract_sheet(file_name.clone(), new_results)
            .context(format!("reading sheet from `{file_name}`"))?;
        let header_map = self.get_header_map(&old_data).context("creating the header map")?;
        let past_event_count = self
            .get_past_event_count(&header_map)
            .context("counting prior events")?;

        let result = if new_results_type == ChampionshipType::Class {
            self.class_csv_builder
                .borrow()
                .create(self.class_results_parser.borrow().parse(
                    past_event_count,
                    header_map,
                    old_data,
                    &self.event_results,
                )?)
        } else {
            let new_drivers = event_drivers_by_id
                .iter()
                .filter(|(_, d)| match new_results_type {
                    ChampionshipType::Novice => d.rookie,
                    ChampionshipType::Ladies => d.ladies_championship,
                    _ => true,
                })
                .map(|(id, d)| (id.clone(), *d))
                .collect::<HashMap<DriverId, &Driver>>();
            let fastest = Self::compute_fastest(&new_drivers);
            let results = self.index_results_parser.borrow().parse(
                past_event_count,
                header_map,
                old_data,
                new_drivers,
                &fastest,
            )?;
            self.indexed_csv_builder.borrow().create(new_results_type, results)
        };

        result.map(|csv_option| csv_option.unwrap_or(format!("No results for {}", new_results_type.name())))
    }

    fn extract_sheet(&self, file_name: String, new_results: &[u8]) -> Result<Range<Data>> {
        let new_results = Cursor::new(new_results);
        let worksheets = if file_name.to_lowercase().ends_with(".xls") {
            Xls::new(new_results)
                .context(format!("opening `{file_name}` as XLS"))?
                .worksheets()
        } else if file_name.to_lowercase().ends_with(".xlsx") {
            Xlsx::new(new_results)
                .context(format!("opening `{file_name}` as XLSX"))?
                .worksheets()
        } else {
            return Err(ChampionshipParseError::UnsupportedExtension { file_name }.into());
        };
        let mut sheets = worksheets
            .iter()
            .filter(|(name, _)| name.trim().to_lowercase() != "calculations")
            .collect::<Vec<&(String, Range<Data>)>>();
        sheets.sort_by(|(lhs_name, ..), (rhs_name, ..)| lhs_name.cmp(rhs_name));
        sheets.reverse();

        Self::find_sheet(file_name, sheets.as_slice())
    }

    fn find_sheet(file_name: String, sheets: &[&(String, Range<Data>)]) -> Result<Range<Data>> {
        let (sheet_name, sheet_data) = sheets.first().ok_or(ChampionshipParseError::NoUsableSheet)?;

        if sheet_data.rows().len() >= 5 {
            console_log!("Found sheet with name {}", sheet_name);
            Ok(sheet_data.clone())
        } else if sheets.len() > 1 {
            log(format!("Sheet '{}' doesn't have enough rows, checking next", sheet_name).as_str());
            Self::find_sheet(file_name, &sheets[1..])
        } else {
            Err(ChampionshipParseError::AllSheetsEmpty { file_name }.into())
        }
    }

    fn get_header_map(&self, data: &Range<Data>) -> Result<HashMap<String, usize>> {
        let re = Regex::new(r"^\s*best\s+\d+\s+of\s+\d+\s*$").context("compiling header regex")?;
        Ok(data
            .rows()
            .find(|row| match row.last() {
                Some(last) => re.is_match(last.to_string().to_lowercase().as_str()),
                None => false,
            })
            .ok_or(ChampionshipParseError::MissingHeaderRow)?
            .iter()
            .enumerate()
            .map(|(index, header)| (header.to_string(), index))
            .collect())
    }

    fn get_past_event_count(&self, header_map: &HashMap<String, usize>) -> Result<usize> {
        let re = Regex::new(r"^(Trophy|Rank|Driver|Total\s+Points|Best\s+\d+\s+of\s+\d+)$")
            .context("compiling past-event-count regex")?;
        Ok(header_map.keys().filter(|header| !re.is_match(header)).count())
    }

    fn compute_fastest(drivers: &HashMap<DriverId, &Driver>) -> LapTime {
        drivers.values().map(|d| d.best_lap(false)).min().unwrap_or_else(dns)
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    use crate::services::csv::builder::championship::class_csv_builder::MockClassCsvBuilder;
    use crate::services::csv::builder::championship::indexed_csv_builder::MockIndexedCsvBuilder;
    use crate::services::csv::parser::class_championship_results_parser::MockClassChampionshipResultsParser;
    use crate::services::csv::parser::index_championship_results_parser::MockIndexChampionshipResultsParser;
    use crate::{ChampionshipResultsParser, ChampionshipType, EventResults};

    struct Context {
        mock_class_results_parser: Rc<RefCell<MockClassChampionshipResultsParser>>,
        mock_index_results_parser: Rc<RefCell<MockIndexChampionshipResultsParser>>,
        mock_class_csv_builder: Rc<RefCell<MockClassCsvBuilder>>,
        mock_indexed_csv_builder: Rc<RefCell<MockIndexedCsvBuilder>>,
        testable: ChampionshipResultsParser,
    }

    impl Context {
        pub fn new(event_results: EventResults) -> Context {
            let mock_class_results_parser = Rc::new(RefCell::new(MockClassChampionshipResultsParser::new()));
            let mock_index_results_parser = Rc::new(RefCell::new(MockIndexChampionshipResultsParser::new()));
            let mock_class_csv_builder = Rc::new(RefCell::new(MockClassCsvBuilder::new()));
            let mock_indexed_csv_builder = Rc::new(RefCell::new(MockIndexedCsvBuilder::new()));

            Context {
                mock_class_results_parser: Rc::clone(&mock_class_results_parser),
                mock_index_results_parser: Rc::clone(&mock_index_results_parser),
                mock_class_csv_builder: Rc::clone(&mock_class_csv_builder),
                mock_indexed_csv_builder: Rc::clone(&mock_indexed_csv_builder),
                testable: ChampionshipResultsParser {
                    class_results_parser: mock_class_results_parser,
                    index_results_parser: mock_index_results_parser,
                    class_csv_builder: mock_class_csv_builder,
                    indexed_csv_builder: mock_indexed_csv_builder,
                    event_results,
                },
            }
        }
    }

    #[test]
    fn test_process_results_bad_excel_fails_gracefully() {
        let results = EventResults {
            results: HashMap::new(),
        };
        let context = Context::new(results);
        {
            context.mock_class_results_parser.borrow_mut().expect_parse().never();
            context.mock_index_results_parser.borrow_mut().expect_parse().never();
            context.mock_class_csv_builder.borrow_mut().expect_create().never();
            context.mock_indexed_csv_builder.borrow_mut().expect_create().never();
        }

        let data = [];
        let actual = context
            .testable
            .process_results(ChampionshipType::Class, &data, "a file.xls".to_string());
        let err = actual.expect_err("expected invalid XLS to yield an error");
        let rendered = format!("{err:#}");
        // The anyhow chain now carries our `.context(...)` frames in front of
        // the underlying calamine error; assert on the substrings we care
        // about rather than an exact match.
        assert!(
            rendered.contains("a file.xls"),
            "expected chain to mention the file name; got: {rendered}"
        );
        assert!(
            rendered.contains("failed to fill whole buffer"),
            "expected chain to include the root cause; got: {rendered}"
        );
    }
}
