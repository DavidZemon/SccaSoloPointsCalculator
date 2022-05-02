use float_cmp;
use float_cmp::F32Margin;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct LapTime {
    pub raw: Option<f32>,
    pub time: Option<f32>,
    pub cones: u8,
    pub dnf: bool,
    pub rerun: bool,
    pub dsq: bool,
    pub dns: bool,
}

impl PartialOrd<Self> for LapTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.time.is_none() && other.time.is_none() {
            Some(Ordering::Equal)
        } else if self.time.is_none() {
            Some(Ordering::Greater)
        } else if other.time.is_none() {
            Some(Ordering::Less)
        } else {
            let self_time = self.time.unwrap();
            let other_time = other.time.unwrap();

            if float_cmp::ApproxEq::approx_eq(self_time, other_time, F32Margin::default()) {
                Some(Ordering::Equal)
            } else {
                self_time.partial_cmp(&other_time)
            }
        }
    }
}

impl Ord for LapTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialEq for LapTime {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
            && self.time == other.time
            && self.cones == other.cones
            && self.dnf == other.dnf
            && self.rerun == other.rerun
            && self.dsq == other.dsq
            && self.dns == other.dns
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for LapTime {}

#[wasm_bindgen]
pub enum Penalty {
    DNF,
    RRN,
    DSQ,
    DNS,
}

#[wasm_bindgen]
impl LapTime {
    #[wasm_bindgen(constructor)]
    pub fn new(raw_time: f32, cones: u8, penalty: Option<Penalty>) -> LapTime {
        match penalty {
            None => LapTime {
                raw: Some(raw_time),
                time: Some(raw_time + (cones as f32) * 2.),
                cones,
                dnf: false,
                rerun: false,
                dsq: false,
                dns: false,
            },
            Some(Penalty::DNF) => LapTime {
                raw: None,
                time: None,
                cones: 0,
                dnf: true,
                rerun: false,
                dsq: false,
                dns: false,
            },
            Some(Penalty::RRN) => LapTime {
                raw: None,
                time: None,
                cones: 0,
                dnf: false,
                rerun: true,
                dsq: false,
                dns: false,
            },
            Some(Penalty::DSQ) => LapTime {
                raw: None,
                time: None,
                cones: 0,
                dnf: false,
                rerun: false,
                dsq: true,
                dns: false,
            },
            Some(Penalty::DNS) => LapTime {
                raw: None,
                time: None,
                cones: 0,
                dnf: false,
                rerun: false,
                dsq: false,
                dns: true,
            },
        }
    }

    #[allow(non_snake_case)]
    pub fn toString(
        &self,
        pax_multiplier: Option<f32>,
        display_cone_count: Option<bool>,
    ) -> String {
        if self.dnf {
            String::from("DNF")
        } else if self.rerun {
            String::from("Re-run")
        } else if self.dsq {
            String::from("DSQ")
        } else if self.dns {
            String::from("DNS")
        } else {
            let result = format!("{:.3}", self.time.unwrap() * pax_multiplier.unwrap_or(1.));
            if display_cone_count.unwrap_or(true) && self.cones != 0 {
                format!("{} ({})", result, self.cones)
            } else {
                result
            }
        }
    }

    pub fn add(&self, rhs: LapTime) -> LapTime {
        if self.dnf || self.rerun || self.dsq || self.dns {
            self.clone()
        } else if rhs.dnf || rhs.rerun || rhs.dsq || rhs.dns {
            rhs.clone()
        } else {
            LapTime::new(
                self.raw.unwrap() + rhs.raw.unwrap(),
                self.cones + rhs.cones,
                None,
            )
        }
    }

    pub fn compare(self: &LapTime, rhs: &LapTime) -> i8 {
        match self.cmp(rhs) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        }
    }

    pub fn dsq() -> LapTime {
        LapTime::new(0., 0, Some(Penalty::DSQ))
    }

    pub fn dns() -> LapTime {
        LapTime::new(0., 0, Some(Penalty::DNS))
    }
}

impl fmt::Display for LapTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.toString(None, None))
    }
}
