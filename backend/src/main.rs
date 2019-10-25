#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate diesel;
extern crate walkdir;
extern crate regex;
extern crate dotenv;

pub mod rocket_fairings;

use diesel::SqliteConnection;

#[database("sqlite_database")]
pub struct DbConnection(SqliteConnection);

fn main() {
    rocket::ignite()
        .manage(DbConnection::fairing())
        .attach(rocket_fairings::NextJs())
        .attach(rocket_fairings::Juniper())
        .launch();
}
