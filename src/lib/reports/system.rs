use crate::api;
use super::{FAIL};
use crate::services::{Context, Area, Category, Model, Report};
use crate::reports::*;
use crate::reports;
use crate::provider;

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
        if let Report::SystemNeighbors(neighbors) = reports::load(Category::Neighbors(Area::System(*id)), &ctx) {
            for neighbor in &neighbors {
                div(output, format!("neighbor: [ {} : {} : {} ] {}",
                    tip("Kills at last 10 minutes", format!("{:0>3}", history::History::system_count(&neighbor.neighbor_id, &10, ctx))),
                    tip("Kills at last 60 minutes", format!("{:0>3}", history::History::system_count(&neighbor.neighbor_id, &60, ctx))),
                    tip("Kills at last 6 hours", format!("{:0>3}", history::History::system_count(&neighbor.neighbor_id, &360, ctx))),
                    ctx.get_api_href("system", neighbor.neighbor_id, neighbor.get_name("neighbor")),
                ));
            }
        }
    }

    fn get_system_href(id: &i32, ctx: &Context) -> String {
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
        use std::collections::HashSet;
        let arrow = format!("&nbsp;&gt;&nbsp;");
        let mut covered = HashSet::new();
        match reports::load(Category::ObservatoryPath(*id), &ctx) {
            Report::ObservatoryPath(paths) => {
                for path in &paths {
                    if path.s1_jo {
                        if covered.insert(&path.s1_id) {
                            div(output, format!("{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx)
                            ));
                        }
                    } else if path.s2_jo {
                        if covered.insert(&path.s2_id) {
                            div(output, format!("{}{}{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx),
                                arrow, Self::get_system_href(&path.s2_id, ctx),
                            ));
                        }
                    } else if path.s3_jo {
                        if covered.insert(&path.s3_id) {
                            div(output, format!("{}{}{}{}{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx),
                                arrow, Self::get_system_href(&path.s2_id, ctx),
                                arrow, Self::get_system_href(&path.s3_id, ctx),
                            ));
                        }
                    } else if path.s4_jo {
                        if covered.insert(&path.s4_id) {
                            div(output, format!("{}{}{}{}{}{}{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx),
                                arrow, Self::get_system_href(&path.s2_id, ctx),
                                arrow, Self::get_system_href(&path.s3_id, ctx),
                                arrow, Self::get_system_href(&path.s4_id, ctx),
                            ));
                        }
                    } else if path.s5_jo {
                        if covered.insert(&path.s5_id) {
                            div(output, format!("{}{}{}{}{}{}{}{}{}{}",
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
                div(output, format!("Unexpected Report: {:?}", report));
            }
        }
    }

    fn observatory_report(output: &mut dyn Write, id: &i32, ctx: &Context) {
        match reports::load(Category::System(*id), &ctx) {
            Report::System(system) => {
                if system.has_observatory() {
                    div(output, format!(r#"<span style="color: green;">Jovian Observatory</span>"#));
                }
                jovian_buttons(output, &system.system_id, &system.get_name("system"));
                div(output, format!("Nearest system with Jovian Observatory:"));
                Self::report_observatory_path(output, id, ctx);
            },
            Report::NotFoundId(id) => div(output, format!("System {} was not found", id)),
            report => warn!("Unexpected report {:?}", report)
        }
    }

    pub fn observatory_add(id: &i32, ctx: &Context) -> String {
        ctx.database.push(Message::Save(Model::Observatory(*id)));
        String::from("Done")
    }

    pub fn observatory_remove(id: &i32, ctx: &Context) -> String {
        ctx.database.push(Message::Delete(Model::Observatory(*id)));
        String::from("Done")
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
                Self::observatory_report(&mut output, &object.system_id, &ctx);
                lazy(&mut output, format!("history/system/{}/{}", id, 60), &ctx);
            }
        } else {
            div(&mut output, format!("Can't query System({}) from CCP API", id));
        }
        return output;
    }
}

