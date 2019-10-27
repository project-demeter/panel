use crate::schema::users;
use crate::schema::servers;

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
