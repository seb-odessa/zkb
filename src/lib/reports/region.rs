use crate::api;
use crate::services::{Context, Area, Category, Report};
use crate::reports::*;
use crate::reports;

#[derive(Debug, PartialEq)]
pub struct Region;
impl Region {

    fn write(output: &mut dyn Write, region: &api::region::Region, root: &String) {
        let url = format!("{}/api/region/{}", root, region.region_id);
        std::fmt::write(
            output,
            format_args!(
                r#"<div id="{id}" data-name="{name}">Region: {url}</div>"#,
                id = region.region_id,
                url = href(&url, &region.name),
                name = region.name
            )
        ).expect(FAIL);
    }

    fn neighbors(output: &mut dyn Write, id: &i32, ctx: &Context) {
        let root = root(&ctx);
        let msg_id = crate::create_id().to_simple();
        let empty = String::new();
        ctx.database.push(Message::Find((msg_id, Category::Neighbors(Area::Region(*id)))));
        if let Report::RegionNeighbors(neighbors) = reports::wait_for(msg_id, &ctx) {
            for neighbor in &neighbors {
                let url = format!("{}/api/region/{}", root, neighbor.neighbor_id);
                let name = neighbor.neighbor_name.as_ref().unwrap_or(&empty);
                div(output, format!("neighbor: [ {} : {} : {} ] {}",
                    tip("Kills at last 10 minutes", format!("{:0>3}", neighbor.ten_minutes)),
                    tip("Kills at last 60 minutes", format!("{:0>3}", neighbor.one_hour)),
                    tip("Kills at last 6 hours", format!("{:0>3}", neighbor.six_hours)),
                    href(&url, name),
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
        } else if let Some(ref id) = find_id("region", arg, ctx) {
            Self::report_by_id(id, ctx, report_type)
        } else {
            format!("<div>Region {} was not found</div>", arg)
        }
    }

    fn report_by_id(id: &i32, ctx: &Context, report_type: ReportType) -> String {
        let mut output = String::new();
        if let Some(object) = api::region::Region::new(id) {
            Self::write(&mut output, &object, &root(ctx));
            if report_type == ReportType::Full {
                Self::neighbors(&mut output, &object.region_id, ctx);
                lazy(&mut output, format!("history/region/{}/{}", id, 60), &ctx);
            }
        } else {
            div(&mut output, format!("Can't query Region({}) from CCP API", id));
        }
        return output;
    }
}
