use serde::Serialize;

#[derive(Serialize)]
pub struct Play {
    pub code: String,
    pub session_id: String,
}
