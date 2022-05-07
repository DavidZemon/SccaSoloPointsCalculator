#[macro_export]
macro_rules! enum_str {
    (enum $name:ident {
        $($variant:ident),*,
    }) => {
        #[wasm_bindgen]
        #[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
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
