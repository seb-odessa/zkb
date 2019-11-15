use crate::api;
use crate::services::Context;
use crate::reports::*;

#[derive(Debug, PartialEq)]
pub struct Region;
impl Region {
    pub fn report(arg: &String, ctx: &Context) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::report_by_id(id, ctx)
        } else if let Some(ref id) = find_id("region", arg, ctx) {
            Self::report_by_id(id, ctx)
        } else {
            format!("Region {} was not found", arg)
        }
    }

    fn report_by_id(id: &i32, _ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(object) = api::region::Region::new(id) {
            div(&mut output, "Region", &href(object.zkb(), object.name.clone()));
        } else {
            div(&mut output, "Region", &format!("{} not found", id));
        }
        return output;
    }
}
