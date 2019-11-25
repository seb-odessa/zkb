use crate::api;
use crate::services::{Context, Area, Category, Report};
use crate::reports::*;
use crate::reports;

#[derive(Debug, PartialEq)]
pub struct Constellation;
impl Constellation {

    pub fn write(output: &mut dyn Write, constellation: &api::constellation::Constellation, root: &String) {
        let url = format!("{}/api/constellation/{}", root, constellation.constellation_id);
        std::fmt::write(
            output,
            format_args!(
                r#"<div id="{id}" data-name="{name}">Constellation: {url}</div>"#,
                id = constellation.constellation_id,
                url = href(&url, &constellation.name),
                name = constellation.name
            )
        ).expect(FAIL);
    }

    fn neighbors(output: &mut dyn Write, id: &i32, ctx: &Context) {
        let root = root(&ctx);
        if let Report::ConstellationNeighbors(neighbors) = reports::load(Category::Neighbors(Area::Constellation(*id)), &ctx) {
            for neighbor in &neighbors {
                let url = format!("{}/api/constellation/{}", root, neighbor.neighbor_id);
                let name = neighbor.get_name("neighbor");
                div(output, format!("neighbor: [ {} : {} : {} ] {}",
                    tip("Kills at last 10 minutes", format!("{:0>3}", history::History::constellation_count(&neighbor.neighbor_id, &10, ctx))),
                    tip("Kills at last 60 minutes", format!("{:0>3}", history::History::constellation_count(&neighbor.neighbor_id, &60, ctx))),
                    tip("Kills at last 6 hours", format!("{:0>3}", history::History::constellation_count(&neighbor.neighbor_id, &360, ctx))),
                    href(&url, &name),
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
        } else if let Some(ref id) = find_id("constellation", arg, ctx) {
            Self::report_by_id(id, ctx, report_type)
        } else {
            format!("<div>Constellation {} was not found</div>", arg)
        }
    }

    fn report_by_id(id: &i32, ctx: &Context, report_type: ReportType) -> String {
        let mut output = String::new();
        if let Some(object) = api::constellation::Constellation::new(id) {
            Self::write(&mut output, &object, &root(ctx));
            if report_type == ReportType::Full {
                lazy(&mut output, format!("api/region_brief/{}", object.region_id), &ctx);
                Self::neighbors(&mut output, &object.constellation_id, &ctx);
                lazy(&mut output, format!("history/constellation/{}/{}", id, 60), &ctx);
            }
        } else {
            div(&mut output, format!("Can't query Constellation({}) from CCP API", id));
        }
        return output;
    }
}
