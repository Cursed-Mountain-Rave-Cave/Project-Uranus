use serde::{Deserialize, Serialize};

pub fn decode<'a, T: Deserialize<'a>>(json: &'a str) -> T {
    serde_json::from_str(json).unwrap()
}

pub fn encode(value: &impl Serialize) -> String {
    serde_json::to_string(value).unwrap()
}
