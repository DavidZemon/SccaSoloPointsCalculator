use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;
use serde::Serialize;

use crate::enums::class_category::ClassCategory;
use crate::enums::long_car_class::LongCarClass;
use crate::enums::short_car_class::ShortCarClass;

#[derive(Copy, Clone, Debug, Serialize)]
pub struct CarClass {
    pub short: ShortCarClass,
    pub long: LongCarClass,
    pub category: ClassCategory,
}

impl CarClass {
    pub fn new(short: ShortCarClass, long: LongCarClass, category: ClassCategory) -> CarClass {
        CarClass {
            short,
            long,
            category,
        }
    }
}

impl Display for CarClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.short.name())
    }
}

lazy_static! {
    static ref CLASS_MAP: HashMap<ShortCarClass, CarClass> = {
        let mut m = HashMap::new();
        m.insert(
            ShortCarClass::SS,
            CarClass::new(
                ShortCarClass::SS,
                LongCarClass::Super_Street,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::AS,
            CarClass::new(
                ShortCarClass::AS,
                LongCarClass::A_Street,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::BS,
            CarClass::new(
                ShortCarClass::BS,
                LongCarClass::B_Street,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::CS,
            CarClass::new(
                ShortCarClass::CS,
                LongCarClass::C_Street,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::DS,
            CarClass::new(
                ShortCarClass::DS,
                LongCarClass::D_Street,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::ES,
            CarClass::new(
                ShortCarClass::ES,
                LongCarClass::E_Street,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::FS,
            CarClass::new(
                ShortCarClass::FS,
                LongCarClass::F_Street,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::GS,
            CarClass::new(
                ShortCarClass::GS,
                LongCarClass::G_Street,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::HS,
            CarClass::new(
                ShortCarClass::HS,
                LongCarClass::H_Street,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::STH,
            CarClass::new(
                ShortCarClass::STH,
                LongCarClass::Street_Touring_Hatchback,
                ClassCategory::Street_Touring_Category,
            ),
        );
        m.insert(
            ShortCarClass::STU,
            CarClass::new(
                ShortCarClass::STU,
                LongCarClass::Street_Touring_Ultra,
                ClassCategory::Street_Touring_Category,
            ),
        );
        m.insert(
            ShortCarClass::STX,
            CarClass::new(
                ShortCarClass::STX,
                LongCarClass::Street_Touring_Xtreme,
                ClassCategory::Street_Touring_Category,
            ),
        );
        m.insert(
            ShortCarClass::STR,
            CarClass::new(
                ShortCarClass::STR,
                LongCarClass::Street_Touring_Roadster,
                ClassCategory::Street_Touring_Category,
            ),
        );
        m.insert(
            ShortCarClass::STS,
            CarClass::new(
                ShortCarClass::STS,
                LongCarClass::Street_Touring_Sport,
                ClassCategory::Street_Touring_Category,
            ),
        );
        m.insert(
            ShortCarClass::SSP,
            CarClass::new(
                ShortCarClass::SSP,
                LongCarClass::Super_Street_Prepared,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::ASP,
            CarClass::new(
                ShortCarClass::ASP,
                LongCarClass::A_Street_Prepared,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::BSP,
            CarClass::new(
                ShortCarClass::BSP,
                LongCarClass::B_Street_Prepared,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::CSP,
            CarClass::new(
                ShortCarClass::CSP,
                LongCarClass::C_Street_Prepared,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::DSP,
            CarClass::new(
                ShortCarClass::DSP,
                LongCarClass::D_Street_Prepared,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::ESP,
            CarClass::new(
                ShortCarClass::ESP,
                LongCarClass::E_Street_Prepared,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::FSP,
            CarClass::new(
                ShortCarClass::FSP,
                LongCarClass::F_Street_Prepared,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::SSM,
            CarClass::new(
                ShortCarClass::SSM,
                LongCarClass::Super_Street_Modified,
                ClassCategory::Street_Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::SM,
            CarClass::new(
                ShortCarClass::SM,
                LongCarClass::Street_Modified,
                ClassCategory::Street_Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::SMF,
            CarClass::new(
                ShortCarClass::SMF,
                LongCarClass::Street_Modified_Front_Wheel_Drive,
                ClassCategory::Street_Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::XP,
            CarClass::new(
                ShortCarClass::XP,
                LongCarClass::X_Prepared,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::CP,
            CarClass::new(
                ShortCarClass::CP,
                LongCarClass::C_Prepared,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::DP,
            CarClass::new(
                ShortCarClass::DP,
                LongCarClass::D_Prepared,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::EP,
            CarClass::new(
                ShortCarClass::EP,
                LongCarClass::E_Prepared,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::FP,
            CarClass::new(
                ShortCarClass::FP,
                LongCarClass::F_Prepared,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::AM,
            CarClass::new(
                ShortCarClass::AM,
                LongCarClass::A_Modified,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::BM,
            CarClass::new(
                ShortCarClass::BM,
                LongCarClass::B_Modified,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::CM,
            CarClass::new(
                ShortCarClass::CM,
                LongCarClass::C_Modified,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::DM,
            CarClass::new(
                ShortCarClass::DM,
                LongCarClass::D_Modified,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::EM,
            CarClass::new(
                ShortCarClass::EM,
                LongCarClass::E_Modified,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::FM,
            CarClass::new(
                ShortCarClass::FM,
                LongCarClass::F_Modified,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::KM,
            CarClass::new(
                ShortCarClass::KM,
                LongCarClass::Kart_Modified,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::CAMC,
            CarClass::new(
                ShortCarClass::CAMC,
                LongCarClass::Classic_American_Muscle_Contemporary,
                ClassCategory::Classic_American_Muscle_Category,
            ),
        );
        m.insert(
            ShortCarClass::CAMT,
            CarClass::new(
                ShortCarClass::CAMT,
                LongCarClass::Classic_American_Muscle_Traditional,
                ClassCategory::Classic_American_Muscle_Category,
            ),
        );
        m.insert(
            ShortCarClass::CAMS,
            CarClass::new(
                ShortCarClass::CAMS,
                LongCarClass::Classic_American_Muscle_Sports,
                ClassCategory::Classic_American_Muscle_Category,
            ),
        );
        m.insert(
            ShortCarClass::XSA,
            CarClass::new(
                ShortCarClass::XSA,
                LongCarClass::Xtreme_Street_A,
                ClassCategory::Xtreme_Street,
            ),
        );
        m.insert(
            ShortCarClass::XSB,
            CarClass::new(
                ShortCarClass::XSB,
                LongCarClass::Xtreme_Street_B,
                ClassCategory::Xtreme_Street,
            ),
        );
        m.insert(
            ShortCarClass::EVX,
            CarClass::new(
                ShortCarClass::EVX,
                LongCarClass::Electric_Vehicle_Xtreme,
                ClassCategory::Miscellaneous_Category,
            ),
        );
        m.insert(
            ShortCarClass::SSC,
            CarClass::new(
                ShortCarClass::SSC,
                LongCarClass::Solo_Spec_Coupe,
                ClassCategory::Miscellaneous_Category,
            ),
        );
        m.insert(
            ShortCarClass::SSL,
            CarClass::new(
                ShortCarClass::SSL,
                LongCarClass::Super_Street_Ladies,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::ASL,
            CarClass::new(
                ShortCarClass::ASL,
                LongCarClass::A_Street_Ladies,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::BSL,
            CarClass::new(
                ShortCarClass::BSL,
                LongCarClass::B_Street_Ladies,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::CSL,
            CarClass::new(
                ShortCarClass::CSL,
                LongCarClass::C_Street_Ladies,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::DSL,
            CarClass::new(
                ShortCarClass::DSL,
                LongCarClass::D_Street_Ladies,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::ESL,
            CarClass::new(
                ShortCarClass::ESL,
                LongCarClass::E_Street_Ladies,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::FSL,
            CarClass::new(
                ShortCarClass::FSL,
                LongCarClass::F_Street_Ladies,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::GSL,
            CarClass::new(
                ShortCarClass::GSL,
                LongCarClass::G_Street_Ladies,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::HSL,
            CarClass::new(
                ShortCarClass::HSL,
                LongCarClass::H_Street_Ladies,
                ClassCategory::Street_Category,
            ),
        );
        m.insert(
            ShortCarClass::STHL,
            CarClass::new(
                ShortCarClass::STHL,
                LongCarClass::Street_Touring_Hatchback_Ladies,
                ClassCategory::Street_Touring_Category,
            ),
        );
        m.insert(
            ShortCarClass::STUL,
            CarClass::new(
                ShortCarClass::STUL,
                LongCarClass::Street_Touring_Ultra_Ladies,
                ClassCategory::Street_Touring_Category,
            ),
        );
        m.insert(
            ShortCarClass::STXL,
            CarClass::new(
                ShortCarClass::STXL,
                LongCarClass::Street_Touring_Xtreme_Ladies,
                ClassCategory::Street_Touring_Category,
            ),
        );
        m.insert(
            ShortCarClass::STRL,
            CarClass::new(
                ShortCarClass::STRL,
                LongCarClass::Street_Touring_Roadster_Ladies,
                ClassCategory::Street_Touring_Category,
            ),
        );
        m.insert(
            ShortCarClass::STSL,
            CarClass::new(
                ShortCarClass::STSL,
                LongCarClass::Street_Touring_Sport_Ladies,
                ClassCategory::Street_Touring_Category,
            ),
        );
        m.insert(
            ShortCarClass::SSPL,
            CarClass::new(
                ShortCarClass::SSPL,
                LongCarClass::Super_Street_Prepared_Ladies,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::ASPL,
            CarClass::new(
                ShortCarClass::ASPL,
                LongCarClass::A_Street_Prepared_Ladies,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::BSPL,
            CarClass::new(
                ShortCarClass::BSPL,
                LongCarClass::B_Street_Prepared_Ladies,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::CSPL,
            CarClass::new(
                ShortCarClass::CSPL,
                LongCarClass::C_Street_Prepared_Ladies,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::DSPL,
            CarClass::new(
                ShortCarClass::DSPL,
                LongCarClass::D_Street_Prepared_Ladies,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::ESPL,
            CarClass::new(
                ShortCarClass::ESPL,
                LongCarClass::E_Street_Prepared_Ladies,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::FSPL,
            CarClass::new(
                ShortCarClass::FSPL,
                LongCarClass::F_Street_Prepared_Ladies,
                ClassCategory::Street_Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::SSML,
            CarClass::new(
                ShortCarClass::SSML,
                LongCarClass::Super_Street_Modified_Ladies,
                ClassCategory::Street_Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::SML,
            CarClass::new(
                ShortCarClass::SML,
                LongCarClass::Street_Modified_Ladies,
                ClassCategory::Street_Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::SMFL,
            CarClass::new(
                ShortCarClass::SMFL,
                LongCarClass::Street_Modified_Front_Wheel_Drive_Ladies,
                ClassCategory::Street_Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::XPL,
            CarClass::new(
                ShortCarClass::XPL,
                LongCarClass::X_Prepared_Ladies,
                ClassCategory::Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::CPL,
            CarClass::new(
                ShortCarClass::CPL,
                LongCarClass::C_Prepared_Ladies,
                ClassCategory::Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::DPL,
            CarClass::new(
                ShortCarClass::DPL,
                LongCarClass::D_Prepared_Ladies,
                ClassCategory::Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::EPL,
            CarClass::new(
                ShortCarClass::EPL,
                LongCarClass::E_Prepared_Ladies,
                ClassCategory::Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::FPL,
            CarClass::new(
                ShortCarClass::FPL,
                LongCarClass::F_Prepared_Ladies,
                ClassCategory::Prepared_Category,
            ),
        );
        m.insert(
            ShortCarClass::AML,
            CarClass::new(
                ShortCarClass::AML,
                LongCarClass::A_Modified_Ladies,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::BML,
            CarClass::new(
                ShortCarClass::BML,
                LongCarClass::B_Modified_Ladies,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::CML,
            CarClass::new(
                ShortCarClass::CML,
                LongCarClass::C_Modified_Ladies,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::DML,
            CarClass::new(
                ShortCarClass::DML,
                LongCarClass::D_Modified_Ladies,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::EML,
            CarClass::new(
                ShortCarClass::EML,
                LongCarClass::E_Modified_Ladies,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::FML,
            CarClass::new(
                ShortCarClass::FML,
                LongCarClass::F_Modified_Ladies,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::KML,
            CarClass::new(
                ShortCarClass::KML,
                LongCarClass::Kart_Modified_Ladies,
                ClassCategory::Modified_Category,
            ),
        );
        m.insert(
            ShortCarClass::CAMCL,
            CarClass::new(
                ShortCarClass::CAMCL,
                LongCarClass::Classic_American_Muscle_Contemporary_Ladies,
                ClassCategory::Classic_American_Muscle_Category,
            ),
        );
        m.insert(
            ShortCarClass::CAMTL,
            CarClass::new(
                ShortCarClass::CAMTL,
                LongCarClass::Classic_American_Muscle_Traditional_Ladies,
                ClassCategory::Classic_American_Muscle_Category,
            ),
        );
        m.insert(
            ShortCarClass::CAMSL,
            CarClass::new(
                ShortCarClass::CAMSL,
                LongCarClass::Classic_American_Muscle_Sports_Ladies,
                ClassCategory::Classic_American_Muscle_Category,
            ),
        );
        m.insert(
            ShortCarClass::XSAL,
            CarClass::new(
                ShortCarClass::XSAL,
                LongCarClass::Xtreme_Street_A_Ladies,
                ClassCategory::Xtreme_Street,
            ),
        );
        m.insert(
            ShortCarClass::XSBL,
            CarClass::new(
                ShortCarClass::XSBL,
                LongCarClass::Xtreme_Street_B_Ladies,
                ClassCategory::Xtreme_Street,
            ),
        );
        m.insert(
            ShortCarClass::EVXL,
            CarClass::new(
                ShortCarClass::EVXL,
                LongCarClass::Electric_Vehicle_Xtreme_Ladies,
                ClassCategory::Miscellaneous_Category,
            ),
        );
        m.insert(
            ShortCarClass::SSCL,
            CarClass::new(
                ShortCarClass::SSCL,
                LongCarClass::Solo_Spec_Coupe_Ladies,
                ClassCategory::Miscellaneous_Category,
            ),
        );
        m.insert(
            ShortCarClass::FUN,
            CarClass::new(
                ShortCarClass::FUN,
                LongCarClass::Fun,
                ClassCategory::Miscellaneous_Category,
            ),
        );
        m.insert(
            ShortCarClass::FSAE,
            CarClass::new(
                ShortCarClass::FSAE,
                LongCarClass::Formula_SAE,
                ClassCategory::Miscellaneous_Category,
            ),
        );
        m
    };
}

pub fn get_car_class(car_class: &ShortCarClass) -> Option<CarClass> {
    if CLASS_MAP.contains_key(car_class) {
        Some(CLASS_MAP[car_class])
    } else {
        None
    }
}
