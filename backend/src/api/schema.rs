use juniper::FieldResult;
use diesel::SqliteConnection;
use r2d2;
use diesel;
use super::models::*;
use super::inputs::*;
use diesel::RunQueryDsl;

pub struct Context {
    pub pool: r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>,
}

impl juniper::Context for Context {}

pub struct Query;

pub struct Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    field users(&executor) -> FieldResult<Vec<User>> {
        use crate::schema::users::dsl;

        let connection = executor.context().pool.get().unwrap();

        Ok(dsl::users.load::<User>(&connection)?)
    }

    field servers(&executor) -> FieldResult<Vec<Server>> {
        Ok(vec![])
    }

    field server(&executor, id: i32) -> FieldResult<Server> {
        Ok(Server {
            id,
            title: String::from("My Minecraft server")
        })
    }
});

#[juniper::object(Context = Context)]
impl Mutation {
    fn register(context: &Context, new_user: NewUser) -> FieldResult<Option<User>> {
        use crate::schema::users::dsl::*;

        let connection = context.pool.get().unwrap();
        diesel::insert_into(users).values(&new_user).execute(&connection);
        Ok(None)
    }

    fn createServer(context: &Context) -> FieldResult<Server> {
        // TODO: Implement database storing
        Ok(Server { id: 100, title: String::from("Minecraft") })
    }
}
