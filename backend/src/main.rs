#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod routes;

fn main() {
    rocket::ignite()
        .mount("/", routes::get_routes())
        .register(routes::get_catchers())
        .launch();
}
