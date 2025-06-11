use crate::enums::short_car_class::ShortCarClass;
use crate::models::class_results::ClassResults;
use crate::models::driver::Driver;
use crate::models::driver_from_pronto::DriverFromPronto;
use crate::models::event_results::EventResults;
use crate::models::lap_time::{LapTime, Penalty};
use crate::models::type_aliases::{PaxMultiplier, Time};
use bigdecimal::ParseBigDecimalError;
use csv::{StringRecord, Trim};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn parse(file_contents: String) -> Result<EventResults, String> {
    let mut reader1 = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::Headers)
        .from_reader(file_contents.as_bytes());

    let mut string_reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(Trim::Headers)
        .from_reader(file_contents.as_bytes());
    let headers = string_reader.headers().map_err(|e| e.to_string())?;
    let header_vec: Vec<&str> = headers.iter().collect();
    let final_column_index = header_vec
        .iter()
        .enumerate()
        .find(|(_index, header)| **header == "Runs Day2")
        .map(|(index, _header)| index)
        .ok_or("Unable to find header column `Runs Day2`".to_string())?;

    let mut results = HashMap::new();

    let records = reader1.deserialize().zip(string_reader.records());
    for (deserialized, string_rec) in records {
        let (driver, string_rec) = validate_row(deserialized, string_rec)?;

        let driver = perform_second_parsing(driver, string_rec, final_column_index + 1)?;
        let driver = Driver::from(driver);
        let class = if driver.expert {
            ShortCarClass::X
        } else {
            driver.car_class.short
        };

        results.entry(class).or_insert_with(|| ClassResults::new(class));

        if let Some(class_results) = results.get_mut(&class) {
            class_results.add_driver(driver)
        }
    }

    Ok(EventResults { results })
}

fn validate_row(
    deserialized: csv::Result<DriverFromPronto>,
    string_record: csv::Result<StringRecord>,
) -> Result<(DriverFromPronto, StringRecord), String> {
    let string_record = string_record.map_err(|e| e.to_string())?;

    match deserialized {
        Ok(driver) => Ok((driver, string_record)),
        Err(e) => match e.kind() {
            csv::ErrorKind::Deserialize { err: root, .. } => {
                match root.kind() {
                    csv::DeserializeErrorKind::UnexpectedEndOfRow => Err(
                        format!(
                            "Encountered an unexpected end of row for a record. One common reason for this is a driver that did not attend the event but remains in Pronto.\n'{:?}'",
                            string_record,
                        )
                    ),
                    _ => Err(format!("Failed to deserialize row {:?} due to {:?}", string_record, e.to_string())),
                }
            }
            _ => Err(format!("Failed to deserialize row {:?} due to {:?}", string_record, e.to_string())),
        },
    }
}

fn perform_second_parsing(
    mut driver: DriverFromPronto,
    string_record: StringRecord,
    first_time_column: usize,
) -> Result<DriverFromPronto, String> {
    // Start by just dealing with the Pronto's poor handling of our "Ladies" field in that it
    // sometimes (always?) puts the ladies indicator in the region column
    {
        if let Some(region) = driver.region.clone() {
            if region.trim() == "L" {
                driver.ladies = Some(region);
            }
        }
    }

    // Then deal with the day 1/2 lap times
    {
        let strings_vec: Vec<&str> = string_record.iter().collect();
        let pax_multiplier = PaxMultiplier::from_str(&driver.pax_multiplier).unwrap();

        let extra_fields = &strings_vec[first_time_column..];
        let extra_field_groups = extra_fields.chunks(3);

        let driver_name = format!(
            "{} {}",
            driver.first_name.as_ref().unwrap_or(&"".to_string()),
            driver.last_name.as_ref().unwrap_or(&"".to_string())
        );

        for run in extra_field_groups {
            if run.len() != 3 {
                #[cfg(not(test))]
                crate::console_log!("Driver {driver_name} has run cell count not divisible by 3");
            } else {
                match build_lap_time(run, &pax_multiplier) {
                    Ok(run) => {
                        driver.runs.push(run);
                    }
                    Err(e) => {
                        return Err(format!(
                            "Failed to extract lap time for {driver_name} ({run:?}) due to {e}"
                        ))
                    }
                }
            }
        }
    }

    Ok(driver)
}

fn build_lap_time(next_fields: &[&str], pax: &PaxMultiplier) -> Result<LapTime, String> {
    Ok(LapTime::new(
        Time::from_str(next_fields[0]).map_err(|e: ParseBigDecimalError| e.to_string())?,
        pax.clone(),
        next_fields[1].parse().map_err(|e: ParseIntError| e.to_string())?,
        match next_fields[2] {
            "DNF" => Some(Penalty::DNF),
            "DNS" => Some(Penalty::DNS),
            "RRN" => Some(Penalty::RRN),
            "DSQ" => Some(Penalty::DSQ),
            _ => None,
        },
    ))
}

#[cfg(test)]
mod test {
    use crate::enums::short_car_class::ShortCarClass;
    use crate::models::lap_time::{LapTime, Penalty};
    use crate::models::type_aliases::{PaxMultiplier, Time};
    use crate::services::csv::parser::event_results_parser::parse;
    use bigdecimal::Zero;
    use std::fs;
    use std::str::FromStr;

    #[test]
    fn parse_2022_e1_event_results() {
        let sample_contents = fs::read_to_string("./SampleData/2022/2022_Event1-DavidExport.csv").unwrap();
        let actual = parse(sample_contents).unwrap();

        assert_eq!(actual.results.len(), 24);
        assert!(actual.results.contains_key(&ShortCarClass::AS));
        assert!(actual.results.contains_key(&ShortCarClass::BS));
        assert!(actual.results.contains_key(&ShortCarClass::CAMC));
        assert!(actual.results.contains_key(&ShortCarClass::CAMS));
        assert!(actual.results.contains_key(&ShortCarClass::CAMT));
        assert!(actual.results.contains_key(&ShortCarClass::CS));
        assert!(actual.results.contains_key(&ShortCarClass::CSP));
        assert!(actual.results.contains_key(&ShortCarClass::DP));
        assert!(actual.results.contains_key(&ShortCarClass::DS));
        assert!(actual.results.contains_key(&ShortCarClass::ES));
        assert!(actual.results.contains_key(&ShortCarClass::EVX));
        assert!(actual.results.contains_key(&ShortCarClass::FS));
        assert!(actual.results.contains_key(&ShortCarClass::FSAE));
        assert!(actual.results.contains_key(&ShortCarClass::FSP));
        assert!(actual.results.contains_key(&ShortCarClass::FUN));
        assert!(actual.results.contains_key(&ShortCarClass::GS));
        assert!(actual.results.contains_key(&ShortCarClass::GSL));
        assert!(actual.results.contains_key(&ShortCarClass::HS));
        assert!(actual.results.contains_key(&ShortCarClass::HSL));
        assert!(actual.results.contains_key(&ShortCarClass::SMF));
        assert!(actual.results.contains_key(&ShortCarClass::SSC));
        assert!(actual.results.contains_key(&ShortCarClass::XP));
        assert!(actual.results.contains_key(&ShortCarClass::XA));
        assert!(actual.results.contains_key(&ShortCarClass::XB));

        let a_street = actual.results.get(&ShortCarClass::AS).unwrap();
        assert_eq!(a_street.car_class.short, ShortCarClass::AS);
        assert_eq!(a_street.drivers.len(), 5);
        assert_eq!(
            a_street.get_best_in_class(),
            LapTime::new(
                Time::from_str("52.288").unwrap(),
                PaxMultiplier::from_str("0.821").unwrap(),
                0,
                None
            )
        );
        let robert = a_street.drivers[0].clone();
        assert!(!robert.error);
        assert_eq!(robert.id, "robert fullriede");
        assert_eq!(robert.name, "Robert Fullriede");
        assert_eq!(robert.car_number, 52);
        assert_eq!(robert.car_class.short, ShortCarClass::AS);
        assert_eq!(robert.car_description, "2010 Porsche Cayman");
        assert_eq!(robert.region, "");
        assert!(!robert.rookie);
        assert!(!robert.ladies_championship);
        assert_eq!(robert.position, Some(1));
        assert!(!robert.dsq);
        assert_eq!(robert.pax_multiplier, PaxMultiplier::from_str("0.821").unwrap());
        assert_eq!(
            robert.times,
            vec![
                LapTime::new(
                    Time::from_str("53.351").unwrap(),
                    PaxMultiplier::from_str("0.821").unwrap(),
                    0,
                    None
                ),
                LapTime::new(
                    Time::zero(),
                    PaxMultiplier::from_str("0.821").unwrap(),
                    0,
                    Some(Penalty::DNF)
                ),
                LapTime::new(
                    Time::from_str("52.288").unwrap(),
                    PaxMultiplier::from_str("0.821").unwrap(),
                    0,
                    None
                ),
            ]
        );

        for (index, driver) in a_street.drivers.iter().enumerate() {
            assert_eq!(driver.position, Some(index + 1));
        }
    }

    #[test]
    fn parse_2023_e3_event_results() {
        let sample_contents = fs::read_to_string("./SampleData/2023/2023_Event3-DavidExport.csv").unwrap();

        let actual = match parse(sample_contents) {
            Ok(actual) => actual,
            Err(e) => panic!("Parsed failed due to: {}", e),
        };
        assert!(actual.results.contains_key(&ShortCarClass::AS));
        assert!(actual.results.contains_key(&ShortCarClass::BS));
        assert!(actual.results.contains_key(&ShortCarClass::CAMC));
        assert!(actual.results.contains_key(&ShortCarClass::CAMS));
        assert!(actual.results.contains_key(&ShortCarClass::CAMT));
        assert!(actual.results.contains_key(&ShortCarClass::CS));
        assert!(actual.results.contains_key(&ShortCarClass::DM));
        assert!(actual.results.contains_key(&ShortCarClass::DML));
        assert!(actual.results.contains_key(&ShortCarClass::DS));
        assert!(actual.results.contains_key(&ShortCarClass::DSP));
        assert!(actual.results.contains_key(&ShortCarClass::ES));
        assert!(actual.results.contains_key(&ShortCarClass::FP));
        assert!(actual.results.contains_key(&ShortCarClass::FS));
        assert!(actual.results.contains_key(&ShortCarClass::GS));
        assert!(actual.results.contains_key(&ShortCarClass::HS));
        assert!(actual.results.contains_key(&ShortCarClass::SMF));
        assert!(actual.results.contains_key(&ShortCarClass::SS));
        assert!(actual.results.contains_key(&ShortCarClass::SSC));
        assert!(actual.results.contains_key(&ShortCarClass::SSL));
        assert!(actual.results.contains_key(&ShortCarClass::SSM));
        assert!(actual.results.contains_key(&ShortCarClass::XA));
        assert!(actual.results.contains_key(&ShortCarClass::XB));
        assert!(actual.results.contains_key(&ShortCarClass::XP));
        assert_eq!(actual.results.len(), 24);

        let a_street = actual.results.get(&ShortCarClass::AS).unwrap();
        assert_eq!(a_street.car_class.short, ShortCarClass::AS);
        assert_eq!(a_street.drivers.len(), 5);
        assert_eq!(
            a_street.get_best_in_class(),
            LapTime::new(
                Time::from_str("45.269").unwrap(),
                PaxMultiplier::from_str("0.823").unwrap(),
                0,
                None
            )
        );

        let robert = a_street.drivers[0].clone();
        assert!(!robert.error);
        assert_eq!(robert.id, "robert fullriede");
        assert_eq!(robert.name, "Robert Fullriede");
        assert_eq!(robert.car_number, 52);
        assert_eq!(robert.car_class.short, ShortCarClass::AS);
        assert_eq!(robert.car_description, "2010 Porsche Cayman");
        assert_eq!(robert.region, "");
        assert!(!robert.rookie);
        assert!(!robert.ladies_championship);
        assert_eq!(robert.position, Some(1));
        assert!(!robert.dsq);
        assert_eq!(robert.pax_multiplier, PaxMultiplier::from_str("0.823").unwrap());
        assert_eq!(
            robert.times,
            vec![
                LapTime::new(
                    Time::from_str("48.317").unwrap(),
                    PaxMultiplier::from_str("0.823").unwrap(),
                    6,
                    None
                ),
                LapTime::new(
                    Time::from_str("47.069").unwrap(),
                    PaxMultiplier::from_str("0.823").unwrap(),
                    0,
                    None
                ),
                LapTime::new(
                    Time::from_str("46.247").unwrap(),
                    PaxMultiplier::from_str("0.823").unwrap(),
                    0,
                    None
                ),
                LapTime::new(
                    Time::from_str("45.519").unwrap(),
                    PaxMultiplier::from_str("0.823").unwrap(),
                    0,
                    None
                ),
                LapTime::new(
                    Time::from_str("45.269").unwrap(),
                    PaxMultiplier::from_str("0.823").unwrap(),
                    0,
                    None
                ),
                LapTime::new(
                    Time::from_str("45.559").unwrap(),
                    PaxMultiplier::from_str("0.823").unwrap(),
                    0,
                    None
                ),
            ],
        );

        for (index, driver) in a_street.drivers.iter().enumerate() {
            assert_eq!(driver.position, Some(index + 1));
        }
    }

    #[test]
    fn parse_results_with_no_show_driver() {
        let sample_contents = r#"Position, Class, Class Category, Class Name, Number, First Name,Last Name, Car Year, Car Make, Car Model, Car Color, Member #, Rookie, Ladies, DSQ, Region, Best Run, Pax Index, Pax Time, Runs Day1, Runs Day2, Runs (Time/Cones/Penalty)
"1","SS","Street","Super Street","78","Sean","Greer","2022","Chevrolet","Challenger Cobra 392","urine","432501","0","","0","STL","41.442","0.83","34.397","6","0","42.429","0","","41.862","0","","41.595","0","","41.537","0","","41.445","0","","41.442","0",""
"17","CAMT","Other","Classic American Muscle Traditional","88","Charles","Hammelman","1999","Ford","Mustang SVT Cobra","Black","691686","1","","0","","DNF","0.816","999","0","0""#;

        let actual = parse(sample_contents.to_string());

        assert!(actual.is_err(), "Should fail on empty driver");
        assert_eq!(actual.err().unwrap(), "Encountered an unexpected end of row for a record. One common reason for this is a driver that did not attend the event but remains in Pronto.\n'StringRecord([\"17\", \"CAMT\", \"Other\", \"Classic American Muscle Traditional\", \"88\", \"Charles\", \"Hammelman\", \"1999\", \"Ford\", \"Mustang SVT Cobra\", \"Black\", \"691686\", \"1\", \"\", \"0\", \"\", \"DNF\", \"0.816\", \"999\", \"0\", \"0\"])'");
    }
}
