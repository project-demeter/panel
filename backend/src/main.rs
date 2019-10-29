#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate juniper;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate rocket_contrib;
extern crate walkdir;
extern crate regex;
extern crate dotenv;
extern crate r2d2;
extern crate crypto;
extern crate chrono;

mod nextjs;
mod api;
mod schema;

use api::schema::ConnectionPool;
use std::env;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

fn main() {
    dotenv::dotenv().expect("No .env file found");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::new(database_url);
    let pool: ConnectionPool = Pool::new(manager).unwrap();

    rocket::ignite()
        .manage(pool)
        .attach(nextjs::NextJsFairing())
        .attach(api::GraphqlFairing())
        .launch();
}
