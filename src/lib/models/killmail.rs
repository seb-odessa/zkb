use std::convert::From;
use chrono::{Duration, Utc};

use crate::api;
use crate::schema::victims;
use crate::schema::attackers;
use crate::schema::killmails;
use crate::schema::named_killmails;
use super::{Integer, OptInteger, OptString, DateTime, Connection, QueryResult};


#[derive(Insertable)]
#[table_name = "killmails"]
pub struct Killmail {
    pub killmail_id: Integer,
    pub killmail_time: DateTime,
    pub solar_system_id: Integer,
    pub moon_id: Option<Integer>,
    pub war_id: Option<Integer>,
}
impl From<&api::Killmail> for Killmail {
    fn from(src: &api::Killmail) -> Self {
        Self {
            killmail_id: src.killmail_id,
            killmail_time: src.killmail_time.naive_utc(),
            solar_system_id: src.solar_system_id,
            moon_id: src.moon_id,
            war_id: src.war_id,
        }
    }
}

#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "named_killmails"]
pub struct KillmailNamed {
    pub killmail_id: Integer,
    pub killmail_time: DateTime,
    pub system_id: Integer,
    pub system_name: OptString,
    pub constellation_id: OptInteger,
    pub constellation_name: OptString,
    pub region_id: OptInteger,
    pub region_name: OptString,
}
impl KillmailNamed {

    pub fn get_id(&self, name: &str) -> Integer {
        match name {
            "id" => Some(self.killmail_id),
            "system" => Some(self.system_id),
            "constellation" => self.constellation_id.clone(),
            "region" => self.region_id.clone(),
            any => { warn!("Unknown pattern {}", any); Some(0)}
        }.unwrap_or_default()
    }

    pub fn get_name(&self, name: &str) -> String {
        match name {
            "system" => self.system_name.clone(),
            "constellation" => self.constellation_name.clone(),
            "region" => self.region_name.clone(),
            any => Some(format!("Unknown pattern {}", any))
        }.unwrap_or_default()
    }

    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        named_killmails::table.filter(named_killmails::killmail_id.eq(id)).first(conn)
    }

    pub fn load_system_history(conn: &Connection, system_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        named_killmails::table
            .filter(named_killmails::killmail_time.gt(start))
            .filter(named_killmails::system_id.eq(system_id))
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }

    pub fn load_constellation_history(conn: &Connection, constellation_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        named_killmails::table
            .filter(named_killmails::killmail_time.gt(start))
            .filter(named_killmails::constellation_id.eq(constellation_id))
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }

    pub fn load_region_history(conn: &Connection, region_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        named_killmails::table
            .filter(named_killmails::killmail_time.gt(start))
            .filter(named_killmails::region_id.eq(region_id))
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }

    pub fn load_character_history_wins(conn: &Connection, character_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;

        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        attackers::table.inner_join(named_killmails::table.on(named_killmails::killmail_id.eq(attackers::killmail_id)))
            .filter(named_killmails::killmail_time.gt(start))
            .filter(attackers::character_id.eq(character_id))
            .select((
                named_killmails::killmail_id,
                named_killmails::killmail_time,
                named_killmails::system_id,
                named_killmails::system_name,
                named_killmails::constellation_id,
                named_killmails::constellation_name,
                named_killmails::region_id,
                named_killmails::region_name,
             ))
            .distinct()
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }

    pub fn load_corporation_history_wins(conn: &Connection, corporation_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;

        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        attackers::table.inner_join(named_killmails::table.on(named_killmails::killmail_id.eq(attackers::killmail_id)))
            .filter(named_killmails::killmail_time.gt(start))
            .filter(attackers::corporation_id.eq(corporation_id))
            .select((
                named_killmails::killmail_id,
                named_killmails::killmail_time,
                named_killmails::system_id,
                named_killmails::system_name,
                named_killmails::constellation_id,
                named_killmails::constellation_name,
                named_killmails::region_id,
                named_killmails::region_name,
             ))
            .distinct()
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }

    pub fn load_alliance_history_wins(conn: &Connection, alliance_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;

        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        attackers::table.inner_join(named_killmails::table.on(named_killmails::killmail_id.eq(attackers::killmail_id)))
            .filter(named_killmails::killmail_time.gt(start))
            .filter(attackers::alliance_id.eq(alliance_id))
            .select((
                named_killmails::killmail_id,
                named_killmails::killmail_time,
                named_killmails::system_id,
                named_killmails::system_name,
                named_killmails::constellation_id,
                named_killmails::constellation_name,
                named_killmails::region_id,
                named_killmails::region_name,
             ))
            .distinct()
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }

    pub fn load_faction_history_wins(conn: &Connection, faction_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;

        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        attackers::table.inner_join(named_killmails::table.on(named_killmails::killmail_id.eq(attackers::killmail_id)))
            .filter(named_killmails::killmail_time.gt(start))
            .filter(attackers::faction_id.eq(faction_id))
            .select((
                named_killmails::killmail_id,
                named_killmails::killmail_time,
                named_killmails::system_id,
                named_killmails::system_name,
                named_killmails::constellation_id,
                named_killmails::constellation_name,
                named_killmails::region_id,
                named_killmails::region_name,
             ))
            .distinct()
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }

    pub fn load_character_history_losses(conn: &Connection, character_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;

        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        victims::table.inner_join(named_killmails::table.on(named_killmails::killmail_id.eq(victims::killmail_id)))
            .filter(named_killmails::killmail_time.gt(start))
            .filter(victims::character_id.eq(character_id))
            .select((
                named_killmails::killmail_id,
                named_killmails::killmail_time,
                named_killmails::system_id,
                named_killmails::system_name,
                named_killmails::constellation_id,
                named_killmails::constellation_name,
                named_killmails::region_id,
                named_killmails::region_name,
             ))
            .distinct()
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }

    pub fn load_corporation_history_losses(conn: &Connection, corporation_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;

        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        victims::table.inner_join(named_killmails::table.on(named_killmails::killmail_id.eq(victims::killmail_id)))
            .filter(named_killmails::killmail_time.gt(start))
            .filter(victims::corporation_id.eq(corporation_id))
            .select((
                named_killmails::killmail_id,
                named_killmails::killmail_time,
                named_killmails::system_id,
                named_killmails::system_name,
                named_killmails::constellation_id,
                named_killmails::constellation_name,
                named_killmails::region_id,
                named_killmails::region_name,
             ))
            .distinct()
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }

    pub fn load_alliance_history_losses(conn: &Connection, alliance_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;

        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        victims::table.inner_join(named_killmails::table.on(named_killmails::killmail_id.eq(victims::killmail_id)))
            .filter(named_killmails::killmail_time.gt(start))
            .filter(victims::alliance_id.eq(alliance_id))
            .select((
                named_killmails::killmail_id,
                named_killmails::killmail_time,
                named_killmails::system_id,
                named_killmails::system_name,
                named_killmails::constellation_id,
                named_killmails::constellation_name,
                named_killmails::region_id,
                named_killmails::region_name,
             ))
            .distinct()
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }

    pub fn load_faction_history_losses(conn: &Connection, faction_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;

        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        victims::table.inner_join(named_killmails::table.on(named_killmails::killmail_id.eq(victims::killmail_id)))
            .filter(named_killmails::killmail_time.gt(start))
            .filter(victims::faction_id.eq(faction_id))
            .select((
                named_killmails::killmail_id,
                named_killmails::killmail_time,
                named_killmails::system_id,
                named_killmails::system_name,
                named_killmails::constellation_id,
                named_killmails::constellation_name,
                named_killmails::region_id,
                named_killmails::region_name,
             ))
            .distinct()
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }

    pub fn load_system_history_count(conn: &Connection, system_id: &Integer, minutes: &Integer) -> QueryResult<i64> {
        use diesel::prelude::*;
        use diesel::dsl::count;
        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails count after {}", &start);
        named_killmails::table
            .filter(named_killmails::killmail_time.gt(start))
            .filter(named_killmails::system_id.eq(system_id))
            .select(count(named_killmails::killmail_id))
            .first(conn)
    }

    pub fn load_constellation_history_count(conn: &Connection, constellation_id: &Integer, minutes: &Integer) -> QueryResult<i64> {
        use diesel::prelude::*;
        use diesel::dsl::count;
        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails count after {}", &start);
        named_killmails::table
            .filter(named_killmails::killmail_time.gt(start))
            .filter(named_killmails::constellation_id.eq(constellation_id))
            .select(count(named_killmails::killmail_id))
            .first(conn)
    }

    pub fn load_region_history_count(conn: &Connection, region_id: &Integer, minutes: &Integer) -> QueryResult<i64> {
        use diesel::prelude::*;
        use diesel::dsl::count;
        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails count after {}", &start);
        named_killmails::table
            .filter(named_killmails::killmail_time.gt(start))
            .filter(named_killmails::region_id.eq(region_id))
            .select(count(named_killmails::killmail_id))
            .first(conn)
    }


    pub fn load_ids_for_last_minutes(conn: &Connection, system_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Integer>> {
        use diesel::prelude::*;

        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        named_killmails::table
            .filter(named_killmails::killmail_time.gt(start))
            .filter(named_killmails::system_id.eq(system_id))
            .select(named_killmails::killmail_id)
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }
}