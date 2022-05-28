use rocket::{Route};

pub fn get_routes() -> Vec<Route> {
    routes![start_game]
}

#[get("/start_game")]
pub fn start_game() {
    
}