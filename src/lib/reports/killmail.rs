use crate::models::*;
use crate::services::Context;
use crate::reports::{div, System, FAIL};
use crate::reports;

use killmail::KillmailNamed;
use item::ItemNamed;

use std::fmt::Write;
use std::fmt::write;

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

    fn get_cost(id: &Integer, ctx: &Context)-> (i32, i32) {
        use crate::services::*;
        let mut destroyed = 0;
        let mut dropped = 0;
        if let Report::Items(items) = reports::load(Category::Items(*id), &ctx) {
            for item in &items {
                if let Some(ref quantity) = item.quantity_destroyed {
                    destroyed = destroyed + (*quantity as f32 * 1.0) as i32;
                }
                if let Some(ref quantity) = item.quantity_dropped {
                    dropped = dropped + (*quantity as f32  * 1.0) as i32;
                }
            }
        }
        (dropped, dropped + destroyed)
    }

    pub fn write(output: &mut dyn Write, killmail: &killmail::KillmailNamed, ctx: &Context) {
        let sums = Self::get_cost(&killmail.get_id("id"), ctx);
        write(
            output,
            format_args!(
                r##"
                    <div>{timestamp} [{api}] [{zkb}] {region} : {constellation} : {system} <span>{status}</span> {dropped}/{total}</div>
                "##,
                timestamp = killmail.killmail_time.to_string(),
                api = ctx.get_api_href("killmail", killmail.get_id("id"), format!("api")),
                zkb = ctx.get_zkb_href("kill", killmail.get_id("id"), format!("zkb")),
                region = ctx.get_api_href("region", killmail.get_id("region"), killmail.get_name("region")),
                constellation = ctx.get_api_href("constellation", killmail.get_id("constellation"), killmail.get_name("constellation")),
                system = ctx.get_api_href("system", killmail.get_id("system"), killmail.get_name("system")),
                status = format!("({})", System::security_status(&killmail.system_id)),
                dropped = sums.0,
                total = sums.1
            )
        ).expect(FAIL);
    }

    pub fn brief(arg: &String, ctx: &Context) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::brief_impl(id, ctx)
        } else {
            format!("<div>Killmail {} was not found</div>", arg)
        }
    }

    pub fn brief_impl(id: &Integer, ctx: &Context) -> String {
        use crate::services::*;
        let mut output = String::new();
        match reports::load(Category::Killmail(*id), &ctx) {
            Report::Killmail(killmail) => Self::write(&mut output, &killmail, &ctx),
            Report::NotFoundId(id) => div(&mut output, format!("<div>Killmail {} was not found</div>", id)),
            report => warn!("Unexpected report {:?}", report)
        }
        return output;
    }
}
