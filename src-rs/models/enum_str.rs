#[macro_export]
macro_rules! enum_str {
    (enum $name:ident {
        $($variant:ident),*,
    }) => {
        use serde::{Deserialize, Serialize};
        use strum_macros::EnumIter;
        use strum::IntoEnumIterator;
        use wasm_bindgen::prelude::*;

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

#[cfg(test)]
mod test {
    enum_str! {
        enum ActualEnum {
            First,
            Second,
            Multi_Word,
        }
    }

    #[test]
    fn should_create_name_method() {
        assert_eq!(ActualEnum::First.name(), String::from("First"));
        assert_eq!(ActualEnum::Second.name(), String::from("Second"));
        assert_eq!(ActualEnum::Multi_Word.name(), String::from("Multi_Word"));
        assert_eq!(ActualEnum::parse("Bogus"), None);
        assert_eq!(ActualEnum::parse("First"), Some(ActualEnum::First));
        assert_eq!(ActualEnum::parse("Second"), Some(ActualEnum::Second));
        assert_eq!(
            ActualEnum::parse("Multi_Word"),
            Some(ActualEnum::Multi_Word)
        );
    }
}
