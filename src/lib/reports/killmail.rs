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

    fn npc_attacker_style(attacker: &models::attacker::AttackerNamed) -> String {
        if 500024 == attacker.get_id("faction") {
            return String::from("background-color: #ff00ff;");
        }
        return String::new();
    }

    fn npc_kill_style(attackers: &Option<Vec<models::attacker::AttackerNamed>>) -> String {
        if let Some(attackers) = attackers {
            for attacker in attackers {
                if 500024 == attacker.get_id("faction") {
                    return String::from("background-color: #ff00ff;");
                }
            }
        }
        return String::new();
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
            format!("color: {};", reports::get_security_status_color(security)),
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
                region = ctx.get_api_link("region", killmail.get_name("region")),
                constellation = ctx.get_api_link("constellation", killmail.get_name("constellation")),
                system = ctx.get_api_link("system", killmail.get_name("system")),
                security_status = security_status,
                count = attackers_count,
                dropped = dropped,
                total = total,
                victim = reports::Victim::report_name(&killmail_id, ctx),
        );
        reports::div(output, content);
    }

    pub fn write_head(output: &mut dyn Write) {
        let head_style = "border: 1px solid black; padding: 2px 5px; text-align: center;";
        reports::table_row_start(output, head_style);
        reports::table_cell_head(output, "API/ZKB", head_style, "Time<br/>ZKB");
        reports::table_cell_head(output, "Total Amount/Dropped Amount", head_style, "Amount<br/>Dropped");
        reports::table_cell_head(output, "Ship Destroyed", head_style, "Ship");
        reports::table_cell_head(output, "Damage Taken", head_style, "Damage");
        reports::table_cell_head(output, "Region/Constellation/System (SS)", head_style, "Region<br/>Constellation<br/>System");
        reports::table_cell_head(output, "Victim's Faction/Alliance/Corporation/Character", head_style, "Faction<br/>Alliance<br/>Corporation<br/>Character");
        reports::table_cell_head(output, "Attackers Count", head_style, "Attackers");
        reports::table_cell_head(output, "Attacker's Faction/Alliance/Corporation/Character", head_style, "Faction<br/>Alliance<br/>Corporation<br/>Character");
        reports::table_row_end(output);
    }

    pub fn write_row(output: &mut dyn Write, killmail: &killmail::KillmailNamed, ctx: &Context) {
        let text_style = "border: 1px solid black; padding: 2px 5px;";

        let killmail_id = killmail.killmail_id;
        let victim = reports::Victim::load(&killmail_id, ctx);
        let attackers = reports::Attacker::load(&killmail_id, ctx);
        let system = reports::System::load(&killmail.system_id, ctx);
        let items = reports::Item::load(&killmail_id, ctx);

        let mut security = 0.0;
        if let Some(system) = system {
            security = system.security_status;
        }

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

        if let Some(victim) = victim {
            let row_style = Self::npc_kill_style(&attackers);
            let timestamp = killmail.killmail_time.time().format("%H:%M:%S").to_string();
            reports::table_row_start(output, row_style);
            reports::table_cell(output, "API/ZKB", text_style,
                format!("{}<br/>{}",
                    ctx.get_api_href("killmail", killmail_id, timestamp),
                    ctx.get_zkb_href("kill", killmail_id, format!("zkb"))
                )
            );
            reports::table_cell(output, "Killmail Amount/Dropped Amount", text_style, format!("{}<br/>{}", total_span, dropped_span));
            reports::table_cell(output, "Ship Destroyed", text_style, ctx.get_zkb_href("ship", victim.get_id("ship"), victim.get_name("ship")));
            reports::table_cell(output, "Damage Taken", text_style, victim.damage_taken.separated_string());
            reports::table_cell(output, "", text_style,
                format!("{}<br/>{}<br/>{} {}",
                    reports::span("Region", "", ctx.get_api_link("region", killmail.get_name("region"))),
                    reports::span("Constellation", "", ctx.get_api_link("constellation", killmail.get_name("constellation"))),
                    reports::span("System", "", ctx.get_api_link("system", killmail.get_name("system"))),
                    reports::span("System Security Status",format!("color: {};", reports::get_security_status_color(security)),format!("({:.2})", security))
                )
            );
            reports::table_cell(output, "Victim Faction/Alliance/Corporation/Character", text_style,
                format!("{}<br/>{}<br/>{}<br/>{}",
                    reports::span("Faction", "", ctx.get_api_link("faction", victim.get_name("faction"))),
                    reports::span("Alliance", "", ctx.get_api_link("alliance", victim.get_name("alliance"))),
                    reports::span("Corporation", "", ctx.get_api_link("corporation", victim.get_name("corporation"))),
                    reports::span("Character", "", ctx.get_api_link("character", victim.get_name("character")))
                )
            );
            if let Some(ref attackers) = attackers {
                reports::table_cell(output, "Attackers Count", text_style, attackers.len().separated_string());
                for attacker in attackers {
                    if attacker.final_blow {
                        reports::table_cell(output, "Victim Faction/Alliance/Corporation/Character", text_style,
                            format!("{}<br/>{}<br/>{}<br/>{}",
                                reports::span("Faction", "", ctx.get_api_link("faction", attacker.get_name("faction"))),
                                reports::span("Alliance", "", ctx.get_api_link("alliance", attacker.get_name("alliance"))),
                                reports::span("Corporation", "", ctx.get_api_link("corporation", attacker.get_name("corporation"))),
                                reports::span("Character", "", ctx.get_api_link("character", attacker.get_name("character")))
                            )
                        );
                    }
                }
            }
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
                format!("Location: {} : {} : {} ({:0.1})",
                    ctx.get_api_link("region", system.get_name("region")),
                    ctx.get_api_link("constellation", system.get_name("constellation")),
                    ctx.get_api_link("system", system.get_name("system")),
                    system.security_status
                )
            );
        }
        let table_style   = "border-collapse: collapse;";
        let head_style    = "border: 1px solid black; padding: 2px 5px; text-align: center; ";
        let text_style    = "border: 1px solid black; padding: 2px 5px;";
        let numeric_style = "border: 1px solid black; padding: 2px 5px;; text-align: right;";

        let total_amount = Self::get_total_sum(&items, &victim);
        let dropped_amount = Self::get_dropped_sum(&items);
        if let Some(victim) = victim {
            reports::table_start(output, "Victim", table_style, "Victim");
            reports::table_row_start(output, "");
            reports::table_cell_head(output, "Total Amount", head_style, "Total");
            reports::table_cell_head(output, "Dropped Amount", head_style, "Dropped");
            reports::table_cell_head(output, "Damage Taken", head_style, "Damage");
            reports::table_cell_head(output, "Ship Type", head_style, "Ship Type");
            reports::table_cell_head(output, "Faction Name", head_style, "Faction");
            reports::table_cell_head(output, "Alliance Name", head_style, "Alliance");
            reports::table_cell_head(output, "Corporation Name", head_style, "Corporation");
            reports::table_cell_head(output, "Character Name", head_style, "Character");
            reports::table_row_end(output);
            reports::table_row_start(output, "");

            let total_amount_style  = format!("{} background-color: {};", numeric_style, Self::volume_color(&total_amount));
            let dropped_amount_style  = format!("{} background-color: {};", numeric_style, Self::volume_color(&dropped_amount));

            reports::table_row_end(output);
            reports::table_cell(output, "Total Amount", total_amount_style, total_amount.separated_string());
            reports::table_cell(output, "Dropped Amount", dropped_amount_style, dropped_amount.separated_string());
            reports::table_cell(output, "Damage Taken", numeric_style, victim.damage_taken.separated_string());
            reports::table_cell(output, "Ship Type", text_style, ctx.get_zkb_href("ship", victim.get_id("ship"), victim.get_name("ship")));
            reports::table_cell(output, "Faction Name", text_style, ctx.get_api_link("faction", victim.get_name("faction")));
            reports::table_cell(output, "Alliance Name", text_style, ctx.get_api_link("alliance", victim.get_name("alliance")));
            reports::table_cell(output, "Corporation Name", text_style, ctx.get_api_link("corporation", victim.get_name("corporation")));
            reports::table_cell(output, "Character Name", text_style, ctx.get_api_link("character", victim.get_name("character")));
            reports::table_end(output);
        }

        if let Some(attackers) = attackers {
            reports::table_start(output, "Attackers", table_style, "Attackers");
            reports::table_row_start(output, "");
            reports::table_cell_head(output, "Security Status", head_style, "SS");
            reports::table_cell_head(output, "Final Blow", head_style, "Final");
            reports::table_cell_head(output, "Damage Done", head_style, "Damage");
            reports::table_cell_head(output, "Weapon", head_style, "Weapon");
            reports::table_cell_head(output, "Ship Type", head_style, "Ship Type");
            reports::table_cell_head(output, "Faction Name", head_style, "Faction");
            reports::table_cell_head(output, "Alliance Name", head_style, "Alliance");
            reports::table_cell_head(output, "Corporation Name", head_style, "Corporation");
            reports::table_cell_head(output, "Character Name", head_style, "Character");

            reports::table_row_end(output);
            for attacker in attackers {
                reports::table_row_start(output, Self::npc_attacker_style(&attacker));
                reports::table_cell(output, "Security Status", text_style, attacker.security_status.separated_string());
                reports::table_cell(output, "Final Blow", text_style, attacker.final_blow.to_string());
                reports::table_cell(output, "Damage Done", numeric_style, attacker.damage_done.separated_string());
                reports::table_cell(output, "Weapon", text_style, ctx.get_zkb_href("item", attacker.get_id("weapon"), attacker.get_name("weapon")));
                reports::table_cell(output, "Ship Type", text_style, ctx.get_zkb_href("ship", attacker.get_id("ship"), attacker.get_name("ship")));
                reports::table_cell(output, "Faction Name", text_style, ctx.get_api_link("faction", attacker.get_name("faction")));
                reports::table_cell(output, "Alliance Name", text_style, ctx.get_api_link("alliance", attacker.get_name("alliance")));
                reports::table_cell(output, "Corporation Name", text_style, ctx.get_api_link("corporation", attacker.get_name("corporation")));
                reports::table_cell(output, "Character Name", text_style, ctx.get_api_link("character", attacker.get_name("character")));
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
