use crate::models::*;
use crate::services::{Context, Report, Category};
use crate::reports;
use crate::provider;
use separator::Separatable;
use killmail::KillmailNamed;

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
    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        let killmail = KillmailNamed::load(conn, id)?;
        Ok(Self {
            killmail_id: killmail.killmail_id,
            killmail_time: killmail.killmail_time,
            system_id: killmail.system_id,
            system_name: killmail.system_name,
            constellation_id: killmail.constellation_id,
            constellation_name: killmail.constellation_name,
            region_id: killmail.region_id,
            region_name: killmail.region_name,
        })
    }

    fn get_cost(id: &Integer, ctx: &Context)-> (u64, u64) {
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
        else if *value > 100_000_000 {"Tomato"}
        else if *value > 1_000_000 {"LightPink"}
        else {"WhiteSmoke"}
        .to_string()
    }

    pub fn write(output: &mut dyn Write, killmail: &killmail::KillmailNamed, ctx: &Context) {
        let sums = Self::get_cost(&killmail.get_id("id"), ctx);
        let mut security = 0.0;
        if let Report::System(system) = reports::load(Category::System(killmail.system_id), &ctx) {
            security = system.security_status;
        }
        let security_status = format!(
            r##"
                <span title="System Security Status" style = "color: {};">{:.1}</span>
            "##,
            Self::security_status_color(security),
            security
        );
        let dropped = format!(
                r##"
                    <span title="Dropped Value" style = "display: inline-block; width: 115px; text-align: right; background-color: {};">
                    {}
                    </span>"##,
                    Self::volume_color(&sums.0),
                    sums.0.separated_string()
        );
        let total = format!(
                r##"
                    <span title="Total Value" style = "display: inline-block; width: 125px; text-align: right; background-color: {};">
                    {}
                    </span>"##,
                    Self::volume_color(&sums.1),
                    sums.1.separated_string()
        );
        let content = format!(
                r##"
                    {timestamp} [{api}] [{zkb}]
                    {total}
                    {dropped}
                    {region} : {constellation} : {system}
                    ({security_status})
                "##,
                timestamp = killmail.killmail_time.to_string(),
                api = ctx.get_api_href("killmail", killmail.get_id("id"), format!("api")),
                zkb = ctx.get_zkb_href("kill", killmail.get_id("id"), format!("zkb")),
                region = ctx.get_api_href("region", killmail.get_id("region"), killmail.get_name("region")),
                constellation = ctx.get_api_href("constellation", killmail.get_id("constellation"), killmail.get_name("constellation")),
                system = ctx.get_api_href("system", killmail.get_id("system"), killmail.get_name("system")),
                security_status = security_status,
                dropped = dropped,
                total = total,
        );
        reports::div(output, content);
    }

    pub fn brief(arg: &String, ctx: &Context) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::brief_impl(id, ctx)
        } else {
            format!("<div>Killmail {} was not found</div>", arg)
        }
    }

    pub fn brief_impl(id: &Integer, ctx: &Context) -> String {
        let mut output = String::new();
        match reports::load(Category::Killmail(*id), &ctx) {
            Report::Killmail(killmail) => Self::write(&mut output, &killmail, &ctx),
            Report::NotFoundId(id) => reports::div(&mut output, format!("<div>Killmail {} was not found</div>", id)),
            report => warn!("Unexpected report {:?}", report)
        }
        return output;
    }
}
