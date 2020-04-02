/*
/register_app
/<app_token>/top100
/<app_token>/score/<user_id>/add?score=100
/<app_token>/score/<user_id>
*/
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use postgres::NoTls;
use r2d2_postgres::{PostgresConnectionManager, r2d2};

use crate::dao::app_dao::AppDao;

mod dao;
mod model;
mod controller;

fn main() {
    let manager = PostgresConnectionManager::new(
        "host=localhost user=postgres password=postgres".parse().unwrap(),
        NoTls,
    );
    let pool = r2d2::Pool::new(manager).unwrap();

    rocket::ignite()
        .manage(AppDao::new(pool))
        .mount("/", routes![crate::controller::app::create])
        .launch();
}