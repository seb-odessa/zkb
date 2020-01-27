use crate::models;
use crate::services;
use crate::services::Context;
use crate::reports;
use crate::separator::Separatable;
use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct System;
impl reports::ReportableEx for System {

    fn get_category() -> String {
        String::from("solar_system")
    }

    fn report_by_id(id: &i32, ctx: &Context, report_type: reports::ReportType) -> String {
        let mut output = String::new();
        if let Some(system) = Self::load(id, ctx) {
            Self::write(&mut output, &system, ctx);
            if report_type == reports::ReportType::Full {
                reports::lazy(&mut output, format!("api/constellation_brief/{}", system.get_id("constellation")), &ctx);
                reports::lazy(&mut output, format!("api/region_brief/{}", system.get_id("region")), &ctx);
                Self::neighbors(&mut output, &id, &ctx);
                Self::observatory_report(&mut output, &system, &ctx);
                reports::systems(&mut output, &system.get_id("constellation"), &ctx);
                reports::constellations(&mut output, &system.get_id("region"), &ctx);
                reports::lazy(&mut output, format!("history/system/{}/{}", id, 60), &ctx);
                //reports::map(&mut output, "json/nodes/a", "json/edges/a", &ctx);
            }
        }
        return output;
    }
}
impl System {

    pub fn write(output: &mut dyn Write, system: &models::system::SystemNamed, ctx: &Context) {
        let id = system.get_id("system");
        let name = system.get_name("system");
        let content = format!(r#"<span id="{id}" data-name="{name}">System: {desc} [{map}]</span>"#,
            id = system.system_id,
            name = &name,
            desc = ctx.get_place_desc("system", id, &name),
            map = ctx.get_dotlan_href(system.get_name("region"), &name, "dotlan")
        );
        reports::div(output, content);
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
        use services::{Category, Report, Area, Message, Api};
        use reports::history::History;
        if let Report::SystemNeighbors(neighbors) = reports::load(Category::Neighbors(Area::System(*id)), &ctx) {
            for neighbor in &neighbors {
                let id = neighbor.get_id("neighbor");
                let name = neighbor.get_name("neighbor");
                if name.is_empty() {
                    ctx.resolver.push(Message::Receive(Api::Object(id)));
                }
                reports::div(output, format!("neighbor: [ {} : {} : {} ] {}",
                    reports::tip("Kills at last 10 minutes", format!("{:0>3}", History::system_count(&neighbor.neighbor_id, &10, ctx))),
                    reports::tip("Kills at last 60 minutes", format!("{:0>3}", History::system_count(&neighbor.neighbor_id, &60, ctx))),
                    reports::tip("Kills at last 6 hours", format!("{:0>3}", History::system_count(&neighbor.neighbor_id, &360, ctx))),
                    ctx.get_place_desc("system", id, name)
                ));
            }
        }
    }

    fn get_system_href(id: &i32, ctx: &Context) -> String {
        if let Some(system) = Self::load(id, ctx) {
            ctx.get_api_link("system", system.get_name("system"))
        } else {
            String::from("...Unknown...")
        }
    }

    // todo create api type for route
    fn get_route(departure: &i32, destination: &i32, flag: &str) -> Option<Vec<i32>> {
        use crate::api;
        let response = if flag.is_empty(){
            api::gw::eve_api(&format!("route/{}/{}", departure, destination)).unwrap_or_default()
        } else {
            api::gw::eve_api_ex(
                &format!("route/{}/{}", departure, destination),
                &format!("flag={}", flag)
            ).unwrap_or_default()
        };
        serde_json::from_str(&response).ok()
    }

    pub fn route(departure: i32, destination: i32, ctx: &Context) -> String {
        let mut path = String::new();
        if let Some(route) = Self::get_route(&departure, &destination, "") {
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


    fn kills_color(count: i32) -> String {
        if count > 10 {"#FF0000"}
        else if count > 5 {"#FF5050"}
        else if count > 1 {"#FFCCCC"}
        else {"#FFFFFF"}
        .to_string()
    }

    pub fn route_named(safety: String, departure: String, destination: String, ctx: &Context) -> String {
        let category = "solar_system";
        let mut output = String::new();
        let src = reports::find_id(category, &departure, ctx).unwrap_or(0);
        if 0 == src {
            reports::div(&mut output, format!("departure {} was not found in category {}", category, departure));
        }
        let dst = reports::find_id(category, &destination, ctx).unwrap_or(0);
        if 0 == src {
            reports::div(&mut output, format!("destination {} was not found in category {}", category, destination));
        }
        let route = if vec!["insecure","unsafe", "u"].contains(&safety.as_ref()) {
            Self::get_route(&src, &dst, "insecure")
        } else if vec!["safe", "secure", "s"].contains(&safety.as_ref()) {
            Self::get_route(&src, &dst, "secure")
        } else {
            Self::get_route(&src, &dst, "")
        };

        if let Some(ids) = route {
            let mut jumps = 0;
            let table_style = "border-collapse: collapse;";
            let head_style = "border: 1px solid black; padding: 2px 5px; text-align: center;";

            reports::table_start(&mut output, "", table_style, "");
            reports::caption(&mut output, "Route");
            reports::table_row_start(&mut output, head_style);
            reports::table_cell_head(&mut output, "Jumps offset", head_style, "Jumps");
            reports::table_cell_head(&mut output, "Region Name", head_style, "Region");
            reports::table_cell_head(&mut output, "Constellation Name", head_style, "Constellation");
            reports::table_cell_head(&mut output, "System Name", head_style, "System");
            reports::table_cell_head(&mut output, "System Security Status", head_style, "SSS");
            reports::table_cell_head(&mut output, "10 minutes history", head_style, "10m");
            reports::table_cell_head(&mut output, "1 hour history", head_style, "1h");
            reports::table_cell_head(&mut output, "6 hours history", head_style, "6h");
            reports::table_cell_head(&mut output, "24 hours history", head_style, "24h");
            reports::table_row_end(&mut output);
            for id in &ids {
                if let Some(system) = System::load(id, ctx) {
                    use services::{Message, Api};
                    use reports::history::History;

                    let color = Self::kills_color(History::system_count(&id, &10, ctx));
                    let text_style = &format!("border: 1px solid black; padding: 2px 5px; background-color: {};", color);
                    let num_style = &format!("border: 1px solid black; padding: 2px 5px; text-align: right;  background-color: {};", color);

                    if system.get_name("system").is_empty() {
                        ctx.resolver.push(Message::Receive(Api::Object(system.get_id("system"))));
                    }
                    if system.get_name("constellation").is_empty() {
                        ctx.resolver.push(Message::Receive(Api::Object(system.get_id("constellation"))));
                    }
                    if system.get_name("region").is_empty() {
                        ctx.resolver.push(Message::Receive(Api::Object(system.get_id("region"))));
                    }

                    reports::table_row_start(&mut output, text_style);
                    reports::table_cell(&mut output, "Jumps offset", num_style, format!("{}", jumps));
                    reports::table_cell(&mut output, "Region Name", text_style,         ctx.get_api_href("region", system.get_id("region"), system.get_name("region")));
                    reports::table_cell(&mut output, "Constellation Name", text_style,  ctx.get_api_href("constellation", system.get_id("constellation"), system.get_name("constellation")));
                    reports::table_cell(&mut output, "System Name", text_style,         ctx.get_api_href("system", *id, system.get_name("system")));
                    reports::table_cell(&mut output, "System Security Status", num_style, format!("{:.2}", system.security_status));
                    reports::table_cell(&mut output, "10 minutes history", num_style,  History::system_count(&id, &10, ctx).separated_string());
                    reports::table_cell(&mut output, "1 hour history", num_style,      History::system_count(&id, &60, ctx).separated_string());
                    reports::table_cell(&mut output, "6 hours history", num_style,     History::system_count(&id, &360, ctx).separated_string());
                    reports::table_cell(&mut output, "24 hours history", num_style,    History::system_count(&id, &1440, ctx).separated_string());
                    reports::table_row_end(&mut output);
                    jumps = jumps + 1;
                }
            }
            reports::table_end(&mut output);
        }

        return output;
    }

    fn report_observatory_path(output: &mut dyn Write, id: &i32, ctx: &Context) {
        use services::{Category, Report};
        use std::collections::HashSet;
        let arrow = format!("&nbsp;=&gt;&nbsp;");
        let mut covered = HashSet::new();
        match reports::load(Category::ObservatoryPath(*id), &ctx) {
            Report::ObservatoryPath(paths) => {
                let mut rest = 5;
                for path in &paths {
                    if path.s1_jo {
                        if covered.insert(&path.s1_id) {
                            reports::div(output, format!("{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx)
                            ));
                            rest = rest - 1;
                        }
                    } else if path.s2_jo {
                        if covered.insert(&path.s2_id) {
                            reports::div(output, format!("{}{}{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx),
                                arrow, Self::get_system_href(&path.s2_id, ctx),
                            ));
                            rest = rest - 1;
                        }
                    } else if path.s3_jo {
                        if covered.insert(&path.s3_id) {
                            reports::div(output, format!("{}{}{}{}{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx),
                                arrow, Self::get_system_href(&path.s2_id, ctx),
                                arrow, Self::get_system_href(&path.s3_id, ctx),
                            ));
                            rest = rest - 1;
                        }
                    } else if path.s4_jo {
                        if covered.insert(&path.s4_id) {
                            reports::div(output, format!("{}{}{}{}{}{}{}{}",
                                arrow, Self::get_system_href(&path.s1_id, ctx),
                                arrow, Self::get_system_href(&path.s2_id, ctx),
                                arrow, Self::get_system_href(&path.s3_id, ctx),
                                arrow, Self::get_system_href(&path.s4_id, ctx),
                            ));
                            rest = rest - 1;
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
                            rest = rest - 1;
                        }
                    }

                    if 0 == rest {
                        break;
                    }
                }
            },
            report => {
                reports::div(output, format!("Unexpected Report: {:?}", report));
            }
        }
    }

    fn observatory_report(output: &mut dyn Write, system: &models::system::SystemNamed, ctx: &Context) {
        if system.has_observatory() {
            let name = system.get_name("system");
            let observatory = reports::span("", "color: green;", "Jovian Observatory");
            let content = reports::span("", "", format!("<span>There are {} in the {} system.</span>", observatory, name));
            reports::div(output, content);
        }

        reports::div(output, format!("Nearest system with Jovian Observatory:"));
        Self::report_observatory_path(output, &system.system_id, ctx);
    }

    pub fn observatory_add(id: &i32, ctx: &Context) -> String {
        use services::{Message, Model};
        ctx.database.push(Message::Save(Model::Observatory(*id)));
        String::from("Done")
    }

    pub fn observatory_del(id: &i32, ctx: &Context) -> String {
        use services::{Message, Model};
        ctx.database.push(Message::Delete(Model::Observatory(*id)));
        String::from("Done")
    }

}

