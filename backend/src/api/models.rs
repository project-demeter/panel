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
