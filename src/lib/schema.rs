table! {
    attackers (attacker_id) {
        attacker_id -> Integer,
        killmail_id -> Integer,
        security_status -> Float,
        final_blow -> Bool,
        damage_done -> Integer,
        ship_type_id -> Nullable<Integer>,
        alliance_id -> Nullable<Integer>,
        character_id -> Nullable<Integer>,
        corporation_id -> Nullable<Integer>,
        faction_id -> Nullable<Integer>,
        weapon_type_id -> Nullable<Integer>,
    }
}

table! {
    items (item_id) {
        item_id -> Integer,
        killmail_id -> Integer,
        item_type_id -> Integer,
        singleton -> Integer,
        flag -> Integer,
        quantity_destroyed -> Nullable<Integer>,
        quantity_dropped -> Nullable<Integer>,
    }
}

table! {
    killmails (killmail_id) {
        killmail_id -> Integer,
        killmail_time -> Timestamp,
        solar_system_id -> Integer,
        moon_id -> Nullable<Integer>,
        war_id -> Nullable<Integer>,
    }
}

table! {
    kills (killmail_id) {
        killmail_id -> Integer,
        killmail_hash -> Text,
        killmail_date -> Date,
    }
}

table! {
    victims (victim_id) {
        victim_id -> Integer,
        killmail_id -> Integer,
        ship_type_id -> Integer,
        damage_taken -> Integer,
        alliance_id -> Nullable<Integer>,
        character_id -> Nullable<Integer>,
        corporation_id -> Nullable<Integer>,
        faction_id -> Nullable<Integer>,
    }
}

joinable!(attackers -> killmails (killmail_id));
joinable!(items -> killmails (killmail_id));
joinable!(victims -> killmails (killmail_id));

allow_tables_to_appear_in_same_query!(
    attackers,
    items,
    killmails,
    kills,
    victims,
);
