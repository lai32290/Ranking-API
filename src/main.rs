/*
/register_app
/<app_token>/top100
/<app_token>/score/<user_id>/add?score=100
/<app_token>/score/<user_id>
*/
#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate rocket;

use postgres::{Client, NoTls};
use rocket::State;
use std::sync::Mutex;

struct DB {
    client : Mutex<Client>
}

#[get("/")]
fn hello(db : State<DB>) -> &'static str {
    db.client.lock().unwrap().execute("INSERT INTO app (name) VALUES ('Test')",&[]);
    return "Hello, world!"
}

#[get("/2")]
fn hello2() -> &'static str {
    "Hello, world!2"
}

fn main() {

    let mut client = Client::connect("host=localhost user=postgres password=postgres", NoTls).unwrap();
    client.execute("INSERT INTO app (name) VALUES ('Test')",&[]);
    rocket::ignite().manage(DB{client:Mutex::new(client)}).mount("/", routes![hello,hello2]).launch();
}