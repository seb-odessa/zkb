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
        use services::{Category, Report};
        let mut output = String::new();
        if let Report::Constellation(constellation) = reports::load(Category::Constellation(*id), &ctx) {
            Self::write(&mut output, &constellation, &ctx);
            if report_type == reports::ReportType::Full {
                reports::lazy(&mut output, format!("api/region_brief/{}", constellation.region_id), &ctx);
                Self::neighbors(&mut output, &constellation.constellation_id, &ctx);
                reports::systems(&mut output, &constellation.constellation_id, &ctx);
                reports::constellations(&mut output, &constellation.region_id, &ctx);
                reports::lazy(&mut output, format!("history/constellation/{}/{}", id, 60), &ctx);
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

}
