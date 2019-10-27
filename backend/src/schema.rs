table! {
    servers (id) {
        id -> Integer,
        title -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    servers,
    users,
);
