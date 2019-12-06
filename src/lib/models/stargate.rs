use std::convert::From;
use crate::api;
use crate::schema::stargates;
use super::{Connection, QueryResult, Integer};

#[derive(Insertable)]
#[table_name = "stargates"]
pub struct Stargate {
    pub stargate_id: Integer,
    pub type_id: Integer,
    pub system_id: Integer,
    pub dst_stargate_id: Integer,
    pub dst_system_id: Integer,
}

impl From<&api::stargate::Stargate> for Stargate{
    fn from(src: &api::stargate::Stargate) -> Self {
        Self {
            stargate_id: src.stargate_id,
            type_id: src.type_id,
            system_id: src.system_id,
            dst_stargate_id: src.destination.stargate_id,
            dst_system_id: src.destination.system_id,
        }
    }
}

impl Stargate {
    pub fn save(conn: &Connection, object: &api::stargate::Stargate) -> QueryResult<bool>  {
        use crate::schema;
        use crate::diesel::RunQueryDsl;
        diesel::insert_into(schema::stargates::table)
                   .values(Self::from(object))
                   .execute(conn).and_then(|count| Ok(1 == count))
    }

    pub fn exist(conn: &Connection, id: &Integer) -> bool {
        use diesel::prelude::*;
        use crate::schema::stargates::dsl as table;
        table::stargates.find(id).select(table::stargate_id).first(conn) == Ok(*id)
    }
}

#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "stargates"]
pub struct SystemLink {
    pub system_id: Integer,
    pub dst_system_id: Integer,
}
impl SystemLink {
    // pub fn load(conn: &Connection, constellation_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Self>> {
    //     use diesel::prelude::*;

    //     let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
    //     info!("Load killmails after {}", &start);
    //     attackers::table.inner_join(named_killmails::table.on(named_killmails::killmail_id.eq(attackers::killmail_id)))
    //         .filter(named_killmails::killmail_time.gt(start))
    //         .filter(attackers::character_id.eq(character_id))
    //         .select((
    //             named_killmails::killmail_id,
    //             named_killmails::killmail_time,
    //             named_killmails::system_id,
    //             named_killmails::system_name,
    //             named_killmails::constellation_id,
    //             named_killmails::constellation_name,
    //             named_killmails::region_id,
    //             named_killmails::region_name,
    //          ))
    //         .order(named_killmails::killmail_time.desc())
    //         .load(conn)
    // }
}
