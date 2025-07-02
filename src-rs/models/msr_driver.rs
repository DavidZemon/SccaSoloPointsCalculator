use crate::models::type_aliases::DriverId;
use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct MsrDriver {
    #[serde(rename = "Last Name")]
    pub last_name: String,
    #[serde(rename = "First Name")]
    pub first_name: String,
    #[serde(rename = "Member #")]
    pub member_number: String,
    #[serde(rename = "Class + Modifier/PAX")]
    pub class_and_pax: String,
    #[serde(rename = "No.")]
    pub car_number: usize,
    #[serde(rename = "Vehicle Year/Make/Model/Color")]
    pub car: String,
    #[serde(rename = "Region of Record Abbreviation")]
    pub region: Option<String>,
    #[serde(rename = "Medical condition? (Optional)")]
    pub medical: Option<String>,
    #[serde(rename = "Novice")]
    pub novice: Option<u8>,
    #[serde(rename = "Ladies")]
    pub ladies: Option<u8>,
}

impl MsrDriver {
    pub fn id(&self) -> DriverId {
        format!("{} {}", self.first_name.to_lowercase(), self.last_name.to_lowercase())
            .trim()
            .to_string()
    }
}
