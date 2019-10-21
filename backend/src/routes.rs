use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use rocket::{Route, Catcher, routes, catchers};

#[get("/<path..>")]
fn static_file(path: PathBuf) -> Option<NamedFile> {
    let mut filename = path.clone();
    if filename.extension().is_none() {
        filename.set_extension("html");
    }

    NamedFile::open(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")).join(filename)).ok()
}

#[get("/")]
fn index() -> Option<NamedFile> {
    static_file(PathBuf::from("index.html"))
}

#[catch(404)]
fn not_found() -> Option<NamedFile> {
    static_file(PathBuf::from("404.html"))
}

pub fn get_routes() -> Vec<Route> {
    routes![
        index,
        static_file
    ]
}

pub fn get_catchers() -> Vec<Catcher> {
    catchers![not_found]
}