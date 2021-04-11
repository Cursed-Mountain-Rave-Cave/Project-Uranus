use serde::Serialize;

pub fn encode(response: &impl Serialize) -> String {
    serde_json::to_string(response).unwrap()
}

#[derive(Serialize)]
pub struct Play {
    pub code: String,
    pub session_id: String,
}
