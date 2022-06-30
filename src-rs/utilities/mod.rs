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
