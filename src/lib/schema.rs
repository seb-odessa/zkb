table! {
    dates (id) {
        id -> Integer,
        year -> Integer,
        month -> Integer,
        day -> Integer,
    }
}

table! {
    killmails (killmail_id) {
        killmail_id -> Integer,
        killmail_time -> Text,
        solar_system_id -> Integer,
        moon_id -> Nullable<Integer>,
        war_id -> Nullable<Integer>,
        victim_id -> Integer,
        attackers_id -> Integer,
    }
}

table! {
    kills (id) {
        id -> Integer,
        hash -> Binary,
        date_id -> Integer,
    }
}

joinable!(kills -> dates (date_id));

allow_tables_to_appear_in_same_query!(
    dates,
    killmails,
    kills,
);
