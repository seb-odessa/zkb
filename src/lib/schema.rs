table! {
    dates (id) {
        id -> Integer,
        year -> Integer,
        month -> Integer,
        day -> Integer,
    }
}

table! {
    kills (id) {
        id -> Integer,
        hash -> Text,
        date -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    dates,
    kills,
);
