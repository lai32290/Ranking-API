/*
/register_app
/<app_token>/top100
/<app_token>/score/<user_id>/add?score=100
/<app_token>/score/<user_id>
*/
#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}
#[get("/2")]
fn hello2() -> &'static str {
    "Hello, world!2"
}

fn main() {
    rocket::ignite().mount("/", routes![hello,hello2]).launch();
}