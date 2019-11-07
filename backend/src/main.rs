#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod api;
mod nextjs;
mod schema;

use crate::api::schema::PrimaryConnection;

fn main() {
    dotenv::dotenv().expect("No .env file found");

    rocket::ignite()
        .attach(PrimaryConnection::fairing())
        .attach(nextjs::NextJsFairing())
        .attach(api::GraphqlFairing())
        .launch();
}
