pub fn get_optional_from_boolean_and_value<T>(option: bool, value: T) -> Option<T> {
    if option {
        Some(value)
    } else {
        None
    }
}