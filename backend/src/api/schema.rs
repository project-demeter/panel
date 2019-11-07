use crate::api::auth::{self, AuthOption};
use crate::api::inputs::*;
use crate::api::models::*;
use chrono::prelude::*;
use crypto::sha2::Sha256;
use diesel;
use diesel::SqliteConnection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use juniper::graphql_object;
use juniper::FieldResult;
use rocket_contrib::database;

#[database("primary")]
pub struct PrimaryConnection(SqliteConnection);

pub struct Context {
    pub connection: PrimaryConnection,
    pub authentication: Option<AuthOption>,
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

        Ok(dsl::users.load::<User>(&*executor.context().connection)?)
    }

    field servers(&executor) -> FieldResult<Vec<Server>> {
        use crate::schema::servers::dsl;

        Ok(dsl::servers.load::<Server>(&*executor.context().connection)?)
    }

    field server(&executor, id: i32) -> FieldResult<Server> {
        use crate::schema::servers::dsl;

        Ok(
            dsl::servers.filter(dsl::id.eq(id))
                .first::<Server>(&*executor.context().connection)?
        )
    }
});

#[juniper::object(Context = Context)]
impl Mutation {
    fn register(context: &Context, user: NewUser) -> FieldResult<User> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(&user)
            .execute(&*context.connection)?;

        let inserted_user = users.order(id.desc()).first::<User>(&*context.connection)?;

        Ok(inserted_user)
    }

    fn createServer(context: &Context, server: NewServer) -> FieldResult<Server> {
        use crate::schema::servers::dsl::*;

        diesel::insert_into(servers)
            .values(&server)
            .execute(&*context.connection)?;

        let inserted_server = servers
            .order(id.desc())
            .first::<Server>(&*context.connection)?;

        Ok(inserted_server)
    }

    fn login(context: &Context, user: LoginInput) -> FieldResult<AuthToken> {
        use crate::schema::users::dsl;

        let user = dsl::users
            .filter(dsl::username.eq(user.username))
            .first::<User>(&*context.connection)?;

        let token = auth::create_token(&user);
        let token = token
            .signed(b"secret_key", Sha256::new())
            .map_err(|e| String::from("Could not sign JWT"))?;

        Ok(AuthToken {
            user,
            token,
            valid_until: Utc::now().naive_utc(),
        })
    }
}
