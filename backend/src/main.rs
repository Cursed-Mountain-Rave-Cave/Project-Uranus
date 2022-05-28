#[macro_use] extern crate rocket;

mod router;
pub mod user;
pub mod session;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", router::get_routes())
}