pub fn swap<T, E>(input: Option<Result<T, E>>) -> Result<Option<T>, E> {
    match input {
        None => Ok(None),
        Some(r) => match r {
            Err(e) => Err(e),
            Ok(v) => Ok(Some(v)),
        },
    }
}
