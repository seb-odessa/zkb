use crate::api;
use super::{FAIL};
use crate::services::*;
use crate::reports::*;
use chrono::Utc;

use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct System;
impl System {

    pub fn write(output: &mut dyn Write, system: &api::system::System, root: &String) {
        let url = format!("{}/api/system/{}", root, system.system_id);
        let name = format!("{} ({:.1})", system.name, system.security_status);
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

    fn report_by_id(id: &i32, ctx: &Context, full_report: ReportType) -> String {
        let mut output = String::new();
        if let Some(object) = api::system::System::new(id) {
            Self::write(&mut output, &object, &root(ctx));
            if full_report == ReportType::Full {
                lazy(&mut output, format!("api/constellation_brief/{}", object.constellation_id), &ctx);
                lazy(&mut output, format!("api/region/{}", object.get_region_id().unwrap_or_default()), &ctx);
                if let Some(ref gates) = &object.stargates {
                    for gate_id in gates {
                        if let Some(object) = api::stargate::Stargate::new(gate_id) {
                            lazy(&mut output, format!("api/system_brief/{}", object.destination.system_id), &ctx);
                        }
                    }
                }
                jovian_buttons(&mut output, &object.system_id, &object.name);
                let now = Utc::now().naive_utc().time().format("%H:%M:%S").to_string();
                div(&mut output, format!("Kill history 60 minutes since {} ", &now));
                lazy(&mut output, format!("history/system/{}/{}", id, 60), &ctx);
            }
        } else {
            div(&mut output, format!("Can't query System({}) from CCP API", id));
        }
        return output;
    }
}

