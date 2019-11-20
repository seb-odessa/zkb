use crate::api;
use super::{FAIL};
use crate::services::{Context, Area, Category, Report};
use crate::reports::*;
use crate::reports;

use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct System;
impl System {

    pub fn write(output: &mut dyn Write, system: &api::system::System, root: &String) {
        let url = format!("{}/api/system/{}", root, system.system_id);
        let name = format!("{} ({:.2})", system.name, system.security_status);
        std::fmt::write(
            output,
            format_args!(
                r#"<div id="{id}" data-name="{name}">System: {url} {zkb}</div>"#,
                id = system.system_id,
                url = href(&url, &name),
                zkb = href(system.zkb(), String::from("(zkb)")),
                name = system.name
            )
        ).expect(FAIL);
    }

    fn neighbors(output: &mut dyn Write, id: &i32, ctx: &Context) {
        let root = root(&ctx);
        let msg_id = crate::create_id().to_simple();
        let empty = String::new();
        ctx.database.push(Message::Find((msg_id, Category::Neighbors(Area::System(*id)))));
        if let Report::SystemNeighbors(neighbors) = reports::wait_for(msg_id, &ctx) {
            for neighbor in &neighbors {
                let url = format!("{}/api/system/{}", root, neighbor.neighbor_id);
                let name = neighbor.neighbor_name.as_ref().unwrap_or(&empty);
                div(output, format!("neighbor: [ {} : {} : {} ] {}",
                    tip("Kills at last 10 minutes", format!("{:0>3}", neighbor.ten_minutes)),
                    tip("Kills at last 60 minutes", format!("{:0>3}", neighbor.one_hour)),
                    tip("Kills at last 6 hours", format!("{:0>3}", neighbor.six_hours)),
                    href(&url, name),
                ));
            }
        }
    }

    pub fn brief(arg: &String, ctx: &Context) -> String {
        Self::perform_report(arg, ctx, ReportType::Brief)
    }

    pub fn report(arg: &String, ctx: &Context) -> String {
        Self::perform_report(arg, ctx, ReportType::Full)
    }

    fn perform_report(arg: &String, ctx: &Context, report_type: ReportType) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::report_by_id(id, ctx, report_type)
        } else if let Some(ref id) = find_id("solar_system", arg, ctx) {
            Self::report_by_id(id, ctx, report_type)
        } else {
            format!("<div>System {} was not found in DB</div>", arg)
        }
    }

    pub fn security_status(id: &i32) -> String {
        if let Some(system) = api::system::System::new(id) {
            format!("{:.2}", system.security_status)
        } else {
            format!("Can't query System({}) from CCP API", id)
        }
    }

    fn report_by_id(id: &i32, ctx: &Context, full_report: ReportType) -> String {
        let mut output = String::new();
        if let Some(object) = api::system::System::new(id) {
            Self::write(&mut output, &object, &root(ctx));
            if full_report == ReportType::Full {
                lazy(&mut output, format!("api/constellation_brief/{}", object.constellation_id), &ctx);
                lazy(&mut output, format!("api/region_brief/{}", object.get_region_id().unwrap_or_default()), &ctx);
                Self::neighbors(&mut output, &object.system_id, &ctx);
                // if let Some(ref gates) = &object.stargates {
                //     for gate_id in gates {
                //         if let Some(object) = api::stargate::Stargate::new(gate_id) {
                //             lazy(&mut output, format!("api/system_brief/{}", object.destination.system_id), &ctx);
                //         }
                //     }
                // }
                jovian_buttons(&mut output, &object.system_id, &object.name);
                lazy(&mut output, format!("history/system/{}/{}", id, 60), &ctx);
            }
        } else {
            div(&mut output, format!("Can't query System({}) from CCP API", id));
        }
        return output;
    }
}

