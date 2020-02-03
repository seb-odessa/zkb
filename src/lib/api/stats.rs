use crate::api::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Entity {
    Character(i32),
    Corporation(i32),
    Alliance(i32),
    Faction(i32),
    Ship(i32),
    Group(i32),
    System(i32),
    Region(i32),
}

pub type Groups = HashMap<IntRequired, GroupStat>;
pub type Months = HashMap<IntRequired, MonthStat>;


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Stats {
    pub id: IntRequired,

    #[serde(alias = "type")]            pub entity: String,
    #[serde(alias = "dangerRatio")]     pub danger_ratio: IntRequired,
    #[serde(alias = "gangRatio")]       pub gang_ratio: IntRequired,
    #[serde(alias = "shipsDestroyed")]  pub ship_destroyed: IntOptional,
    #[serde(alias = "pointsDestroyed")] pub points_destroyed: IntOptional,
    #[serde(alias = "iskDestroyed")]    pub isk_destroyed: LongOptional,
    #[serde(alias = "shipsLost")]       pub ship_lost: IntOptional,
    #[serde(alias = "pointsLost")]      pub points_lost: IntOptional,
    #[serde(alias = "iskLost")]         pub isk_lost: LongOptional,
    #[serde(alias = "soloKills")]       pub solo_kills: IntOptional,
    #[serde(alias = "soloLosses")]      pub solo_losses: IntOptional,

    #[serde(alias = "groups")]          pub groups: Groups,
    #[serde(alias = "months")]          pub months: Months,
    #[serde(alias = "topAllTime")]      pub tops: Vec<TopRecords>,
    #[serde(alias = "topIskKills")]     pub top_isk_kills: Option<HashMap<u32, IntRequired>>,

    #[serde(alias = "allTimeSum")]      pub all_time_sum: IntRequired,
    #[serde(alias = "nextTopRecalc")]   pub next_top_recalculate: IntRequired,
    #[serde(alias = "sequence")]        pub sequence: IntOptional,
    #[serde(alias = "trophies")]        pub trophies: Trophies,
    #[serde(alias = "activepvp")]       pub active_pvp: Kills,
    #[serde(alias = "info")]            pub info: Info,
    #[serde(alias = "topIskKillIDs")]   pub top_isk_kill_ids: Vec<IntRequired>,

/*
    "topLists":[
        {
            "type":"character",
            "title":"Top Characters",
            "values":[{"kills":2,"characterID":2114350216,"characterName":"Seb Odessa","id":2114350216,"typeID":null,"name":"Seb Odessa"}]
        },
        {
            "type":"corporation",
            "title":"Top Corporations",
            "values":[{"kills":2,"corporationID":98095669,"corporationName":"Techno Hive","cticker":"TE-HI","id":98095669,"typeID":null,"name":"Techno Hive"}]
        },
        {
            "type":"alliance",
            "title":"Top Alliances",
            "values":[{"kills":2,"allianceID":99009168,"allianceName":"Valkyrie Alliance","aticker":"VLK","id":99009168,"typeID":null,"name":"Valkyrie Alliance"}]
        },
        {
            "type":"shipType",
            "title":"Top Ships",
            "values":[{"kills":2,"shipTypeID":34828,"shipName":"Jackdaw","groupID":1305,"groupName":"Tactical Destroyer","id":34828,"typeID":null,"name":"Jackdaw"}]
        },
        {
            "type":"solarSystem",
            "title":"Top Systems",
            "values":[{"kills":2,"solarSystemID":30000776,"solarSystemName":"4M-QXK","sunTypeID":3802,"solarSystemSecurity":"-0.1","systemColorCode":"#F30202","regionID":10000009,"regionName":"Insmother","constellationID":20000113,"constellationName":"QA-P7J","id":30000776,"typeID":null,"name":"4M-QXK"}]
        },
        {
            "type":"location",
            "title":"Top Locations",
            "values":[{"kills":2,"locationID":40048967,"itemName":null,"locationName":"4M-QXK VI - Moon 10","typeID":null,"id":40048967,"name":"4M-QXK VI - Moon 10"}]
        }
    ],


    "topIskKillIDs":[81383507,81383490],
    "activity":{
        "max":10,
        "0":{"8":1,"9":1,"10":4,"12":2,"13":5,"15":1,"16":4,"19":8,"20":1},
        "1":{"18":6,"19":7,"20":1,"22":1},"2":{"16":1,"18":5,"19":9,"20":7},
        "3":{"17":3,"18":5,"19":2,"20":2,"21":1},
        "4":{"17":3,"18":4,"19":8,"20":5,"21":6},
        "5":{"19":2,"20":5},
        "6":{"13":3,"15":3,"16":5,"17":3,"18":2,"19":10,"20":9},
        "days":["Sun","Mon","Tue","Wed","Thu","Fri","Sat"]}}

*/
/*
    #[serde(skip, default, alias = "topLists")]
    pub top_lists: String,
    #[serde(skip, default, alias = "topIskKillIDs")]
    pub top_isk_kill_ids: String,
    #[serde(skip, default, alias = "activity")]
    pub activity: String,
*/
    #[serde(alias = "hasSupers")]
    pub has_supers: BoolOptional,
/*
    #[serde(skip, default, alias = "supers")]
    pub supers: String,
*/

}
impl Stats {
    fn load(entity: &str, id: &i32) -> Option<Self> {
        let json = gw::get_stats(entity, id);
        Some(serde_json::from_str(&json).expect(""))
    }

    pub fn new(entity: Entity) -> Option<Self> {
        match entity {
            Entity::Character(id)   => Self::load("characterID", &id),
            Entity::Corporation(id) => Self::load("corporationID", &id),
            Entity::Alliance(id)    => Self::load("allianceID", &id),
            Entity::Faction(id)     => Self::load("factionID", &id),
            Entity::Ship(id)        => Self::load("shipTypeID", &id),
            Entity::Group(id)       => Self::load("groupID", &id),
            Entity::System(id)      => Self::load("solarSystemID", &id),
            Entity::Region(id)      => Self::load("regionID", &id),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GroupStat {
    #[serde(alias = "groupID")]
    pub group_id: IntRequired,
    #[serde(alias = "shipsDestroyed")]
    pub ship_destroyed: IntOptional,
    #[serde(alias = "pointsDestroyed")]
    pub points_destroyed: IntOptional,
    #[serde(alias = "iskDestroyed")]
    pub isk_destroyed: LongOptional,
    #[serde(alias = "shipsLost")]
    pub ship_lost: IntOptional,
    #[serde(alias = "pointsLost")]
    pub points_lost: IntOptional,
    #[serde(alias = "iskLost")]
    pub isk_lost: LongOptional,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MonthStat {
    pub year: IntRequired,
    pub month: IntRequired,
    #[serde(alias = "shipsDestroyed")]
    pub ship_destroyed: IntOptional,
    #[serde(alias = "pointsDestroyed")]
    pub points_destroyed: IntOptional,
    #[serde(alias = "iskDestroyed")]
    pub isk_destroyed: LongOptional,
    #[serde(alias = "shipsLost")]
    pub ship_lost: IntOptional,
    #[serde(alias = "pointsLost")]
    pub points_lost: IntOptional,
    #[serde(alias = "iskLost")]
    pub isk_lost: LongOptional,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TopStat {
    pub kills: IntRequired,
    #[serde(alias = "characterID")]     pub character_id: IntOptional,
    #[serde(alias = "corporationID")]   pub corporation_id: IntOptional,
    #[serde(alias = "factionID")]       pub faction_id: IntOptional,
    #[serde(alias = "shipTypeID")]      pub ship_id: IntOptional,
    #[serde(alias = "solarSystemID")]   pub system_id: IntOptional,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct TopRecords {
    #[serde(alias = "type")] record_type: String,
    #[serde(alias = "data")] payload: Vec<TopStat>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Trophies	{
    pub levels: IntRequired,
    pub max: IntRequired,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Kills {
    #[serde(alias = "type")] pub record_type: String,
    #[serde(alias = "type")] pub count: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Info {
    #[serde(alias = "allianceID")]      pub alliance_id: IntOptional,
    #[serde(alias = "corporationID")]   pub corporation_id: IntOptional,
    #[serde(alias = "factionID")]       pub faction_id: IntOptional,
    #[serde(alias = "id")]              pub character_id: IntOptional,
    #[serde(alias = "lastApiUpdate")]   pub api_update: TimeOffset,
    #[serde(alias = "name")]            pub name: String,
    #[serde(alias = "secStatus")]       pub sec_status: FloatOptional,
    #[serde(alias = "type")]            pub record_type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct TimeOffset {
    pub sec: LongRequired,
    pub usec: LongRequired,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tops() {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
        struct Holder {
            #[serde(alias = "topAllTime")] tops: Vec<TopRecords>
        }

        let rec = json!({
            "topAllTime":[
                {"type":"character",    "data":[{"kills":667,"characterID":2114350216}]},
                {"type":"corporation",  "data":[{"kills":536,"corporationID":98095669},{"kills":129,"corporationID":98573194}]},
                {"type":"alliance",     "data":[{"kills":407,"allianceID":99007807},{"kills":129,"allianceID":99009168}]},
                {"type":"faction",      "data":[]},
                {"type":"ship",         "data":[{"kills":151,"shipTypeID":34828},{"kills":65,"shipTypeID":621}]},
                {"type":"system",       "data":[{"kills":85,"solarSystemID":30002386},{"kills":40,"solarSystemID":30002385}]}
            ]
        });
        let json = serde_json::to_string(&rec);
        assert!(json.is_ok());
        let val: Result<Holder, serde_json::Error> = serde_json::from_str(&json.unwrap());
        assert!(val.is_ok());
        let holder = val.unwrap();
        assert!(!holder.tops.is_empty());

    }

    #[test]
    fn months() {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
        struct Holder {
            months: Months
        }

        let rec = json!({
            "months":{
                "201810":{"year":2018,"month":10,"shipsLost":1,"pointsLost":1,"iskLost":59516640},
                "201811":{"year":2018,"month":11,"shipsLost":4,"pointsLost":9,"iskLost":1117539099},
                "201901":{"year":2019,"month":1,"shipsDestroyed":7,"pointsDestroyed":7,"iskDestroyed":305712147,"shipsLost":2,"pointsLost":9,"iskLost":605249501},
                "201902":{"year":2019,"month":2,"shipsDestroyed":28,"pointsDestroyed":89,"iskDestroyed":1560509893,"shipsLost":4,"pointsLost":41,"iskLost":560300039},
            }
        });

        let json = serde_json::to_string(&rec);
        assert!(json.is_ok());
        let val: Result<Holder, serde_json::Error> = serde_json::from_str(&json.unwrap());
        assert!(val.is_ok());
        let holder = val.unwrap();
        let maybe_item = holder.months.get(&201810);
        assert!(maybe_item.is_some());
        let item = maybe_item.unwrap();
        assert_eq!(2018, item.year);
        assert_eq!(  10, item.month);
    }

    #[test]
    fn groups() {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
        struct Holder {
            groups: Groups
        }

        let rec = json!({
            "groups":{
                "25":{
                    "groupID":25,
                    "shipsLost":10,
                    "pointsLost":146,
                    "iskLost":1565690214,
                    "shipsDestroyed":132,
                    "pointsDestroyed":266,
                    "iskDestroyed":2236504239u64},
                "1250":{
                    "groupID":1250,
                    "shipsDestroyed":17,
                    "pointsDestroyed":17,
                    "iskDestroyed":348701721},
            }
        });

        let json = serde_json::to_string(&rec);
        assert!(json.is_ok());
        let val: Result<Holder, serde_json::Error> = serde_json::from_str(&json.unwrap());
        assert!(val.is_ok());
        let holder = val.unwrap();
        let maybe_item = holder.groups.get(&25);
        assert!(maybe_item.is_some());
        let item = maybe_item.unwrap();
        assert_eq!(25, item.group_id);
    }


    #[test]
    fn from_api_for_character() {
        let response = Stats::new(Entity::Character(2114350216));
        assert!(response.is_some());
        let object = response.unwrap();
        assert_eq!(object.id, 2114350216);
    }
}
