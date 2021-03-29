table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
        user_id -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
