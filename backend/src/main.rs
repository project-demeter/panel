#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate walkdir;
extern crate regex;

pub mod routes;

fn main() {
    let routes = routes::get_routes().expect("Could not find static frontend files");

    rocket::ignite()
        .mount("/", routes)
        .register(routes::get_catchers())
        .launch();
}
