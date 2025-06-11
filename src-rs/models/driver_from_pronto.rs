use crate::enums::short_car_class::ShortCarClass;
use crate::models::lap_time::LapTime;
use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct DriverFromPronto {
    #[serde(rename = "Position")]
    pub position: Option<u16>,
    #[serde(rename = "Class")]
    pub car_class: ShortCarClass,
    #[serde(rename = "Number")]
    pub car_number: u16,
    #[serde(rename = "First Name")]
    pub first_name: Option<String>,
    #[serde(rename = "Last Name")]
    pub last_name: Option<String>,
    #[serde(rename = "Car Year")]
    pub year: Option<u32>,
    #[serde(rename = "Car Make")]
    pub make: Option<String>,
    #[serde(rename = "Car Model")]
    pub model: Option<String>,
    #[serde(rename = "Car Color")]
    pub color: Option<String>,
    #[serde(rename = "Member #")]
    pub member_number: Option<String>,
    #[serde(rename = "Rookie")]
    pub rookie: Option<u8>,
    #[serde(rename = "Ladies")]
    pub ladies: Option<String>,
    #[serde(rename = "Expert")]
    pub expert: Option<u8>,
    #[serde(rename = "DSQ")]
    pub dsq: Option<u8>,
    #[serde(rename = "Region")]
    pub region: Option<String>,
    #[serde(rename = "Best Run")]
    pub best_run: String,
    #[serde(rename = "Pax Index")]
    pub pax_multiplier: String,
    #[serde(rename = "Pax Time")]
    pub pax_time: String,
    #[serde(skip)]
    pub runs: Vec<LapTime>,
}
