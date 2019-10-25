#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate juniper;
extern crate diesel;
extern crate walkdir;
extern crate regex;
extern crate dotenv;

mod nextjs;
mod api;

use diesel::SqliteConnection;

#[database("sqlite_database")]
pub struct DbConnection(SqliteConnection);

fn main() {
    rocket::ignite()
        .manage(DbConnection::fairing())
        .attach(nextjs::NextJsFairing())
        .attach(api::GraphqlFairing())
        .launch();
}
