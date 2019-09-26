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
    kills (killmail_id) {
        killmail_id -> Integer,
        killmail_hash -> Text,
        killmail_date -> Date,
    }
}

allow_tables_to_appear_in_same_query!(
    killmails,
    kills,
);
