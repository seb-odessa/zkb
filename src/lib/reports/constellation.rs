use crate::api;
use crate::services::Context;
use crate::reports::*;
use chrono::Utc;

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

    fn neighbors(output: &mut dyn Write, constellation: &api::constellation::Constellation, ctx: &Context) {
        for system_id in &constellation.systems {
            if let Some(system) = api::system::System::new(system_id)
            {
                if system.constellation_id != constellation.constellation_id {
                    lazy(output, format!("api/constellation_brief/{}", system.constellation_id), &ctx);
                }
            } else {
                div(output, format!("Can't query System({}) from CCP API", system_id));
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
                lazy(&mut output, format!("api/region/{}", object.region_id), &ctx);
                Self::neighbors(&mut output, &object, &ctx);
                let now = Utc::now().naive_utc().time().format("%H:%M:%S").to_string();
                div(&mut output, format!("Kill history 60 minutes since {} ", &now));
                lazy(&mut output, format!("history/constellation/{}/{}", id, 60), &ctx);
            }
        } else {
            div(&mut output, format!("Can't query Constellation({}) from CCP API", id));
        }
        return output;
    }
}
