use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Rocket, State};
use rocket::response::content::Html;
use super::schema::{Schema, Query, Context, Mutation, ConnectionPool};
use super::auth::AuthOption;

#[get("/graphiql")]
fn graphiql() -> Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler_authenticated(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    pool: State<ConnectionPool>,
    authentication: AuthOption,
) -> juniper_rocket::GraphQLResponse {
    let context = Context {
        pool: pool.clone(),
        authentication: Some(authentication)
    };
    
    request.execute(schema.inner(), &context)
}

#[get("/graphql?<request>", rank = 2)]
fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    pool: State<ConnectionPool>,
) -> juniper_rocket::GraphQLResponse {
    let context = Context { pool: pool.clone(), authentication: None };

    request.execute(schema.inner(), &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    pool: State<ConnectionPool>,
    authentication: AuthOption,
) -> juniper_rocket::GraphQLResponse {
    let context = Context {
        pool: pool.clone(),
        authentication: Some(authentication)
    };

    request.execute(schema.inner(), &context)
}

#[post("/graphql", data = "<request>", rank = 2)]
fn post_graphql_handler_authenticated(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
    pool: State<ConnectionPool>,
) -> juniper_rocket::GraphQLResponse {
    let context = Context { pool: pool.clone(), authentication: None };

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
            .mount("/", routes![
                graphiql,
                get_graphql_handler,
                get_graphql_handler_authenticated,
                post_graphql_handler,
                post_graphql_handler_authenticated,
            ]);

        Ok(rocket)
    }
}
