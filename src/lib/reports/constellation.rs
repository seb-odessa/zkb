use crate::models;
use crate::services::{Context, Area, Category, Report};
use crate::reports::*;
use crate::reports;

#[derive(Debug, PartialEq)]
pub struct Constellation;
impl reports::ReportableEx for Constellation {

    fn get_category() -> String {
        String::from("constellation")
    }

    fn report_by_id(id: &i32, ctx: &Context, report_type: reports::ReportType) -> String {
        let mut output = String::new();
        if let Report::Constellation(object) = reports::load(Category::Constellation(*id), &ctx) {
            Self::write(&mut output, &object, &ctx);
            if report_type == ReportType::Full {
                reports::lazy(&mut output, format!("api/region_brief/{}", object.region_id), &ctx);
                Self::neighbors(&mut output, &object.constellation_id, &ctx);
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
            r#"<span id="{id}" data-name="{name}">Constellation: {api}  [{map}]</span>"#,
            id = id,
            name = &name,
            api = ctx.get_api_href("constellation", id, &name),
            map = ctx.get_dotlan_href(region_name, &name, "dotlan")
        )
    }

    pub fn write(output: &mut dyn Write, constellation: &models::constellation::ConstellationNamed, ctx: &Context) {
        reports::div(output, Self::format(constellation, ctx));
    }

    fn neighbors(output: &mut dyn Write, id: &i32, ctx: &Context) {
        if let Report::ConstellationNeighbors(neighbors) = reports::load(Category::Neighbors(Area::Constellation(*id)), &ctx) {
            for neighbor in &neighbors {
                let id = neighbor.neighbor_id;
                let name = neighbor.get_name("neighbor");
                div(output, format!("neighbor: [ {} : {} : {} ] {}",
                    tip("Kills at last 10 minutes", format!("{:0>3}", history::History::constellation_count(&neighbor.neighbor_id, &10, ctx))),
                    tip("Kills at last 60 minutes", format!("{:0>3}", history::History::constellation_count(&neighbor.neighbor_id, &60, ctx))),
                    tip("Kills at last 6 hours", format!("{:0>3}", history::History::constellation_count(&neighbor.neighbor_id, &360, ctx))),
                    ctx.get_api_href("constellation", id, name)
                ));
            }
        }
    }

}
