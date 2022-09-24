use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! enum_str {
    (enum $name:ident {
        $($variant:ident),*,
    }) => {
        use serde::{Deserialize, Serialize};
        use strum_macros::EnumIter;
        use strum::IntoEnumIterator;
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        #[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Ord, PartialOrd, EnumIter)]
        #[allow(non_camel_case_types)]
        pub enum $name {
            $($variant),*
        }

        #[allow(dead_code)]
        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }

            pub fn parse(name: &str) -> Option<$name> {
                $name::iter().filter(|candidate| candidate.name() == name).next()
            }
        }
    };
}

pub fn swap<T, E>(input: Option<Result<T, E>>) -> Result<Option<T>, E> {
    match input {
        None => Ok(None),
        Some(r) => match r {
            Err(e) => Err(e),
            Ok(v) => Ok(Some(v)),
        },
    }
}

pub fn events_to_count(event_count: usize) -> usize {
    if event_count < 4 {
        event_count
    } else {
        (((event_count as f32) / 2.).ceil() as usize) + 1
    }
}

#[cfg(test)]
mod test {
    use crate::utilities::events_to_count;

    enum_str! {
        enum ActualEnum {
            First,
            Second,
            Multi_Word,
        }
    }

    #[test]
    fn should_create_name_method() {
        assert_eq!(ActualEnum::First.name(), "First".to_string());
        assert_eq!(ActualEnum::Second.name(), "Second".to_string());
        assert_eq!(ActualEnum::Multi_Word.name(), "Multi_Word".to_string());
        assert_eq!(ActualEnum::parse("Bogus"), None);
        assert_eq!(ActualEnum::parse("First"), Some(ActualEnum::First));
        assert_eq!(ActualEnum::parse("Second"), Some(ActualEnum::Second));
        assert_eq!(
            ActualEnum::parse("Multi_Word"),
            Some(ActualEnum::Multi_Word)
        );
    }

    #[test]
    fn test_events_to_count() {
        assert_eq!(events_to_count(1), 1);
        assert_eq!(events_to_count(2), 2);
        assert_eq!(events_to_count(3), 3);
        assert_eq!(events_to_count(4), 3);
        assert_eq!(events_to_count(5), 4);
        assert_eq!(events_to_count(6), 4);
        assert_eq!(events_to_count(7), 5);
        assert_eq!(events_to_count(8), 5);
        assert_eq!(events_to_count(9), 6);
        assert_eq!(events_to_count(10), 6);
        assert_eq!(events_to_count(11), 7);
        assert_eq!(events_to_count(12), 7);
    }
}
