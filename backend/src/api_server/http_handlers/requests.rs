use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameRequest {
    pub player_id: String,
}
