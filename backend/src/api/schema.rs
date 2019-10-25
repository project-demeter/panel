use juniper::FieldResult;
use diesel::SqliteConnection;
use r2d2;
use diesel;

pub struct Context {
    pub pool: r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>,
}

impl juniper::Context for Context {}

#[derive(GraphQLObject)]
struct Server {
    id: String,
    title: String,
}

pub struct Query;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field server(&executor, id: String) -> FieldResult<Server> {
        Ok(Server {
            id,
            title: String::from("My Minecraft server")
        })
    }
});

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn createServer(context: &Context) -> FieldResult<Server> {
        // TODO: Implement database storing
        Ok(Server { id: String::from("abc-123"), title: String::from("Minecraft") })
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
