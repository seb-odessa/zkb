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
    named_attackers (attacker_id) {
        attacker_id -> Integer,
        killmail_id -> Integer,
        security_status -> Float,
        final_blow -> Bool,
        damage_done -> Integer,
        ship_id -> Nullable<Integer>,
        ship_name -> Nullable<Text>,
        character_id -> Nullable<Integer>,
        character_name -> Nullable<Text>,
        corporation_id -> Nullable<Integer>,
        corporation_name -> Nullable<Text>,
        alliance_id -> Nullable<Integer>,
        alliance_name -> Nullable<Text>,
        faction_id -> Nullable<Integer>,
    	faction_name -> Nullable<Text>,
        weapon_id -> Nullable<Integer>,
        weapon_name -> Nullable<Text>,
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
    named_items (item_id) {
        item_id -> Integer,
        killmail_id -> Integer,
        item_type_id -> Integer,
        item_type_name -> Nullable<Text>,
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
    named_killmails (killmail_id) {
        killmail_id -> Integer,
        killmail_time -> Timestamp,
        system_id -> Integer,
        system_name -> Nullable<Text>,
        constellation_id -> Nullable<Integer>,
        constellation_name -> Nullable<Text>,
        region_id -> Nullable<Integer>,
        region_name -> Nullable<Text>,
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
        ship_name -> Nullable<Text>,
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

table! {
    systems (system_id) {
        system_id -> Integer,
        star_id -> Nullable<Integer>,
        security_status -> Float,
        constellation_id -> Integer,
    }
}

table! {
    observatories (system_id) {
        system_id -> Integer,
    }
}

table! {
    named_systems (system_id) {
    	system_id -> Integer,
    	system_name -> Nullable<Text>,
    	constellation_id -> Integer,
    	constellation_name -> Nullable<Text>,
    	region_id -> Integer,
    	region_name -> Nullable<Text>,
    	security_status -> Float,
    	observatory -> Nullable<Text>,
    }
}

table! {
    constellations (constellation_id) {
        constellation_id -> Integer,
        type_id -> Integer,
		region_id -> Integer,
    }
}

table! {
    stargates (stargate_id) {
        stargate_id -> Integer,
        type_id -> Integer,
		system_id -> Integer,
		dst_stargate_id -> Integer,
		dst_system_id -> Integer,
    }
}

table! {
    neighbors_systems (own_id, neighbor_id) {
        own_id -> Integer,
        own_name -> Nullable<Text>,
		neighbor_id -> Integer,
		neighbor_name -> Nullable<Text>,
    }
}

table! {
    neighbors_constellations (own_id, neighbor_id) {
        own_id -> Integer,
        own_name -> Nullable<Text>,
		neighbor_id -> Integer,
		neighbor_name -> Nullable<Text>,
    }
}

table! {
    neighbors_regions (own_id, neighbor_id) {
        own_id -> Integer,
        own_name -> Nullable<Text>,
		neighbor_id -> Integer,
		neighbor_name -> Nullable<Text>,
    }
}

table! {
    named_constellations (constellation_id) {
        constellation_id -> Integer,
        constellation_name -> Nullable<Text>,
		region_id -> Integer,
		region_name -> Nullable<Text>,
    }
}

table! {
    observatory_path (s0_id, s2_id, s3_id, s4_id, s5_id) {
        s0_id -> Integer,
        s0_name -> Nullable<Text>,
        s1_id -> Integer,
        s1_name -> Nullable<Text>,
        s1_jo -> Bool,
        s2_id -> Integer,
        s2_name -> Nullable<Text>,
        s2_jo -> Bool,
        s3_id -> Integer,
        s3_name -> Nullable<Text>,
        s3_jo -> Bool,
        s4_id -> Integer,
        s4_name -> Nullable<Text>,
        s4_jo -> Bool,
        s5_id -> Integer,
        s5_name -> Nullable<Text>,
        s5_jo -> Bool,
    }
}

table! {
    combat_participants (killmail_id, killmail_id) {
        killmail_id -> Integer,
        killmail_time -> Timestamp,
        victim_character_id -> Nullable<Integer>,
        victim_character_name -> Nullable<Text>,
        victim_corporation_id -> Nullable<Integer>,
        victim_corporation_name -> Nullable<Text>,
        victim_alliance_id -> Nullable<Integer>,
        victim_alliance_name -> Nullable<Text>,
        victim_faction_id -> Nullable<Integer>,
    	victim_faction_name -> Nullable<Text>,
        attacker_character_id -> Nullable<Integer>,
        attacker_character_name -> Nullable<Text>,
        attacker_corporation_id -> Nullable<Integer>,
        attacker_corporation_name -> Nullable<Text>,
        attacker_alliance_id -> Nullable<Integer>,
        attacker_alliance_name -> Nullable<Text>,
        attacker_faction_id -> Nullable<Integer>,
    	attacker_faction_name -> Nullable<Text>,
    }
}

table! {
    combat_items (killmail_id, killmail_id) {
        killmail_id -> Integer,
        killmail_time -> Timestamp,
        victim_ship_id -> Nullable<Integer>,
        victim_ship_name -> Nullable<Text>,
        attacker_ship_id -> Nullable<Integer>,
        attacker_ship_name -> Nullable<Text>,
        attacker_weapon_id -> Nullable<Integer>,
        attacker_weapon_name -> Nullable<Text>,
    }
}

joinable!(attackers -> killmails (killmail_id));
joinable!(items -> killmails (killmail_id));
joinable!(objects -> categories (category_id));
joinable!(victims -> killmails (killmail_id));
joinable!(systems -> constellations (constellation_id));
joinable!(stargates -> systems (system_id));

allow_tables_to_appear_in_same_query!(
    attackers,
    categories,
    items,
    killmails,
    kills,
    objects,
    victims,
    named_items,
    named_victims,
    named_attackers,
    named_killmails,
    named_constellations,
    neighbors_constellations,
);
