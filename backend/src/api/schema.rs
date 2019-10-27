use juniper::FieldResult;
use diesel::SqliteConnection;
use r2d2;
use diesel;
use super::models::*;
use super::inputs::*;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};

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
        use crate::schema::servers::dsl;

        let connection = executor.context().pool.get().unwrap();
        Ok(dsl::servers.load::<Server>(&connection)?)
    }

    field server(&executor, id: i32) -> FieldResult<Server> {
        use crate::schema::servers::dsl;

        let connection = executor.context().pool.get().unwrap();
        Ok(
            dsl::servers.filter(dsl::id.eq(id))
                .first::<Server>(&connection)?
        )
    }
});

#[juniper::object(Context = Context)]
impl Mutation {
    fn register(context: &Context, user: NewUser) -> FieldResult<User> {
        use crate::schema::users::dsl::*;

        let connection = context.pool.get().unwrap();
        diesel::insert_into(users).values(&user).execute(&connection)?;

        let inserted_user = users.order(id.desc())
            .first::<User>(&connection)?;

        Ok(inserted_user)
    }

    fn createServer(context: &Context, server: NewServer) -> FieldResult<Server> {
        use crate::schema::servers::dsl::*;

        let connection = context.pool.get().unwrap();
        diesel::insert_into(servers).values(&server).execute(&connection)?;

        let inserted_server = servers.order(id.desc())
            .first::<Server>(&connection)?;

        Ok(inserted_server)
    }
}
