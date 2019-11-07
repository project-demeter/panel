use regex::Regex;
use rocket::{
    catch, catchers,
    fairing::{Fairing, Info, Kind},
    get,
    handler::{Handler, Outcome},
    http::Method,
    response::NamedFile,
    routes, Data, Request, Rocket, Route,
};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn get_static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")).join(path)).ok()
}

#[get("/<path..>", rank = 3)]
fn static_files(path: PathBuf) -> Option<NamedFile> {
    get_static_file(path)
}

#[get("/")]
fn index() -> Option<NamedFile> {
    get_static_file(PathBuf::from("index.html"))
}

#[catch(404)]
fn not_found() -> Option<NamedFile> {
    get_static_file(PathBuf::from("404.html"))
}

#[derive(Clone)]
struct DynamicRouteHandler(String);
impl Handler for DynamicRouteHandler {
    fn handle<'r>(&self, req: &'r Request, _: Data) -> Outcome<'r> {
        let buffer = PathBuf::from(&self.0.trim_start_matches('/'));
        Outcome::from(req, get_static_file(buffer))
    }
}

pub struct NextJsFairing();

impl Fairing for NextJsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Next.js Template Provider",
            kind: Kind::Attach,
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let directory = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

        let mut route_list = routes!(index, static_files);

        let replacement = Regex::new(r"\[(.+?)\]").unwrap();
        for entry in WalkDir::new(directory)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_name().to_string_lossy().ends_with(".html") {
                continue;
            }

            // The relative path of the file is equivalent to the URL this file is available from
            let relative_path = entry.path().to_string_lossy().replace(directory, "");

            let route = relative_path.trim_end_matches(".html");
            let route_with_rocket_arguments = replacement.replace_all(route, "<$1>").to_string();

            route_list.push(Route::new(
                Method::Get,
                route_with_rocket_arguments,
                DynamicRouteHandler(relative_path),
            ));
        }

        let rocket = rocket.mount("/", route_list).register(catchers![not_found]);

        Ok(rocket)
    }
}
