use std::collections::HashMap;

pub trait ToMap {
    fn to_map(self) -> HashMap<String, String>;
}

// {"name": "bri", "age": "28"}

pub trait FromMap {
    fn from_map(map: HashMap<String, String>) -> Self;
}
