use crate::schema::{servers, users};
use juniper::GraphQLInputObject;

#[derive(Insertable, GraphQLInputObject)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Insertable, GraphQLInputObject)]
#[table_name = "servers"]
pub struct NewServer {
    pub title: String,
}

#[derive(GraphQLInputObject)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}
