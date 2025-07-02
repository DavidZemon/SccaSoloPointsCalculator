use crate::enum_str;

enum_str! {
    enum LongCarClass {
        Pro,
        Super_Street,
        A_Street,
        B_Street,
        C_Street,
        D_Street,
        E_Street,
        F_Street,
        G_Street,
        H_Street,
        Super_Street_Touring,
        A_Street_Touring,
        B_Street_Touring,
        C_Street_Touring,
        D_Street_Touring,
        E_Street_Touring,
        G_Street_Touring,
        Super_Street_Prepared,
        A_Street_Prepared,
        B_Street_Prepared,
        C_Street_Prepared,
        D_Street_Prepared,
        E_Street_Prepared,
        F_Street_Prepared,
        Super_Street_Modified,
        Street_Modified,
        Street_Modified_Front_Wheel_Drive,
        X_Prepared,
        B_Prepared,
        C_Prepared,
        D_Prepared,
        E_Prepared,
        F_Prepared,
        A_Modified,
        B_Modified,
        C_Modified,
        D_Modified,
        E_Modified,
        F_Modified,
        Kart_Modified,
        Classic_American_Muscle_Contemporary,
        Classic_American_Muscle_Traditional,
        Classic_American_Muscle_Sports,
        Xtreme_Street_A,
        Xtreme_Street_B,
        Xtreme_Street_Ultra,
        Electric_Vehicle_Xtreme,
        Solo_Spec_Coupe,
        Club_Spec_Mustang,
        Club_Spec_Miata,
        Super_Street_Ladies,
        A_Street_Ladies,
        B_Street_Ladies,
        C_Street_Ladies,
        D_Street_Ladies,
        E_Street_Ladies,
        F_Street_Ladies,
        G_Street_Ladies,
        H_Street_Ladies,
        Super_Street_Touring_Ladies,
        A_Street_Touring_Ladies,
        B_Street_Touring_Ladies,
        C_Street_Touring_Ladies,
        D_Street_Touring_Ladies,
        E_Street_Touring_Ladies,
        G_Street_Touring_Ladies,
        Super_Street_Prepared_Ladies,
        A_Street_Prepared_Ladies,
        B_Street_Prepared_Ladies,
        C_Street_Prepared_Ladies,
        D_Street_Prepared_Ladies,
        E_Street_Prepared_Ladies,
        F_Street_Prepared_Ladies,
        Super_Street_Modified_Ladies,
        Street_Modified_Ladies,
        Street_Modified_Front_Wheel_Drive_Ladies,
        X_Prepared_Ladies,
        B_Prepared_Ladies,
        C_Prepared_Ladies,
        D_Prepared_Ladies,
        E_Prepared_Ladies,
        F_Prepared_Ladies,
        A_Modified_Ladies,
        B_Modified_Ladies,
        C_Modified_Ladies,
        D_Modified_Ladies,
        E_Modified_Ladies,
        F_Modified_Ladies,
        Kart_Modified_Ladies,
        Classic_American_Muscle_Contemporary_Ladies,
        Classic_American_Muscle_Traditional_Ladies,
        Classic_American_Muscle_Sports_Ladies,
        Xtreme_Street_A_Ladies,
        Xtreme_Street_B_Ladies,
        Xtreme_Street_Ultra_Ladies,
        Electric_Vehicle_Xtreme_Ladies,
        Solo_Spec_Coupe_Ladies,
        Club_Spec_Mustang_Ladies,
        Club_Spec_Miata_Ladies,
        Fun,
        Formula_SAE,
        Formula_SAE_Ladies,
    }
}

#[wasm_bindgen]
pub fn to_display_name(long_car_class: LongCarClass) -> String {
    long_car_class
        .name()
        .replace("Front_Wheel_Drive", "Front-Wheel-Drive")
        .replace('_', " ")
}

#[cfg(test)]
mod test {
    use crate::enums::long_car_class::to_display_name;
    use crate::enums::long_car_class::LongCarClass;

    #[test]
    fn to_display_name_should_convert_correctly() {
        assert_eq!(to_display_name(LongCarClass::Super_Street), "Super Street".to_string());
        assert_eq!(to_display_name(LongCarClass::Formula_SAE), "Formula SAE".to_string());
        assert_eq!(
            to_display_name(LongCarClass::Street_Modified_Front_Wheel_Drive),
            "Street Modified Front-Wheel-Drive".to_string()
        );
    }
}
