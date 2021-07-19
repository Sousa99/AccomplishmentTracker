use crate::controller;

#[get("/")]
pub fn index() -> &'static str {
    controller::say_something();
    return "Hello, world!";
}