use chrono::prelude::*;

#[derive(Queryable, GraphQLObject, Debug)]
pub struct Server {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, GraphQLObject, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[graphql(skip)]
    pub password: String,
}

#[derive(GraphQLObject)]
pub struct AuthToken {
    pub user: User,
    pub token: String,
    pub valid_until: NaiveDateTime,
}
