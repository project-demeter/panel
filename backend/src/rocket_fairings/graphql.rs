use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Rocket, State};
use rocket::response::content::Html;

#[get("/graphiql")]
fn graphiql() -> Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

/*
#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<()>,
    request: juniper_rocket::GraphQLRequest
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<()>,
    request: juniper_rocket::GraphQLRequest
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}
*/

pub struct Juniper();

impl Fairing for Juniper {
    fn info(&self) -> Info {
        Info {
            name: "Juniper Route Provider",
            kind: Kind::Attach
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let rocket = rocket
            .mount("/", routes![graphiql]);

        Ok(rocket)
    }
}
