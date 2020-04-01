/*
/register_app
/<app_token>/top100
/<app_token>/score/<user_id>/add?score=100
/<app_token>/score/<user_id>
*/
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use crate::dao::app_dao;

mod dao;

use postgres::{NoTls, Client};
use r2d2_postgres::{PostgresConnectionManager, r2d2};
use rocket::State;
use std::sync::{Mutex, Arc};
use r2d2::Pool;
use serde::Serialize;
use rocket_contrib::json::{Json, JsonValue};
use crate::dao::app_dao::AppDao;

struct DAO {
    app: Arc<AppDao>
}


#[derive(Serialize, Deserialize)]
struct AppRequest {
    name: String
}

#[post("/register_app", format = "json", data = "<request>")]
fn hello(dao: State<DAO>, request: Json<AppRequest>) -> JsonValue {
    if dao.app.exists(request.0.name.clone()) {
        return json!({ "status": "NOK" });
    }
    return match dao.app.create(request.0.name.clone()) {
        None => { json!({ "status": "NOK" }) }
        Some(id) => {
            json!({ "status":id.as_str()})
        }
    };
}

#[get("/2")]
fn hello2() -> &'static str {
    "Hello, world!2"
}

fn main() {
    let manager = PostgresConnectionManager::new(
        "host=localhost user=postgres password=postgres".parse().unwrap(),
        NoTls,
    );
    let pool = r2d2::Pool::new(manager).unwrap();

    rocket::ignite()
        .manage(DAO { app: Arc::new(AppDao::new(pool)) })
        .mount("/", routes![hello,hello2])
        .launch();
}