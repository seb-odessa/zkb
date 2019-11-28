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
        let mut result = 0;
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

    fn get_cost(id: &Integer, ctx: &Context)-> (u64, u64) {
        use services::{Category, Report};
        let mut destroyed = 0;
        let mut dropped = 0;
        if let Report::Items(items) = reports::load(Category::Items(*id), &ctx) {
            for item in &items {
                if let Some(ref price) = provider::get_avg_price(&Some(item.item_type_id)){
                    if let Some(ref quantity) = item.quantity_destroyed {
                        destroyed = destroyed + (*quantity as f32 * *price) as u64;
                    }
                    if let Some(ref quantity) = item.quantity_dropped {
                        dropped = dropped + (*quantity as f32  * *price) as u64;
                    }
                }
            }
        }
        if let Report::Victim(victim) = reports::load(Category::Victim(*id), &ctx) {
            if let Some(price) = provider::get_avg_price(&Some(victim.ship_id)){
                destroyed = destroyed + price as u64;
            }
        }
        (dropped, dropped + destroyed)
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

    pub fn write(output: &mut dyn Write, killmail: &killmail::KillmailNamed, ctx: &Context) {
        let killmail_id = killmail.killmail_id;
        use services::{Category, Report};
        let sums = Self::get_cost(&killmail_id, ctx);
        let mut security = 0.0;
        if let Report::System(system) = reports::load(Category::System(killmail.system_id), &ctx) {
            security = system.security_status;
        }
        let security_status = reports::span(
            "System Security Status",
            format!("color: {};", Self::security_status_color(security)),
            format!("{:.1}", security),
        );

        let dropped = reports::span(
            "Dropped Value",
            format!("display: inline-block; width: 115px; text-align: right; background-color: {};", Self::volume_color(&sums.0)),
            sums.0.separated_string(),
        );

        let total = reports::span(
            "Total Kill Mail Value",
            format!("display: inline-block; width: 125px; text-align: right; background-color: {};", Self::volume_color(&sums.1)),
            sums.1.separated_string(),
        );

        let content = format!(
                r##"
                    {time} [{zkb}]
                    {total}
                    {dropped}
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
                dropped = dropped,
                total = total,
                victim = reports::Victim::report_name(&killmail_id, ctx),
        );
        reports::div(output, content);
    }

    fn write_report(output: &mut dyn Write, killmail: &killmail::KillmailNamed, ctx: &Context) {
        let killmail_id = killmail.killmail_id;
        let victim = reports::Victim::load(&killmail_id, ctx);
        let attackers = reports::Attacker::load(&killmail_id, ctx);
        let system = reports::System::load(&killmail.system_id, ctx);
        let items = reports::Item::load(&killmail_id, ctx);
        {
            let content = format!("{timestamp} [{zkb}]",
                timestamp = ctx.get_api_href("killmail", killmail_id, killmail.killmail_time.to_string()),
                zkb = ctx.get_zkb_href("kill", killmail.get_id("id"), format!("zkb")),
            );
            reports::div(output, content);
        }
        {
            reports::div(output, format!("Total killmail value: {}", Self::get_total_sum(&items, &victim).separated_string()));
            reports::div(output, format!("Dropped value: {}", Self::get_dropped_sum(&items).separated_string()));
        }
        if let Some(victim) = victim {
            {
                let class = "ship";
                reports::div(output, format!("Ship: {}", ctx.get_api_href(class, victim.get_id(class), victim.get_name(class))));
            }
            reports::div(output, format!("Damage Taken: {}", victim.damage_taken));
            {
                let class = "character";
                reports::div(output, format!("Character: {}", ctx.get_api_href(class, victim.get_id(class), victim.get_name(class))));
            }
            {
                let class = "corporation";
                reports::div(output, format!("Corporation: {}", ctx.get_api_href(class, victim.get_id(class), victim.get_name(class))));
            }
            {
                let class = "alliance";
                reports::div(output, format!("Alliance: {}", ctx.get_api_href(class, victim.get_id(class), victim.get_name(class))));
            }
            {
                let class = "faction";
                reports::div(output, format!("Faction: {}", ctx.get_api_href(class, victim.get_id(class), victim.get_name(class))));
            }
        }
        if let Some(system) = system {
            {
                let class = "system";
                reports::div(output, format!("System: {}", ctx.get_api_href(class, system.get_id(class), system.get_name(class))));
            }
            {
                let class = "Constellation";
                reports::div(output, format!("constellation: {}", ctx.get_api_href(class, system.get_id(class), system.get_name(class))));
            }
            {
                let class = "region";
                reports::div(output, format!("Region: {}", ctx.get_api_href(class, system.get_id(class), system.get_name(class))));
            }
        }

        let table_style = "border: 1px solid black;";
        let head_style = "text-align: center; border: 1px solid black;";
        let text_style = "padding: 5px; border: 1px solid black;";
        let numeric_style = "padding: 5px; text-align: right; border: 1px solid black;";

        if let Some(attackers) = attackers {
            reports::div(output, format!("Attackers:"));
            reports::table_start(output, "Attackers", table_style);
            reports::table_row_start(output, "");
            reports::table_cell_head(output, "Security Status", head_style, "SS");
            reports::table_cell_head(output, "Final Blow", head_style, "Final");
            reports::table_cell_head(output, "Damage Done", head_style, "Damage");
            reports::table_cell_head(output, "Ship Type", head_style, "Ship Type");
            reports::table_cell_head(output, "Character Name", head_style, "Character");
            reports::table_cell_head(output, "Corporation Name", head_style, "Corporation");
            reports::table_cell_head(output, "Alliance Name", head_style, "Alliance");
            reports::table_cell_head(output, "Faction Name", head_style, "Faction");
            reports::table_row_end(output);
            for attacker in attackers {
                reports::table_row_start(output, "");
                reports::table_cell(output, "Security Status", text_style, attacker.security_status.separated_string());
                reports::table_cell(output, "Final Blow", text_style, attacker.final_blow.to_string());
                reports::table_cell(output, "Damage Done", numeric_style, attacker.damage_done.separated_string());
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
                name: String,
                dropped: u64,
                destroyed: u64,
                price: Option<f32>,
            }
            impl ZippedItem {
                fn new(id: i32, name: String) -> Self {
                    Self{ id: id, name: name, dropped: 0, destroyed: 0, price: None }
                }
            }
            use std::collections::BTreeMap;
            let mut zipped_map = BTreeMap::new();
            for item in items {
                let id = item.get_id();
                let name = item.get_name();
                let mut zipped = zipped_map.entry(name.clone()).or_insert(ZippedItem::new(id, name));
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

            reports::div(output, format!("Items:"));
            reports::table_start(output, "Items", table_style);
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
                reports::table_cell(output, "Item's average price", numeric_style, desc.price.clone().unwrap_or(0.0).separated_string());
                reports::table_cell(output, "Dropped quantity", numeric_style, (desc.price.clone().unwrap_or(0.0) * desc.dropped as f32).separated_string());
                reports::table_cell(output, "Destroyed quantity", numeric_style, (desc.price.clone().unwrap_or(0.0) * desc.destroyed as f32).separated_string());
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
