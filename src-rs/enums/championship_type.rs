use crate::enum_str;
use crate::enums::driver_group::DriverGroup;

enum_str! {
    enum ChampionshipType {
        Class,
        PAX,
        Novice,
        Ladies,
    }
}

impl ChampionshipType {
    pub fn from(driver_group: DriverGroup) -> Option<Self> {
        match driver_group {
            DriverGroup::Ladies => Some(Self::Ladies),
            DriverGroup::Novice => Some(Self::Novice),
            DriverGroup::PAX => Some(Self::PAX),
            DriverGroup::Raw => None,
        }
    }
}
