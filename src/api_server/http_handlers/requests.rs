use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameRequest {
    pub player_id: String,
}

pub fn decode<'a, T: Deserialize<'a>>(request: &'a str) -> T {
    serde_json::from_str(request).unwrap()
}
