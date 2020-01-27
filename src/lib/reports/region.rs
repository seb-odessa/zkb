use crate::models;
use crate::services;
use crate::services::Context;
use crate::reports;

use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct Region;
impl reports::ReportableEx for Region {

    fn get_category() -> String {
        String::from("region")
    }

    fn report_by_id(id: &i32, ctx: &Context, report_type: reports::ReportType) -> String {
        use services::{Category, Report};
        let mut output = String::new();
        if let Report::Region(region) = reports::load(Category::Region(*id), &ctx) {
            Self::write(&mut output, &region, ctx);
            if report_type == reports::ReportType::Full {
                Self::neighbors(&mut output, id, ctx);
                reports::constellations(&mut output, &region.region_id, &ctx);
                reports::lazy(&mut output, format!("history/region/{}/{}", id, 60), &ctx);
            }
        }
        return output;
    }
}
impl Region {

    fn format(region: &models::region::RegionNamed, ctx: &Context) -> String {
        let id = region.get_id();
        let name = region.get_name();
        format!(
            r#"<span id="{id}" data-name="{name}">Region: {api}  [{zkb}] [{map}]</span>"#,
            id = id,
            name = name.clone(),
            api = ctx.get_api_link("region", &name),
            zkb = ctx.get_zkb_href("region", id, "zkb"),
            map = ctx.get_dotlan_href(&name, "", "dotlan")
        )
    }

    pub fn write(output: &mut dyn Write, region: &models::region::RegionNamed, ctx: &Context) {
        reports::div(output, Self::format(region, ctx));
    }

    fn neighbors(output: &mut dyn Write, id: &i32, ctx: &Context) {
        use reports::history::History;
        use services::{Category, Report, Area, Message, Api};

        if let Report::RegionNeighbors(neighbors) = reports::load(Category::Neighbors(Area::Region(*id)), &ctx) {
            for neighbor in &neighbors {
                let id = neighbor.get_id("neighbor");
                let name = neighbor.get_name("neighbor");
                if name.is_empty() {
                    ctx.resolver.push(Message::Receive(Api::Object(id)));
                }
                reports::div(output, format!("neighbor: [ {} : {} : {} ] {} ({})",
                    reports::tip("Kills at last 10 minutes", format!("{:0>3}", History::region_count(&id, &10, ctx))),
                    reports::tip("Kills at last 60 minutes", format!("{:0>3}", History::region_count(&id, &60, ctx))),
                    reports::tip("Kills at last 6 hours", format!("{:0>3}", History::region_count(&id, &360, ctx))),
                    ctx.get_api_link("region", name),
                    ctx.get_zkb_href("region", id, "zkb"),
                ));
            }
        }
    }

}
