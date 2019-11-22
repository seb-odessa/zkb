use crate::api;
use super::{FAIL};
use crate::services::{Context, Area, Category, Model, Report, Filter};
use crate::reports::*;
use crate::reports;
use crate::models;
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
                    ctx.get_api_href("system", neighbor.neighbor_id, neighbor.get_neighbor_name()),
                ));
            }
        }
    }

    fn load_constellation_observatory(id: &i32, ctx: &Context) -> Vec<models::system::SystemNamed> {
        let area = Area::Constellation(*id);
        let filter = Filter::WithJovianObservatoryOnly;
        if let Report::Systems(systems) = reports::load(Category::Systems((area, filter)), &ctx) {
            systems
        } else {
            Vec::new()
        }
    }

    fn load_neighbor_observatories(system: &models::system::SystemNamed, ctx: &Context) -> Vec<models::system::SystemNamed> {
        let mut systems = Self::load_constellation_observatory(&system.constellation_id, ctx);
        if let Report::ConstellationNeighbors(neighbors) = reports::load(Category::Neighbors(Area::Constellation(system.constellation_id)), &ctx) {
            for neighbor in &neighbors {
                let mut other = Self::load_constellation_observatory(&neighbor.neighbor_id, ctx);
                systems.append(&mut other);
            }
        }
        return systems;
    }

    fn get_system_href(id: &i32, ctx: &Context) -> String {
        match reports::load(Category::System(*id), ctx) {
            Report::System(system) => ctx.get_api_href("system", system.system_id, system.get_system_name()),
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

    fn observatory_report(output: &mut dyn Write, id: &i32, ctx: &Context) {
        match reports::load(Category::System(*id), &ctx) {
            Report::System(system) => {
                if system.has_observatory() {
                    div(output, format!(r#"<span style="color: green;">Jovian Observatory</span>"#));
                }
                jovian_buttons(output, &system.system_id, &system.get_system_name());
                div(output, format!("Nearest system with Jovian Observatory:"));
                for neighbor in &Self::load_neighbor_observatories(&system, ctx) {
                    lazy(output, format!("services/route/{}/{}", system.system_id, neighbor.system_id), &ctx);
                }
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

