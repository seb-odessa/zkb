use crate::api;
use super::{FAIL};
use crate::models;
use crate::services;
use crate::services::Context;
//use crate::services::{Context, Area, Category, Model, Report};
//use crate::reports::*;
use crate::reports;
use crate::provider;

use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct System;
impl System {
    // pub fn write(output: &mut dyn Write, system: &models::system::System, _ctx: &Context) {
    //     reports::div(output, format!("{}", victim.get_name("character")));
    // }

    pub fn write(output: &mut dyn Write, system: &api::system::System, ctx: &Context) {
        let root = reports::root(ctx);
        let url = format!("{}/api/system/{}", root, system.system_id);
        let name = format!("{} ({:.2})", system.name, system.security_status);
        std::fmt::write(
            output,
            format_args!(
                r#"<div id="{id}" data-name="{name}">System: {url} {zkb}</div>"#,
                id = system.system_id,
                url = reports::href(&url, &name),
                zkb = reports::href(system.zkb(), String::from("(zkb)")),
                name = system.name
            )
        ).expect(FAIL);
    }

    pub fn load(id: &i32, ctx: &Context) -> Option<models::system::SystemNamed> {
        use services::{Category, Report};
        match reports::load(Category::System(*id), &ctx) {
            Report::System(system) => return Some(system),
            Report::NotFoundId(id) => warn!("{} was not found", id),
            report => warn!("Unexpected report {:?}", report)
        }
        return None;
    }

    fn neighbors(output: &mut dyn Write, id: &i32, ctx: &Context) {
        use services::{Category, Report, Area};
        use reports::history::History;
        if let Report::SystemNeighbors(neighbors) = reports::load(Category::Neighbors(Area::System(*id)), &ctx) {
            for neighbor in &neighbors {
                reports::div(output, format!("neighbor: [ {} : {} : {} ] {}",
                    reports::tip("Kills at last 10 minutes", format!("{:0>3}", History::system_count(&neighbor.neighbor_id, &10, ctx))),
                    reports::tip("Kills at last 60 minutes", format!("{:0>3}", History::system_count(&neighbor.neighbor_id, &60, ctx))),
                    reports::tip("Kills at last 6 hours", format!("{:0>3}", History::system_count(&neighbor.neighbor_id, &360, ctx))),
                    ctx.get_api_href("system", neighbor.neighbor_id, neighbor.get_name("neighbor")),
                ));
            }
        }
    }

    fn get_system_href(id: &i32, ctx: &Context) -> String {
        use services::{Category, Report};
        match reports::load(Category::System(*id), ctx) {
            Report::System(system) => ctx.get_api_href("system", system.system_id, system.get_name("system")),
            _ => String::from("...Unknown...")
        }
    }

    // todo create api type for route
    fn get_route(departure: &i32, destination: &i32) -> Option<Vec<i32>> {
        let uri = format!("route/{}/{}", departure, destination);
        let response = api::gw::eve_api(&uri).unwrap_or_default();
        serde_json::from_str(&response).ok()
    }

    pub fn route(departure: i32, destination: i32, ctx: &Context) -> String {
        let mut path = String::new();
        if let Some(route) = provider::get_route(&departure, &destination, &Self::get_route) {
            for id in route.iter().skip(1) {
                if path.is_empty() {
                    path = Self::get_system_href(&id, ctx);
                } else {
                    path = path + " &gt; " + &Self::get_system_href(&id, ctx);
                }
            }
        }
        return path;
    }

    fn report_observatory_path(output: &mut dyn Write, id: &i32, ctx: &Context) {
        use services::{Category, Report};
        use std::collections::HashSet;
        let arrow = format!("&nbsp;&gt;&nbsp;");
        let mut covered = HashSet::new();
        match reports::load(Category::ObservatoryPath(*id), &ctx) {
            Report::ObservatoryPath(paths) => {
                for path in &paths {
                    if path.s1_jo {
                        if covered.insert(&path.s1_id) {
                            reports::div(output, format!("{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx)
                            ));
                        }
                    } else if path.s2_jo {
                        if covered.insert(&path.s2_id) {
                            reports::div(output, format!("{}{}{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx),
                                arrow, Self::get_system_href(&path.s2_id, ctx),
                            ));
                        }
                    } else if path.s3_jo {
                        if covered.insert(&path.s3_id) {
                            reports::div(output, format!("{}{}{}{}{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx),
                                arrow, Self::get_system_href(&path.s2_id, ctx),
                                arrow, Self::get_system_href(&path.s3_id, ctx),
                            ));
                        }
                    } else if path.s4_jo {
                        if covered.insert(&path.s4_id) {
                            reports::div(output, format!("{}{}{}{}{}{}{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx),
                                arrow, Self::get_system_href(&path.s2_id, ctx),
                                arrow, Self::get_system_href(&path.s3_id, ctx),
                                arrow, Self::get_system_href(&path.s4_id, ctx),
                            ));
                        }
                    } else if path.s5_jo {
                        if covered.insert(&path.s5_id) {
                            reports::div(output, format!("{}{}{}{}{}{}{}{}{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx),
                                arrow, Self::get_system_href(&path.s2_id, ctx),
                                arrow, Self::get_system_href(&path.s3_id, ctx),
                                arrow, Self::get_system_href(&path.s4_id, ctx),
                                arrow, Self::get_system_href(&path.s5_id, ctx),
                            ));
                        }
                    }
                }
            },
            report => {
                reports::div(output, format!("Unexpected Report: {:?}", report));
            }
        }
    }

    fn observatory_report(output: &mut dyn Write, id: &i32, ctx: &Context) {
        use services::{Category, Report};
        match reports::load(Category::System(*id), &ctx) {
            Report::System(system) => {
                if system.has_observatory() {
                    reports::div(output, format!(r#"<span style="color: green;">Jovian Observatory</span>"#));
                }
                reports::jovian_buttons(output, &system.system_id, &system.get_name("system"));
                reports::div(output, format!("Nearest system with Jovian Observatory:"));
                Self::report_observatory_path(output, id, ctx);
            },
            Report::NotFoundId(id) => reports::div(output, format!("System {} was not found", id)),
            report => warn!("Unexpected report {:?}", report)
        }
    }

    pub fn observatory_add(id: &i32, ctx: &Context) -> String {
        use services::{Message, Model};
        ctx.database.push(Message::Save(Model::Observatory(*id)));
        String::from("Done")
    }

    pub fn observatory_remove(id: &i32, ctx: &Context) -> String {
        use services::{Message, Model};
        ctx.database.push(Message::Delete(Model::Observatory(*id)));
        String::from("Done")
    }

    pub fn brief(arg: &String, ctx: &Context) -> String {
        Self::perform_report(arg, ctx, reports::ReportType::Brief)
    }

    pub fn report(arg: &String, ctx: &Context) -> String {
        Self::perform_report(arg, ctx, reports::ReportType::Full)
    }

    fn perform_report(arg: &String, ctx: &Context, report_type: reports::ReportType) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::report_by_id(id, ctx, report_type)
        } else if let Some(ref id) = reports::find_id("solar_system", arg, ctx) {
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

    fn report_by_id(id: &i32, ctx: &Context, full_report: reports::ReportType) -> String {
        let mut output = String::new();
        if let Some(object) = api::system::System::new(id) {
            Self::write(&mut output, &object, ctx);
            if full_report == reports::ReportType::Full {
                reports::lazy(&mut output, format!("api/constellation_brief/{}", object.constellation_id), &ctx);
                reports::lazy(&mut output, format!("api/region_brief/{}", object.get_region_id().unwrap_or_default()), &ctx);
                Self::neighbors(&mut output, &object.system_id, &ctx);
                Self::observatory_report(&mut output, &object.system_id, &ctx);
                reports::lazy(&mut output, format!("history/system/{}/{}", id, 60), &ctx);
            }
        } else {
            reports::div(&mut output, format!("Can't query System({}) from CCP API", id));
        }
        return output;
    }
}

