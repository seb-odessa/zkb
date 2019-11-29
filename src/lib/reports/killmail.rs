use crate::models::*;
use crate::services;
use crate::services::Context;
use crate::reports;
use crate::models;
use crate::provider;
use separator::Separatable;


use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct Killmail {
    pub killmail_id: Integer,
    pub killmail_time: DateTime,
    pub system_id: Integer,
    pub system_name: OptString,
    pub constellation_id: OptInteger,
    pub constellation_name: OptString,
    pub region_id: OptInteger,
    pub region_name: OptString,
}
impl Killmail {

    fn get_dropped_sum(items: &Option<Vec<models::item::ItemNamed>>) -> u64 {
        let mut result = 0;
        if let Some(items) = items {
            for item in items {
                if let Some(ref price) = provider::get_avg_price(&Some(item.item_type_id)){
                    if let Some(ref quantity) = item.quantity_dropped {
                        result = result + (*quantity as f32  * *price) as u64;
                    }
                }
            }
        }
        return result;
    }

    fn get_total_sum(items: &Option<Vec<models::item::ItemNamed>>, victim: &Option<models::victim::VictimNamed>) -> u64 {
        let mut result = Self::get_dropped_sum(items);
        if let Some(items) = items {
            for item in items {
                if let Some(ref price) = provider::get_avg_price(&Some(item.item_type_id)){
                    if let Some(ref quantity) = item.quantity_destroyed {
                        result = result + (*quantity as f32  * *price) as u64;
                    }
                }
            }
        }
        if let Some(victim) = victim {
            if let Some(price) = provider::get_avg_price(&Some(victim.ship_id)){
                result = result + price as u64;
            }
        }
        return result;
    }

    fn security_status_color(status: f32) -> String {
        if status <= 0.0 {"Crimson"}
        else if status < 0.5 {"Red"}
        else if status < 0.8 {"YellowGreen"}
        else {"SkyBlue"}
        .to_string()
    }

    fn volume_color(value: &u64) -> String {
        if *value > 1_000_000_000 {"Red"}
        else if *value > 500_000_000 {"OrangeRed"}
        else if *value > 100_000_000 {"Tomato"}
        else if *value > 50_000_000 {"IndianRed"}
        else if *value > 10_000_000 {"LightCoral"}
        else if *value > 1_000_000 {"LightPink"}
        else {"WhiteSmoke"}
        .to_string()
    }

    fn npc_attacker_color(attacker: &models::attacker::AttackerNamed) -> String {
        if 500024 == attacker.get_id("faction") {
            return String::from("#ff00ff");
        }
        return String::from("WhiteSmoke");
    }

    fn npc_kill_color(attackers: &Option<Vec<models::attacker::AttackerNamed>>) -> String {
        if let Some(attackers) = attackers {
            for attacker in attackers {
                if 500024 == attacker.get_id("faction") {
                    return String::from("#ff00ff");
                }
            }
        }
        return String::from("WhiteSmoke");
    }

    pub fn write(output: &mut dyn Write, killmail: &killmail::KillmailNamed, ctx: &Context) {

        let killmail_id = killmail.killmail_id;
        let victim = reports::Victim::load(&killmail_id, ctx);
        let attackers = reports::Attacker::load(&killmail_id, ctx);
        let system = reports::System::load(&killmail.system_id, ctx);
        let items = reports::Item::load(&killmail_id, ctx);

        let mut security = 0.0;
        if let Some(system) = system {
            security = system.security_status;
        }
        let mut attackers_count = 0;
        if let Some(attackers) = attackers {
            attackers_count = attackers.len();
        }
        let security_status = reports::span(
            "System Security Status",
            format!("color: {};", Self::security_status_color(security)),
            format!("{:.1}", security),
        );

        let dropped_sum = Self::get_dropped_sum(&items);
        let dropped = reports::span(
            "Dropped Volume",
            format!("display: inline-block; width: 115px; text-align: right; background-color: {};", Self::volume_color(&dropped_sum)),
            dropped_sum.separated_string()
        );

        let total_sum = Self::get_total_sum(&items, &victim);
        let total = reports::span(
            "Total Kill Mail Volume",
            format!("display: inline-block; width: 125px; text-align: right; background-color: {};", Self::volume_color(&total_sum)),
            total_sum.separated_string()
        );

        let content = format!(
                r##"
                    {time} [{zkb}] |
                    {total} |
                    {dropped} |
                    {count} |
                    {region} : {constellation} : {system}
                    ({security_status})
                    {victim}
                "##,

                time = ctx.get_api_href("killmail", killmail_id, killmail.killmail_time.time().to_string()),
                zkb = ctx.get_zkb_href("kill", killmail_id, format!("zkb")),
                region = ctx.get_api_href("region", killmail.get_id("region"), killmail.get_name("region")),
                constellation = ctx.get_api_href("constellation", killmail.get_id("constellation"), killmail.get_name("constellation")),
                system = ctx.get_api_href("system", killmail.get_id("system"), killmail.get_name("system")),
                security_status = security_status,
                count = attackers_count,
                dropped = dropped,
                total = total,
                victim = reports::Victim::report_name(&killmail_id, ctx),
        );
        reports::div(output, content);
    }

    pub fn write_row(output: &mut dyn Write, killmail: &killmail::KillmailNamed, ctx: &Context) {
        let text_style    = "border: 0px solid black; padding: 2px 5px;";

        let killmail_id = killmail.killmail_id;
        let victim = reports::Victim::load(&killmail_id, ctx);
        let attackers = reports::Attacker::load(&killmail_id, ctx);
        let system = reports::System::load(&killmail.system_id, ctx);
        let items = reports::Item::load(&killmail_id, ctx);

        let mut security = 0.0;
        if let Some(system) = system {
            security = system.security_status;
        }
        let mut attackers_count = 0;
        if let Some(ref attackers) = attackers {
            attackers_count = attackers.len();
        }
        let security_status_span = reports::span(
            "System Security Status",
            format!("color: {};", Self::security_status_color(security)),
            format!("{:.2}", security),
        );

        let dropped_sum = Self::get_dropped_sum(&items);
        let dropped_span = reports::span(
            "Dropped Volume",
            format!("display: inline-block; width: 100%; text-align: right; background-color: {};", Self::volume_color(&dropped_sum)),
            format!("{}", dropped_sum.separated_string())
        );

        let total_sum = Self::get_total_sum(&items, &victim);
        let total_span = reports::span(
            "Total Kill Mail Volume",
            format!("display: inline-block; width: 100%; text-align: right; background-color: {};", Self::volume_color(&total_sum)),
            format!("{}", total_sum.separated_string())
        );

        let system_style = format!("background-color: {};", Self::security_status_color(security));

        if let Some(victim) = victim {
            let row_style = format!("background-color: {};", Self::npc_kill_color(&attackers));
            reports::table_row_start(output, row_style);
            reports::table_cell(output, "Time", text_style, ctx.get_api_href("killmail", killmail_id, killmail.killmail_time.time().to_string()));
            reports::table_cell(output, "Reference to ZKB", text_style, ctx.get_zkb_href("kill", killmail_id, format!("zkb")));
            reports::table_cell(output, "Killmail Amount", text_style, total_span);
            reports::table_cell(output, "Dropped Amount", text_style, dropped_span);
            reports::table_cell(output, "Attackers Count", text_style, attackers_count.separated_string());
            reports::table_cell(output, "Region", text_style, ctx.get_api_href("region", killmail.get_id("region"), killmail.get_name("region")));
            reports::table_cell(output, "Constellation", text_style, ctx.get_api_href("constellation", killmail.get_id("constellation"), killmail.get_name("constellation")));
            reports::table_cell(output, "System", system_style, ctx.get_api_href("system", killmail.get_id("system"), killmail.get_name("system")));
            reports::table_cell(output, "Security status", text_style, security_status_span);
            reports::table_cell(output, "Character Name", text_style, ctx.get_api_href("character", victim.get_id("character"), victim.get_name("character")));
            reports::table_cell(output, "Corporation Name", text_style, ctx.get_api_href("corporation", victim.get_id("corporation"), victim.get_name("corporation")));
            reports::table_cell(output, "Alliance Name", text_style, ctx.get_api_href("alliance", victim.get_id("alliance"), victim.get_name("alliance")));
            reports::table_cell(output, "Faction Name", text_style, ctx.get_api_href("faction", victim.get_id("faction"), victim.get_name("faction")));
            reports::table_row_end(output);
        }
    }

    fn write_report(output: &mut dyn Write, killmail: &killmail::KillmailNamed, ctx: &Context) {
        let killmail_id = killmail.killmail_id;
        let victim = reports::Victim::load(&killmail_id, ctx);
        let attackers = reports::Attacker::load(&killmail_id, ctx);
        let system = reports::System::load(&killmail.system_id, ctx);
        let items = reports::Item::load(&killmail_id, ctx);

        reports::div(output, format!("{timestamp} [{zkb}]",
            timestamp = ctx.get_api_href("killmail", killmail_id, killmail.killmail_time.to_string()),
            zkb = ctx.get_zkb_href("kill", killmail.get_id("id"), format!("zkb"))));

        if let Some(system) = system {
            reports::div(output,
                format!("Location: {} : {} : {}",
                    ctx.get_api_href("region", system.get_id("region"), system.get_name("region")),
                    ctx.get_api_href("constellation", system.get_id("constellation"), system.get_name("constellation")),
                    ctx.get_api_href("system", system.get_id("system"), system.get_name("system")),
                )
            );
        }

        reports::div(output, format!("Total killmail amount: {}", Self::get_total_sum(&items, &victim).separated_string()));
        reports::div(output, format!("Dropped amount: {}", Self::get_dropped_sum(&items).separated_string()));

        if let Some(victim) = victim {
            reports::div(output, format!("Damage Taken: {}", victim.damage_taken.separated_string()));
            reports::div(output, format!("Ship: {}", ctx.get_api_href("ship", victim.get_id("ship"), victim.get_name("ship"))));
            reports::div(output,
                format!("Pilot: {} {} {} {}",
                    ctx.get_api_href("faction", victim.get_id("faction"), victim.get_name("faction")),
                    ctx.get_api_href("alliance", victim.get_id("alliance"), victim.get_name("alliance")),
                    ctx.get_api_href("corporation", victim.get_id("corporation"), victim.get_name("corporation")),
                    ctx.get_api_href("character", victim.get_id("character"), victim.get_name("character")),
                )
            );
        }
        let table_style   = "border-collapse: collapse;";
        let head_style    = "border: 1px solid black; padding: 2px 5px; text-align: center; ";
        let text_style    = "border: 1px solid black; padding: 2px 5px;";
        let numeric_style = "border: 1px solid black; padding: 2px 5px;; text-align: right;";

        if let Some(attackers) = attackers {
            reports::table_start(output, "Attackers", table_style, "Attackers");
            reports::table_row_start(output, "");
            reports::table_cell_head(output, "Security Status", head_style, "SS");
            reports::table_cell_head(output, "Final Blow", head_style, "Final");
            reports::table_cell_head(output, "Damage Done", head_style, "Damage");
            reports::table_cell_head(output, "Weapon", head_style, "Weapon");
            reports::table_cell_head(output, "Ship Type", head_style, "Ship Type");
            reports::table_cell_head(output, "Character Name", head_style, "Character");
            reports::table_cell_head(output, "Corporation Name", head_style, "Corporation");
            reports::table_cell_head(output, "Alliance Name", head_style, "Alliance");
            reports::table_cell_head(output, "Faction Name", head_style, "Faction");
            reports::table_row_end(output);
            for attacker in attackers {
                reports::table_row_start(output, format!("background-color: {};", Self::npc_attacker_color(&attacker)));
                reports::table_cell(output, "Security Status", text_style, attacker.security_status.separated_string());
                reports::table_cell(output, "Final Blow", text_style, attacker.final_blow.to_string());
                reports::table_cell(output, "Damage Done", numeric_style, attacker.damage_done.separated_string());
                reports::table_cell(output, "Weapon", text_style, ctx.get_api_href("weapon", attacker.get_id("weapon"), attacker.get_name("weapon")));
                reports::table_cell(output, "Ship Type", text_style, ctx.get_api_href("ship", attacker.get_id("ship"), attacker.get_name("ship")));
                reports::table_cell(output, "Character Name", text_style, ctx.get_api_href("character", attacker.get_id("character"), attacker.get_name("character")));
                reports::table_cell(output, "Corporation Name", text_style, ctx.get_api_href("corporation", attacker.get_id("corporation"), attacker.get_name("corporation")));
                reports::table_cell(output, "Alliance Name", text_style, ctx.get_api_href("alliance", attacker.get_id("alliance"), attacker.get_name("alliance")));
                reports::table_cell(output, "Faction Name", text_style, ctx.get_api_href("faction", attacker.get_id("faction"), attacker.get_name("faction")));
                reports::table_row_end(output);
            }
            reports::table_end(output);
        }

        if let Some(items) = items {
            #[derive(Debug, Clone)]
            struct ZippedItem {
                id: i32,
                dropped: u64,
                destroyed: u64,
                price: Option<f32>,
            }
            impl ZippedItem {
                fn new(id: i32) -> Self {
                    Self{ id: id, dropped: 0, destroyed: 0, price: None }
                }
                fn get_price(&self) -> f32 {
                    self.price.clone().unwrap_or(0.0)
                }
                fn get_dropped_volume(&self) -> u64 {
                    (self.price.clone().unwrap_or(0.0) * self.dropped as f32) as u64
                }
                fn get_destroyed_volume(&self) -> u64 {
                    (self.price.clone().unwrap_or(0.0) * self.destroyed as f32) as u64
                }
            }
            use std::collections::BTreeMap;
            let mut zipped_map = BTreeMap::new();
            for item in items {
                let id = item.get_id();
                let name = item.get_name();
                let mut zipped = zipped_map.entry(name.clone()).or_insert(ZippedItem::new(id));
                if let Some(dropped) = item.quantity_dropped {
                    zipped.dropped += dropped as u64;
                }
                if let Some(destroyed) = item.quantity_destroyed {
                    zipped.destroyed += destroyed as u64;
                }
                if zipped.price.is_none() {
                    zipped.price = provider::get_avg_price(&Some(id))
                }
            }

            reports::table_start(output, "Items", table_style, "Items");
            reports::table_row_start(output, "");
            reports::table_cell_head(output, "Item Name", head_style, "Item");
            reports::table_cell_head(output, "Dropped quantity", head_style, "Dropped");
            reports::table_cell_head(output, "Destroyed quantity", head_style, "Destroyed");
            reports::table_cell_head(output, "Price", head_style, "Price");
            reports::table_cell_head(output, "Dropped value", head_style, "Dropped");
            reports::table_cell_head(output, "Destroyed value", head_style, "Destroyed");

            reports::table_row_end(output);
            for (name, desc) in &zipped_map {
                reports::table_row_start(output, "");
                reports::table_cell(output, "Item name", text_style, name);
                reports::table_cell(output, "Dropped quantity", numeric_style, desc.dropped.separated_string());
                reports::table_cell(output, "Destroyed quantity", numeric_style, desc.destroyed.separated_string());
                reports::table_cell(output, "Item's average price", numeric_style, desc.get_price().separated_string());
                reports::table_cell(output, "Dropped quantity", numeric_style, desc.get_dropped_volume().separated_string());
                reports::table_cell(output, "Destroyed quantity", numeric_style, desc.get_destroyed_volume().separated_string());
                reports::table_row_end(output);
            }
            reports::table_end(output);
        }
    }


    pub fn brief(arg: &String, ctx: &Context) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::brief_impl(id, ctx)
        } else {
            format!("Can't parse {}", arg)
        }
    }

    pub fn report(arg: &String, ctx: &Context) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::report_impl(id, ctx)
        } else {
            format!("Can't parse {}", arg)
        }
    }

    pub fn load(id: &i32, ctx: &Context) -> Option<models::killmail::KillmailNamed> {
        use services::{Category, Report};
        match reports::load(Category::Killmail(*id), &ctx) {
            Report::Killmail(killmail) => return Some(killmail),
            Report::NotFoundId(id) => warn!("{} was not found", id),
            report => warn!("Unexpected report {:?}", report)
        }
        return None;
    }

    pub fn brief_impl(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(object) = Self::load(id, ctx) {
            Self::write(&mut output, &object, ctx);
        }
        return output;
    }

    pub fn report_impl(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(object) = Self::load(id, ctx) {
            Self::write_report(&mut output, &object, ctx);
        }
        return output;
    }
}
