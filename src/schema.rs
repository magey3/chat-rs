table! {
    messages (id) {
        id -> Int4,
        content -> Text,
        userid -> Int4,
        time -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
