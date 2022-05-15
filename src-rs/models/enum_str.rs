#[macro_export]
macro_rules! enum_str {
    (enum $name:ident {
        $($variant:ident),*,
    }) => {
        #[wasm_bindgen]
        #[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
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
        }
    };
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use wasm_bindgen::prelude::*;

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
    }
}
