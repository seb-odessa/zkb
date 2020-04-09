use crate::api;
use crate::models;
use crate::services;
use crate::services::Context;
use crate::reports;

use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct Constellation;
impl reports::ReportableEx for Constellation {

    fn get_category() -> String {
        String::from("constellation")
    }

    fn report_by_id(id: &i32, ctx: &Context, report_type: reports::ReportType) -> String {
        let mut output = String::new();
        if let Some(constellation) = Self::load(id, &ctx) {
            Self::write(&mut output, &constellation, &ctx);
            if report_type == reports::ReportType::Full {
                reports::lazy(&mut output, format!("api/region_brief/{}", constellation.region_id), &ctx);
                Self::neighbors(&mut output, id, &ctx);
                Self::systems(&mut output, id, &ctx);
                reports::Region::constellations(&mut output, &constellation.region_id, &ctx);
                reports::map(&mut output, id, 0, "constellation", &ctx);                
                reports::lazy(&mut output, format!("history/constellation/{}/{}", id, 60), &ctx);
                reports::lazy(&mut output, format!("stat/constellation/{}", id), &ctx);
                reports::div(&mut output, "");

            }
        }
        return output;
    }
}
impl Constellation {

    fn format(constellation: &models::constellation::ConstellationNamed, ctx: &Context) -> String {
        let id = constellation.constellation_id;
        let name = constellation.get_name("constellation");
        let region_name = constellation.get_name("region");
        format!(
            r#"<span id="{id}" data-name="{name}">Constellation: {api}  [{zkb}] [{map}]</span>"#,
            id = id,
            name = &name,
            api = ctx.get_api_link("constellation", &name),
            zkb = ctx.get_zkb_href("constellation", id, "zkb"),
            map = ctx.get_dotlan_href(region_name, &name, "dotlan")
        )
    }

    pub fn write(output: &mut dyn Write, constellation: &models::constellation::ConstellationNamed, ctx: &Context) {
        reports::div(output, Self::format(constellation, ctx));
    }

    fn neighbors(output: &mut dyn Write, id: &i32, ctx: &Context) {
        use services::{Category, Report, Area, Message, Api};
        if let Report::ConstellationNeighbors(neighbors) = reports::load(Category::Neighbors(Area::Constellation(*id)), &ctx) {
            use reports::history::History;
            for neighbor in &neighbors {
                let id = neighbor.neighbor_id;
                let name = neighbor.get_name("neighbor");
                if name.is_empty() {
                    ctx.resolver.push(Message::Receive(Api::Object(id)));
                }
                reports::div(output, format!("neighbor: [ {} : {} : {} ] {}",
                    reports::tip("Kills at last 10 minutes", format!("{:0>3}", History::constellation_count(&id, &10, ctx))),
                    reports::tip("Kills at last 60 minutes", format!("{:0>3}", History::constellation_count(&id, &60, ctx))),
                    reports::tip("Kills at last 6 hours", format!("{:0>3}", History::constellation_count(&id, &360, ctx))),
                    ctx.get_api_href("constellation", id, name)
                ));
            }
        }
    }

    pub fn load(id: &i32, ctx: &Context) -> Option<models::constellation::ConstellationNamed> {
        use services::{Category, Report};
        match reports::load(Category::Constellation(*id), &ctx) {
            Report::Constellation(constellation) => return Some(constellation),
            Report::NotFoundId(id) => warn!("{} was not found", id),
            report => warn!("Unexpected report {:?}", report)
        }
        return None;
    }

    pub fn get_systems(constellation_id: &i32, ctx: &Context) -> Option<Vec<models::system::SystemNamed>> {
        use services::{Category, Report, Area};
        let area = Area::Constellation(*constellation_id);
        let any = models::system::SystemFilter::Any;
        match reports::load(Category::Systems((area, any)), &ctx) {
            Report::Systems(systems) => return Some(systems),
            Report::NotFoundId(id) => warn!("{} was not found", id),
            report => warn!("Unexpected report {:?}", report)
        }
        return None;
    }

    pub fn systems(output: &mut dyn Write, constellation_id: &i32, ctx: &Context) {
        use std::collections::BTreeSet;
        let mut set = BTreeSet::new();
        if let Some(systems) = Self::get_systems(constellation_id, ctx) {
            for system in &systems {
                let url = reports::span("Solar System", "", ctx.get_api_link("system", &system.get_name("system")));
                set.insert(url);
            }
            let mut list = String::new();
            for url in &set {
                list += url;
                list += " ";
            }
            reports::div(output, format!("Systems in constellation: {}", list));    
        }
    }

    pub fn stat(id: &i32, ctx: &Context) -> String {
        use api::stats::Stats;
        use api::stats::Entity;
        use api::stats::TopList;
        use std::collections::HashSet;

        let mut output = String::new();
        if let Some(stats) = Stats::new(Entity::Constellation(*id)) {
            //character, corporation, alliance, shipType, solarSystem, location
            let allowed: HashSet<String> = vec!["character", "corporation", "alliance", "shipType", "solarSystem", "location"].into_iter().map(|s| String::from(s)).collect();
            TopList::write(&mut output, &stats.top_lists, allowed, ctx);
        }
        return output;
    }
}
