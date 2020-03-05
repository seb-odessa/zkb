use crate::api::*;
use crate::reports;
use crate::services::Context;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;

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
//#[serde(deny_unknown_fields)]
pub struct Stats {
    pub id: IntRequired,

    #[serde(alias = "type")]            pub record_type: String,
    #[serde(alias = "dangerRatio")]     pub danger_ratio: IntOptional,
    #[serde(alias = "gangRatio")]       pub gang_ratio: IntOptional,
    #[serde(alias = "shipsDestroyed")]  pub ship_destroyed: IntOptional,
    #[serde(alias = "pointsDestroyed")] pub points_destroyed: IntOptional,
    #[serde(alias = "iskDestroyed")]    pub isk_destroyed: LongOptional,
    #[serde(alias = "shipsLost")]       pub ship_lost: IntOptional,
    #[serde(alias = "pointsLost")]      pub points_lost: IntOptional,
    #[serde(alias = "iskLost")]         pub isk_lost: LongOptional,
    #[serde(alias = "soloKills")]       pub solo_kills: IntOptional,
    #[serde(alias = "soloLosses")]      pub solo_losses: IntOptional,

    #[serde(skip, alias = "groups")]          pub groups: Groups,
    #[serde(skip, alias = "months")]          pub months: Months,
    #[serde(alias = "topAllTime")]      pub tops: Vec<TopRecords>,
    #[serde(alias = "topIskKills")]     pub top_isk_kills: Option<Vec<IntRequired>>,

    #[serde(skip, alias = "allTimeSum")]      pub all_time_sum: IntRequired,
    #[serde(skip, alias = "nextTopRecalc")]   pub next_top_recalculate: IntRequired,
    #[serde(skip, alias = "sequence")]        pub sequence: IntOptional,
    #[serde(skip, alias = "trophies")]        pub trophies: Option<Trophies>,
    #[serde(skip, alias = "activepvp")]       pub active_pvp: ActivePvp,
    #[serde(skip, alias = "info")]            pub info: String, //Info,
    #[serde(skip, alias = "topIskKillIDs")]   pub top_isk_kill_ids: Vec<IntRequired>,
    #[serde(alias = "topLists")]        pub top_lists: Vec<TopList>,
    #[serde(alias = "activity")]        pub activity: Option<Activity>,
    #[serde(skip, alias = "hasSupers")]       pub has_supers: BoolOptional,
    #[serde(skip)]                      pub supers: Option<SuperValues>, //alias = "supers"
}

impl Stats {
    fn load(entity: &str, id: &i32) -> Option<Self> {
        let json = gw::get_stats(entity, id);
        match serde_json::from_str(&json) {
            Ok(object) => Some(object),
            Err(err) => {println!("{}", err); None}
        }
    }

    pub fn danger_ratio(&self)-> IntRequired {
        self.danger_ratio.unwrap_or_default()
    }

    pub fn gang_ratio(&self)-> IntRequired {
        self.gang_ratio.unwrap_or_default()
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

    pub fn report_win_loses<S: Into<String>>(output: &mut dyn Write, title: S, wins: Option<i32>, losses: Option<i32>) {
        let wins = wins.unwrap_or_default();
        let losses = losses.unwrap_or_default();
        let total = wins + losses;
        let eff = if total != 0 {
            100 * wins / total
        } else {
            0
        };
        reports::div(output, format!("{}: {}/{} eff: {}%", title.into(), wins, total, eff));
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
    #[serde(alias = "type")] record_type: StrRequired,
    #[serde(alias = "data")] payload: Vec<TopStat>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename = "trophies")]
pub struct Trophies	{
    pub levels: IntRequired,
    pub max: IntRequired,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ActivePvp {
    pub kills: ActivePvpKills,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename = "kills")]
pub struct ActivePvpKills {
    #[serde(alias = "type")] pub record_type: StrRequired,
    pub count: IntRequired,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct LastApiUpdate {
    pub sec: LongRequired,
    pub usec: LongRequired,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Info {
    CharacterInfo {
        #[serde(alias = "allianceID")]      alliance_id: IntRequired,
        #[serde(alias = "corporationID")]   corporation_id: IntRequired,
        #[serde(alias = "factionID")]       faction_id: IntRequired,
        #[serde(alias = "id")]              character_id: IntRequired,
        #[serde(alias = "lastApiUpdate")]   last_update: LastApiUpdate,
        #[serde(alias = "name")]            name: StrRequired,
        #[serde(alias = "secStatus")]       sec_status: FloatRequired,
        #[serde(alias = "type")]            record_type: StrRequired,
    },

    CorporationInfo {
        #[serde(alias = "allianceID")]      alliance_id: IntRequired,
        #[serde(alias = "ceoID")]           ceo_id: IntRequired,
        #[serde(alias = "factionID")]       faction_id: IntRequired,
        #[serde(alias = "id")]              corporation_id: IntRequired,
        #[serde(alias = "lastApiUpdate")]   last_update: LastApiUpdate,
        #[serde(alias = "name")]            name: StrRequired,
        #[serde(alias = "ticker")]          ticker: StrRequired,
        #[serde(alias = "type")]            record_type: StrRequired,
    },

    AllianceInfo {
        #[serde(alias = "executorCorpID")]  exec_corp_id: IntRequired,
        #[serde(alias = "factionID")]       faction_id: IntRequired,
        #[serde(alias = "id")]              alliance_id: IntRequired,
        #[serde(alias = "lastApiUpdate")]   last_update: LastApiUpdate,
        #[serde(alias = "memberCount")]     member_count: IntRequired,
        #[serde(alias = "corpCount")]       corp_count: IntRequired,
        #[serde(alias = "name")]            name: StrRequired,
        #[serde(alias = "ticker")]          ticker: StrRequired,
        #[serde(alias = "type")]            record_type: StrRequired,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TopList {
    #[serde(alias = "type")]            pub record_type: StrRequired,
    #[serde(alias = "title")]           pub title: StrRequired,
    #[serde(alias = "values")]          pub values: Vec<TopValue>,
}
impl TopList {

    #[allow(unused_variables)]
    pub fn write(output: &mut dyn Write, tops: &Vec<Self>, allowed: HashSet<String>, ctx: &Context) {
        reports::div(output, "Last week activity");
        let table_style   = "border-collapse: collapse;";
        let text_style    = "border: 1px solid black; padding: 1px 5px;";
        for top in tops {
            if allowed.contains(&top.record_type) {
                reports::table_start(output, &top.title, table_style, "");
                if !top.values.is_empty() {
                    reports::caption(output, &top.title);
                }
                for value in &top.values {
                    reports::table_row_start(output, "");
                    match value {
                        TopValue::CharacterTop {kills, character_id, character_name, .. } => {
                            reports::table_cell(output, "Kills", text_style, kills.to_string());
                            reports::table_cell(output, "character name", text_style, ctx.get_api_href("character", *character_id, character_name));
                        },
                        TopValue::CorporationTop {kills, corporation_id, corporation_name, corporation_ticker, .. } => {
                            reports::table_cell(output, "Kills", text_style, kills.to_string());
                            reports::table_cell(output, "corporation name", text_style, ctx.get_api_href("corporation", *corporation_id, corporation_name));
                            reports::table_cell(output, "corporation ticker", text_style, corporation_ticker);
                        },
                        TopValue::AllianceTop {kills, alliance_id, alliance_name, alliance_ticker, .. } => {
                            reports::table_cell(output, "Kills", text_style, kills.to_string());
                            reports::table_cell(output, "alliance name", text_style, ctx.get_api_href("alliance", *alliance_id, alliance_name));
                            reports::table_cell(output, "alliance ticker", text_style, alliance_ticker);
                        },
                        TopValue::ShipTop {kills, ship_id, ship_name, group_id, group_name, .. } => {
                            reports::table_cell(output, "Kills", text_style, kills.to_string());
                            reports::table_cell(output, "ship", text_style, ctx.get_zkb_href("ship", *ship_id, ship_name));
                            reports::table_cell(output, "group", text_style, ctx.get_zkb_href("group", *group_id, group_name));
                        },
                        TopValue::SystemTop {kills, system_id, system_name, sun_type_id, system_security, system_color, region_id, region_name, .. } => {
                            let style = format!("{} background-color: {};", text_style, system_color);
                            reports::table_cell(output, "Kills", &style, kills.to_string());
                            reports::table_cell(output, "system", &style, ctx.get_api_href("system", *system_id, system_name));
                            reports::table_cell(output, "system security", &style, system_security);
                            reports::table_cell(output, "region", &style, ctx.get_api_href("region", *region_id, region_name));
                        }
                        TopValue::LocationTop {kills, location_id, location_name, .. } => {
                            reports::table_cell(output, "Kills", text_style, kills.to_string());
                            reports::table_cell(output, "location", text_style, ctx.get_zkb_href("location", *location_id, location_name));
                        }
                    }
                    reports::table_row_end(output);
                }
                reports::table_end(output);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum TopValue {
    CharacterTop {
        #[serde(alias = "kills")]           kills: IntRequired,
        #[serde(alias = "characterID")]     character_id: IntRequired,
        #[serde(alias = "characterName")]   character_name: StrRequired,
        #[serde(alias = "id")]              id: IntRequired,
        #[serde(alias = "typeID")]          type_id: IntOptional,
        #[serde(alias = "name")]            name: StrRequired,
    },
    CorporationTop {
        #[serde(alias = "kills")]           kills: IntRequired,
        #[serde(alias = "corporationID")]   corporation_id: IntRequired,
        #[serde(alias = "corporationName")] corporation_name: StrRequired,
        #[serde(alias = "cticker")]         corporation_ticker: StrRequired,
        #[serde(alias = "id")]              id: IntRequired,
        #[serde(alias = "typeID")]          type_id: IntOptional,
        #[serde(alias = "name")]            name: StrRequired,
    },
    AllianceTop {
        #[serde(alias = "kills")]           kills: IntRequired,
        #[serde(alias = "allianceID")]      alliance_id: IntRequired,
        #[serde(alias = "allianceName")]    alliance_name: StrRequired,
        #[serde(alias = "aticker")]         alliance_ticker: StrRequired,
        #[serde(alias = "id")]              id: IntRequired,
        #[serde(alias = "typeID")]          type_id: IntOptional,
        #[serde(alias = "name")]            name: StrRequired,
    },
    ShipTop {
        #[serde(alias = "kills")]           kills: IntRequired,
        #[serde(alias = "shipTypeID")]      ship_id: IntRequired,
        #[serde(alias = "shipName")]        ship_name: StrRequired,
        #[serde(alias = "groupID")]         group_id: IntRequired,
        #[serde(alias = "groupName")]       group_name: StrRequired,
        #[serde(alias = "id")]              id: IntRequired,
        #[serde(alias = "typeID")]          type_id: IntOptional,
        #[serde(alias = "name")]            name: StrRequired,
    },
    SystemTop {
        #[serde(alias = "kills")]               kills: IntRequired,
        #[serde(alias = "solarSystemID")]       system_id: IntRequired,
        #[serde(alias = "solarSystemName")]     system_name: StrRequired,
        #[serde(alias = "sunTypeID")]           sun_type_id: IntRequired,
        #[serde(alias = "solarSystemSecurity")] system_security: StrRequired,
        #[serde(alias = "systemColorCode")]     system_color: StrRequired,
        #[serde(alias = "regionID")]            region_id: IntRequired,
        #[serde(alias = "regionName")]          region_name: StrRequired,
        #[serde(alias = "constellationID")]     constellation_id: IntRequired,
        #[serde(alias = "constellationName")]   constellation_name: StrRequired,
        #[serde(alias = "id")]                  id: IntRequired,
        #[serde(alias = "typeID")]              type_id: IntOptional,
        #[serde(alias = "name")]                name: StrRequired,
    },
    LocationTop {
        #[serde(alias = "kills")]           kills: IntRequired,
        #[serde(alias = "locationID")]      location_id: IntRequired,
        #[serde(alias = "locationName")]    location_name: StrRequired,
        #[serde(alias = "itemName")]        item_name: StrOptional,
        #[serde(alias = "id")]              id: IntRequired,
        #[serde(alias = "typeID")]          type_id: IntOptional,
        #[serde(alias = "name")]            name: StrRequired,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum HourKills {
    AsMap(HashMap<StrRequired, IntRequired>),
    AsVec(Vec<IntRequired>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Activity {
    pub max: IntRequired,
    #[serde(alias = "0")]   pub sun: HourKills,
    #[serde(alias = "1")]   pub mon: HourKills,
    #[serde(alias = "2")]   pub tue: HourKills,
    #[serde(alias = "3")]   pub wed: HourKills,
    #[serde(alias = "4")]   pub thu: HourKills,
    #[serde(alias = "5")]   pub fri: HourKills,
    #[serde(alias = "6")]   pub sat: HourKills,
    pub days: Vec<StrRequired>,
}
impl Activity {

    #[allow(unused_variables)]
    pub fn write(output: &mut dyn Write, activity: &Activity, ctx: &Context) {
        reports::script(output, ctx.get_js_url("Chart.bundle.min.js"));
        for day in &activity.days {
            let id = format!("{}_{}", &day, crate::create_id());
            reports::canvas(output, &id, 20, 20);
            let script = format!(r#"
            <script>
                document.addEventListener(
                    "DOMContentLoaded",
                    function(event) {{
                        var ctx = document.getElementById("{id}").getContext('2d');
                        var myChart = new Chart(ctx, {{
                        type: 'radar',
                        data: {{
                            labels: [ '0',  '1',  '2',  '3',  '4',  '5',  '6',  '7',  '8',  '9', '10', '11',
                                    '12', '13', '14', '15', '16', '17', '18', '19', '20', '21', '22', '23'],
                            datasets: [{{
                                label: '{day}',
                                data: [1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6],
                                borderWidth: 1
                            }}]
                        }},
                        options: {{
                            scales: {{
                                yAxes: [{{
                                    ticks: {{
                                        beginAtZero: false
                                    }}
                                }}]
                            }}
                        }}
                    }});
                }});
            </script>\n"#, id=id, day=day);
            reports::write(output, script);
        }
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum SuperValues {
    Empty(Vec<String>),
    Exists(HashMap<String, Super>)
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Super {
    data: Vec<SuperStat>,
    title: StrRequired,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SuperStat {
    kills: IntRequired,
    #[serde(alias = "characterID")]     character_id: IntRequired,
    #[serde(alias = "characterName")]   character_name: StrRequired,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    #[allow(unused_variables)]
    fn info() {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
        struct Holder {
            #[serde(alias = "info")] info: Info
        }
        { // character
            let rec = json!({
                "info":{
                    "allianceID":99009168,
                    "corporationID":98095669,
                    "factionID":0,
                    "id":2114350216,
                    "lastApiUpdate":{"sec":1579812891,"usec":0},
                    "name":"Seb Odessa",
                    "secStatus":5.0053732177373,
                    "type":"characterID"
                }
            });
            let json = serde_json::to_string(&rec);
            assert!(json.is_ok());
            let val: Result<Holder, serde_json::Error> = serde_json::from_str(&json.unwrap());
            assert!(val.is_ok());
            let item = val.unwrap();
            match item.info {
                Info::CharacterInfo{ alliance_id, corporation_id, faction_id, character_id, last_update, name, .. } => {
                    assert_eq!(99009168,        alliance_id);
                    assert_eq!(98095669,        corporation_id);
                    assert_eq!(2114350216,      character_id);
                    assert_eq!("Seb Odessa",   &name);
                },
                _ => assert!(false)
            }
        }
        { // corporation
            let rec = json!({
                "info":{
                    "allianceID":99009168,
                    "ceoID":1383227978,
                    "factionID":0,
                    "id":98095669,
                    "lastApiUpdate":{"sec":1580685054,"usec":0},
                    "memberCount":63,
                    "name":"Techno Hive",
                    "ticker":"TE-HI",
                    "type":"corporationID"
                },
            });
            let json = serde_json::to_string(&rec);
            assert!(json.is_ok());
            let val: Result<Holder, serde_json::Error> = serde_json::from_str(&json.unwrap());
            assert!(val.is_ok());
            let item = val.unwrap();
            match item.info {
                Info::CorporationInfo{ alliance_id, ceo_id, faction_id, corporation_id, last_update, name, .. } => {
                    assert_eq!(99009168,        alliance_id);
                    assert_eq!(98095669,        corporation_id);
                    assert_eq!(1383227978,      ceo_id);
                    assert_eq!("Techno Hive",  &name);
                },
               _ => assert!(false)
            }
        }

    }

    #[test]
    fn tops() {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
        struct Holder {
            #[serde(alias = "topAllTime")] inner: Vec<TopRecords>
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
        assert!(!holder.inner.is_empty());

    }

    #[test]
    fn months() {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
        struct Holder {
            inner: Months
        }

        let rec = json!({
            "inner":{
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
        let maybe_item = holder.inner.get(&201810);
        assert!(maybe_item.is_some());
        let item = maybe_item.unwrap();
        assert_eq!(2018, item.year);
        assert_eq!(  10, item.month);
    }

    #[test]
    fn groups() {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
        struct Holder {
            inner: Groups
        }

        let rec = json!({
            "inner":{
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
        let maybe_item = holder.inner.get(&25);
        assert!(maybe_item.is_some());
        let item = maybe_item.unwrap();
        assert_eq!(25, item.group_id);
    }

    #[test]
    fn active_pvp() {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
        struct Holder {
            inner: ActivePvp
        }
        let rec = json!({"inner":{"kills":{"type":"Total Kills","count":2}}});
        let json = serde_json::to_string(&rec);
        assert!(json.is_ok());
        let val: Result<Holder, serde_json::Error> = serde_json::from_str(&json.unwrap());
        assert!(val.is_ok());
        let holder = val.unwrap();
        let item: ActivePvp = holder.inner;
        assert_eq!("Total Kills", &item.kills.record_type);
        assert_eq!(2, item.kills.count);
    }

    #[test]
    fn from_api_for_character() {
        let response = Stats::new(Entity::Character(2114350216));
        assert!(response.is_some());
        let object = response.unwrap();
        assert_eq!(object.id, 2114350216);
        assert_eq!(&object.record_type, "characterID");
    }

    #[test]
    fn character_activity() {
        let response = Stats::new(Entity::Character(2114350216));
        assert!(response.is_some());
        let object = response.unwrap();
        assert_eq!(object.id, 2114350216);
        assert_eq!(&object.record_type, "characterID");
        assert!(object.activity.is_some());
        let activity: Activity = object.activity.unwrap();
        assert_eq!(activity.max, 32);
        assert_eq!(&activity.days[0], "Sun");
        assert_eq!(&activity.days[1], "Mon");
        assert_eq!(&activity.days[2], "Tue");
        assert_eq!(&activity.days[3], "Wed");
        assert_eq!(&activity.days[4], "Thu");
        assert_eq!(&activity.days[5], "Fri");
        assert_eq!(&activity.days[6], "Sat");
        if let HourKills::AsMap(kill_map) = activity.sun {
            assert_eq!(kill_map.get(&String::from("8")), Some(&1));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn from_api_for_corporation() {
        let response = Stats::new(Entity::Corporation(98190062));
        assert!(response.is_some());
        let object = response.unwrap();
        assert_eq!(object.id, 98190062);
        assert_eq!(&object.record_type, "corporationID");
    }

    #[test]
    fn from_api_for_alliance() {
        let response = Stats::new(Entity::Alliance(1354830081));
        assert!(response.is_some());
        let object = response.unwrap();
        assert_eq!(object.id, 1354830081);
        assert_eq!(&object.record_type, "allianceID");
    }

    #[test]
    fn from_api_for_system() {
        let response = Stats::new(Entity::System(30000142));
        assert!(response.is_some());
        let object = response.unwrap();
        assert_eq!(object.id, 30000142);
        assert_eq!(&object.record_type, "solarSystemID");
    }

    #[test]
    fn from_api_for_region() {
        let response = Stats::new(Entity::Region(10000002));
        assert!(response.is_some());
        let object = response.unwrap();
        assert_eq!(object.id, 10000002);
        assert_eq!(&object.record_type, "regionID");
    }
}
