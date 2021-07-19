#[macro_use] extern crate rocket;

pub mod router;
pub mod controller;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![router::index])
}