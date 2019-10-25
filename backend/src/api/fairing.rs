use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Rocket, State};
use rocket::response::content::Html;
use crate::api::schema::{Schema, Query, Context, Mutation};

#[get("/graphiql")]
fn graphiql() -> Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    context: State<Context>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema.inner(), &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    context: State<Context>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema.inner(), &context)
}

pub struct GraphqlFairing();

impl Fairing for GraphqlFairing {
    fn info(&self) -> Info {
        Info {
            name: "Juniper Route Provider",
            kind: Kind::Attach
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let rocket = rocket
            .manage(Schema::new(Query, Mutation {}))
            .mount("/", routes![graphiql, get_graphql_handler, post_graphql_handler]);

        Ok(rocket)
    }
}
