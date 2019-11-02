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
    categories (category_id) {
        category_id -> Integer,
        category_name -> Text,
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
    objects (object_id) {
        object_id -> Integer,
        category_id -> Integer,
        object_name -> Text,
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

table! {
    named_victims (victim_id) {
        victim_id -> Integer,
        killmail_id -> Integer,
        damage_taken -> Integer,
        ship_id -> Integer,
        ship_name -> Text,
        character_id -> Nullable<Integer>,
        character_name -> Nullable<Text>,
        corporation_id -> Nullable<Integer>,
        corporation_name -> Nullable<Text>,
        alliance_id -> Nullable<Integer>,
        alliance_name -> Nullable<Text>,
        faction_id -> Nullable<Integer>,
    	faction_name -> Nullable<Text>,
    }
}

joinable!(attackers -> killmails (killmail_id));
joinable!(items -> killmails (killmail_id));
joinable!(objects -> categories (category_id));
joinable!(victims -> killmails (killmail_id));

allow_tables_to_appear_in_same_query!(
    attackers,
    categories,
    items,
    killmails,
    kills,
    objects,
    victims,
    named_victims,
);
