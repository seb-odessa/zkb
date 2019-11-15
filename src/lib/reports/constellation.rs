use crate::api;
use crate::services::Context;
use crate::reports::*;

#[derive(Debug, PartialEq)]
pub struct Constellation;
impl Constellation {

    pub fn report(arg: &String, ctx: &Context) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::report_by_id(id, ctx)
        } else if let Some(ref id) = find_id("constellation", arg, ctx) {
            Self::report_by_id(id, ctx)
        } else {
            format!("<div>Constellation {} was not found</div>", arg)
        }
    }

    pub fn report_by_id(id: &i32, _: &Context) -> String {
        let mut output = String::new();
        if let Some(object) = api::constellation::Constellation::new(id) {
            div(&mut output, format!("Constellation {}", object.name));
        } else {
            div(&mut output, format!("Can't query Constellation({}) from CCP API", id));
        }
        return output;
    }
}
